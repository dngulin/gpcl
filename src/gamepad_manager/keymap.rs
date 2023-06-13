use gilrs::Button;

#[derive(Debug)]
#[repr(i32)]
pub enum Key {
    Escape = 0x01000000,
    Return = 0x01000004,
    Left = 0x01000012,
    Up = 0x01000013,
    Right = 0x01000014,
    Down = 0x01000015,
}

#[derive(Debug)]
pub enum KeyState {
    Pressed(bool), // Auto-repeat flag
    Released,
}

impl TryFrom<Button> for Key {
    type Error = ();

    fn try_from(value: Button) -> Result<Self, Self::Error> {
        match value {
            Button::South => Ok(Key::Return),
            Button::East => Ok(Key::Escape),
            Button::DPadUp => Ok(Key::Up),
            Button::DPadDown => Ok(Key::Down),
            Button::DPadLeft => Ok(Key::Left),
            Button::DPadRight => Ok(Key::Right),
            _ => Err(()),
        }
    }
}
