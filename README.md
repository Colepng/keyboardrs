# Keyboard
Keyboard firmware written in rust 

# Todo
- [ ] abstraction for keycodes - in progress
- [ ] layers
    - [x] switch layers
    - [ ] toggle layers
    - [ ] momentary layers
- [ ] rotary encoders
    - [x] single rotary encoder support
    - [x] multi layer action support
    - [x] multi rotary encoder support
    - [ ] action on holding down and rotating
- [ ] macros
- [ ] row2col scanning
- [ ] duplex matrix scanning
- [ ] square / round-robin matrix scanning
- [ ] mouse keys
- [ ] oled display support
- [ ] led support

# Example warnings
both onekey and late-night-engineering need the encoders feature to be enabled when building
normally if you would enable when your declaring this library as a dependency
