#![no_std]
#![no_main]

pub mod keycode;
pub mod config;
pub mod hardware;
use config::Config;
use cortex_m::delay::Delay;
use cortex_m::prelude::{_embedded_hal_watchdog_Watchdog, _embedded_hal_watchdog_WatchdogEnable};
use embedded_hal::digital::v2::{InputPin, OutputPin};
use fugit::ExtU32;
use hardware::Encoder;
use keycode::Keycodes;
use panic_halt as _;
use rp_pico::hal::{self, Clock};
use rp_pico::hal::{gpio::DynPin, pac::interrupt};
use usb_device::{
    class_prelude::UsbBusAllocator,
    prelude::{UsbDevice, UsbDeviceBuilder, UsbVidPid},
};

// USB Human Interface Device (HID) Class support
// use usbd_hid::descriptor::generator_prelude::*;
use usbd_hid::descriptor::KeyboardReport;
use usbd_hid::descriptor::SerializedDescriptor;
use usbd_hid::hid_class;

/// The USB Device Driver (shared with the interrupt).
static mut USB_DEVICE: Option<UsbDevice<hal::usb::UsbBus>> = None;

/// The USB Bus Driver (shared with the interrupt).
static mut USB_BUS: Option<UsbBusAllocator<hal::usb::UsbBus>> = None;

/// The USB Human Interface Device Driver (shared with the interrupt).
static mut USB_HID: Option<hid_class::HIDClass<hal::usb::UsbBus>> = None;

// maybe remove the watchdog in the future
pub fn init() -> (rp_pico::Pins, hal::Watchdog, Delay) {
    // setup peripherals
    let mut pac = hal::pac::Peripherals::take().unwrap();

    // setup watchdog
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);
    watchdog.start(1.secs());

    // setup serial input/output
    let sio = hal::Sio::new(pac.SIO);

    // setup clock at 125Mhz
    let clocks = hal::clocks::init_clocks_and_plls(
        rp_pico::XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let pins = rp_pico::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // Set up the USB Communications Class Device driver
    let usb_bus = UsbBusAllocator::new(hal::usb::UsbBus::new(
        pac.USBCTRL_REGS,
        pac.USBCTRL_DPRAM,
        clocks.usb_clock,
        true,
        &mut pac.RESETS,
    ));
    unsafe {
        // Note (safety): This is safe as interrupts haven't been started yet
        USB_BUS = Some(usb_bus);
    }

    let bus_ref = unsafe { USB_BUS.as_ref().unwrap() };

    // Setup usb hid class
    let usb_hid = hid_class::HIDClass::new_ep_in_with_settings(
        bus_ref,
        KeyboardReport::desc(),
        60,
        hid_class::HidClassSettings {
            subclass: hid_class::HidSubClass::NoSubClass,
            config: hid_class::ProtocolModeConfig::ForceReport,
            locale: hid_class::HidCountryCode::US,
            protocol: hid_class::HidProtocol::Keyboard,
        },
    );

    unsafe {
        USB_HID = Some(usb_hid);
    }

    let usb_dev = UsbDeviceBuilder::new(bus_ref, UsbVidPid(0x16c0, 0x27da))
        .manufacturer("Cole corp")
        .product("One Key")
        .serial_number("1")
        .device_class(0)
        .build();

    unsafe {
        // Note (safety): This is safe as interrupts haven't been started yet
        USB_DEVICE = Some(usb_dev);
    }

    unsafe {
        hal::pac::NVIC::unmask(hal::pac::interrupt::USBCTRL_IRQ);
    };

    let core = hal::pac::CorePeripherals::take().unwrap();
    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());
    (pins, watchdog, delay)
}

pub fn matrix_scaning<const COLS: usize, const ROWS: usize, const LAYERS: usize>(
    mut cols: [DynPin; COLS],
    mut rows: [DynPin; ROWS],
    keys: [[[Keycodes; COLS]; ROWS]; LAYERS],
    mut encoder: Option<Encoder>,
    // mut led: DynPin,
    config: Config,
    mut watchdog: hal::Watchdog,
    mut delay: Delay,
) -> ! {
    rows.iter_mut().for_each(|pin| {
        pin.into_pull_down_input();
    });
    cols.iter_mut().for_each(|pin| {
        pin.into_readable_output();
    });

    let mut last_state_a: bool = false;
    let mut state_a: bool = false;
    if config.encoder {
        encoder.as_mut().expect("If encoder is true, must supply a pin for channel_a").channel_a.into_pull_up_input();
        // If the excpect did not happend the first time it must mean that encoder was supplyed so
        // it's safe to unwrap it
        encoder.as_mut().unwrap().channel_b.into_pull_up_input();
        last_state_a = encoder.as_mut().unwrap().channel_a.is_high().unwrap();
    }

    let mut layer = 0;
    let mut index: usize;
    let mut report: KeyboardReport;
    loop {
        // feed watchdog
        watchdog.feed();
        // moving this outside of the loop can make this more effiecnt by checking if the key is pressed and not over writing it
        index = 0;

        report = KeyboardReport {
            modifier: 0x00,
            reserved: 0x00,
            leds: 0x00,
            keycodes: [0x00; 6],
        };

        if config.encoder {
            state_a = encoder.as_mut().unwrap().channel_a.is_high().unwrap();
            if last_state_a != state_a && state_a {
                if state_a != encoder.as_mut().unwrap().channel_b.is_high().unwrap() {
                    report.keycodes[0] = 0x80;
                    delay.delay_ms(50);
                    push_keyboard_inputs(report).ok().unwrap_or(0);
                    report.keycodes[0] = 0x00;
                } else {
                    delay.delay_ms(50);
                    report.keycodes[0] = 0x81;
                    push_keyboard_inputs(report).ok().unwrap_or(0);
                    report.keycodes[0] = 0x00;
                }
            }
        }

        for (col, pin) in cols.iter_mut().enumerate() {
            pin.set_high().unwrap();
            for (row, pin) in rows.iter_mut().enumerate() {
                if index <= 6 && pin.is_high().unwrap() {
                    report.keycodes[index] = keys[layer][row][col] as u8;
                    index += 1;
                }
            }
            pin.set_low().unwrap();
        }

        push_keyboard_inputs(report).ok().unwrap_or(0);
        if config.encoder {
            last_state_a = state_a;
        }
    }
}

fn push_keyboard_inputs(report: KeyboardReport) -> Result<usize, usb_device::UsbError> {
    critical_section::with(|_| unsafe {
        // Now interrupts are disabled, grab the global variable and, if
        // available, send it a HID report
        USB_HID.as_mut().map(|hid| hid.push_input(&report))
    })
    .unwrap()
}

#[interrupt]
unsafe fn USBCTRL_IRQ() {
    let usb_hid = USB_HID.as_mut().unwrap();
    let usb_device = USB_DEVICE.as_mut().unwrap();
    usb_device.poll(&mut [usb_hid]);
}