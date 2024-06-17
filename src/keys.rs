
pub enum Button{
    LB,
    RB,
    Tab,
    CapsLock,
    Esc,
    LShift,
    RShift,
    Shift,
    LControl,
    RControl,
    Control,
    LAlt,
    RAlt,
    Alt,
    Return,
    Backspace,
    Delete,
    Char(char),
    Other(i16),

}

impl From <i16> for Button{
    fn from(value: i16) -> Self {
        match value{
            0x01 => Button::LB,
            0x02 => Button::RB,
            0x09 => Button::Tab,
            0x0d => Button::Return,
            0x08 => Button::Backspace,
            0x10 => Button::Shift,
            0x11 => Button::Control,
            0x12 => Button::Alt,
            _ => todo!()


        }
    }
}