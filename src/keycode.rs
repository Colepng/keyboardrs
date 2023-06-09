use self::Keycodes::*;
#[derive(Copy, Clone)]
#[allow(non_camel_case_types, unused)]
#[repr(u8)]
pub enum Keycodes {
    KC_NO = 0x00,

    KC_A = 0x04,
    KC_B = 0x05,
    KC_C = 0x06,
    KC_D = 0x07,
    KC_E = 0x08,
    KC_F = 0x09,
    KC_G = 0x0a,
    KC_H = 0x0b,
    KC_I = 0x0c,
    KC_J = 0x0d,
    KC_K = 0x0e,
    KC_L = 0x0f,
    KC_M = 0x10,
    KC_N = 0x11,
    KC_O = 0x12,
    KC_P = 0x13,
    KC_Q = 0x14,
    KC_R = 0x15,
    KC_S = 0x16,
    KC_T = 0x17,
    KC_U = 0x18,
    KC_V = 0x19,
    KC_W = 0x1a,
    KC_X = 0x1b,
    KC_Y = 0x1c,
    KC_Z = 0x1d,

    KC_1 = 0x1e,
    KC_2 = 0x1f,
    KC_3 = 0x20,
    KC_4 = 0x21,
    KC_5 = 0x22,
    KC_6 = 0x23,
    KC_7 = 0x24,
    KC_8 = 0x25,
    KC_9 = 0x26,
    KC_0 = 0x27,

    KC_ENTER = 0x28,
    KC_ESCAPE = 0x29,
    KC_BACKSPACE = 0x2A,
    KC_TAB = 0x2B,
    KC_SPACE = 0x2C,
    KC_MINUS = 0x2D,
    KC_EQUAL = 0x2E,
    KC_LEFT_BRACKET = 0x2F,
    KC_RIGHT_BRACKET = 0x30,
    KC_BACKSLASH = 0x31,
    KC_NONUS_HASH = 0x32,
    KC_SEMICOLON = 0x33,
    KC_QUOTE = 0x34,
    KC_GRAVE = 0x35,
    KC_COMMA = 0x36,
    KC_DOT = 0x37,
    KC_SLASH = 0x38,
    KC_CAPS_LOCK = 0x39,

    KC_DELETE_FORWARD = 0x4C,

    KC_RIGHT_ARROW = 0x4F,
    KC_LEFT_ARROW = 0x50,
    KC_DOWN_ARROW = 0x51,
    KC_UP_ARROW = 0x52,

    KC_MUTE = 0x7f,
    KC_VOLUP = 0x80,
    KC_VOLDOWN = 0x81,

    KC_LEFT_CTRL = 0xE0,
    KC_LEFT_SHIFT = 0xE1,
    KC_LEFT_ALT = 0xE2,
    KC_LEFT_GUI = 0xE3,
    KC_RIGHT_CTRL = 0xE4,
    KC_RIGHT_SHIFT = 0xE5,
    KC_RIGHT_ALT = 0xE6,
    KC_RIGHT_GUI = 0xE7,
    KC_APP = 0x65,

    KC_MNEXT = 0xB5,
    KC_MPREV = 0xB6,
    KC_MSTOP = 0xB7,
    KC_MPLAY_PAUSE = 0xCD,

    KC_LAYER(usize),
    KC_MO(usize),
    KC_NO_KEY,
}

impl Keycodes {
    pub fn is_consumer(&self) -> bool {
        match self {
            Self::KC_MPREV => true,
            Self::KC_MNEXT => true,
            Self::KC_MSTOP => true,
            Self::KC_MPLAY_PAUSE => true,
            _ => false,
        }
    }
}

// impl From<u8> for Keycodes {
//     fn from(value: u8) -> Self {
//         match value {
//         }
//     }
// }

impl TryInto<u8> for Keycodes {
    type Error = &'static str;
    fn try_into(self) -> Result<u8, Self::Error> {
        match self {
            KC_NO => Ok(0x00),

            KC_A => Ok(0x04),
            KC_B => Ok(0x05),
            KC_C => Ok(0x06),
            KC_D => Ok(0x07),
            KC_E => Ok(0x08),
            KC_F => Ok(0x09),
            KC_G => Ok(0x0a),
            KC_H => Ok(0x0b),
            KC_I => Ok(0x0c),
            KC_J => Ok(0x0d),
            KC_K => Ok(0x0e),
            KC_L => Ok(0x0f),
            KC_M => Ok(0x10),
            KC_N => Ok(0x11),
            KC_O => Ok(0x12),
            KC_P => Ok(0x13),
            KC_Q => Ok(0x14),
            KC_R => Ok(0x15),
            KC_S => Ok(0x16),
            KC_T => Ok(0x17),
            KC_U => Ok(0x18),
            KC_V => Ok(0x19),
            KC_W => Ok(0x1a),
            KC_X => Ok(0x1b),
            KC_Y => Ok(0x1c),
            KC_Z => Ok(0x1d),

            KC_1 => Ok(0x1e),
            KC_2 => Ok(0x1f),
            KC_3 => Ok(0x20),
            KC_4 => Ok(0x21),
            KC_5 => Ok(0x22),
            KC_6 => Ok(0x23),
            KC_7 => Ok(0x24),
            KC_8 => Ok(0x25),
            KC_9 => Ok(0x26),
            KC_0 => Ok(0x27),

            KC_ENTER => Ok(0x28),
            KC_ESCAPE => Ok(0x29),
            KC_BACKSPACE => Ok(0x2A),
            KC_TAB => Ok(0x2B),
            KC_SPACE => Ok(0x2C),
            KC_MINUS => Ok(0x2D),
            KC_EQUAL => Ok(0x2E),
            KC_LEFT_BRACKET => Ok(0x2F),
            KC_RIGHT_BRACKET => Ok(0x30),
            KC_BACKSLASH => Ok(0x31),
            KC_NONUS_HASH => Ok(0x32),
            KC_SEMICOLON => Ok(0x33),
            KC_QUOTE => Ok(0x34),
            KC_GRAVE => Ok(0x35),
            KC_COMMA => Ok(0x36),
            KC_DOT => Ok(0x37),
            KC_SLASH => Ok(0x38),
            KC_CAPS_LOCK => Ok(0x39),

            KC_DELETE_FORWARD => Ok(0x4C),

            KC_RIGHT_ARROW => Ok(0x4F),
            KC_LEFT_ARROW => Ok(0x50),
            KC_DOWN_ARROW => Ok(0x51),
            KC_UP_ARROW => Ok(0x52),

            KC_MUTE => Ok(0x7f),
            KC_VOLUP => Ok(0x80),
            KC_VOLDOWN => Ok(0x81),

            KC_LEFT_CTRL => Ok(0xE0),
            KC_LEFT_SHIFT => Ok(0xE1),
            KC_LEFT_ALT => Ok(0xE2),
            KC_LEFT_GUI => Ok(0xE3),
            KC_RIGHT_CTRL => Ok(0xE4),
            KC_RIGHT_SHIFT => Ok(0xE5),
            KC_RIGHT_ALT => Ok(0xE6),
            KC_RIGHT_GUI => Ok(0xE7),
            KC_APP => Ok(0x65),

            KC_MNEXT => Ok(0xB5),
            KC_MPREV => Ok(0xB6),
            KC_MSTOP => Ok(0xB7),
            KC_MPLAY_PAUSE => Ok(0xCD),

            _ => Err("Can't convert non usb key code"),
        }
    }
}

#[derive(Copy, Clone)]
#[allow(non_camel_case_types, unused)]
#[repr(u8)]
pub enum Modifers {
    MOD_LCTRL   = 0b00000001,
    MOD_LSHIFT  = 0b00000010,
    MOD_LALT    = 0b00000100,
    MOD_LGUI    = 0b00001000,
    MOD_RCTRL   = 0b00010000,
    MOD_RSHIFT  = 0b00100000,
    MOD_RALT    = 0b01000000,
    MOD_RGUI    = 0b10000000,
}

// impl From<u8> for Modifers {
//     fn from(value: u8) -> Self {
//         match value {
//             0b00000001 => Modifers::MOD_LCTRL,
//             0b00000010 => Modifers::MOD_LSHIFT,
//             0b00000100 => Modifers::MOD_LALT,
//             0b00001000 => Modifers::MOD_LGUI,
//             0b00010000 => Modifers::MOD_RCTRL,
//             0b00100000 => Modifers::MOD_RSHIFT,
//             0b01000000 => Modifers::MOD_RALT,
//             0b10000000 => Modifers::MOD_RGUI,
//         }
//     }
// }

impl Into<u8> for Modifers {
    fn into(self) -> u8 {
        match self {
            Self::MOD_LCTRL   => 0b00000001,
            Self::MOD_LSHIFT  => 0b00000010,
            Self::MOD_LALT    => 0b00000100,
            Self::MOD_LGUI    => 0b00001000,
            Self::MOD_RCTRL   => 0b00010000,
            Self::MOD_RSHIFT  => 0b00100000,
            Self::MOD_RALT    => 0b01000000,
            Self::MOD_RGUI    => 0b10000000,
        }
    }
}
