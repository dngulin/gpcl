use super::keymap::Key;
use crate::gamepad_manager::keymap::KeyState;
use crate::gamepad_manager::keymap::KeyState::{Pressed, Released};
use gilrs::{EventType, Gilrs};

pub struct GamepadManager {
    gilrs: Gilrs,
}

impl GamepadManager {
    pub fn new() -> Result<Self, String> {
        let gilrs =
            Gilrs::new().map_err(|error| format!("Failed to init input backend: {}", error))?;

        Ok(Self { gilrs })
    }

    pub fn next_event(&mut self) -> Option<(Key, KeyState)> {
        while let Some(event) = self.gilrs.next_event() {
            match event.event {
                EventType::ButtonPressed(btn, _) => {
                    if let Ok(key) = Key::try_from(btn) {
                        return Some((key, Pressed(false)));
                    }
                }
                EventType::ButtonReleased(btn, _) => {
                    if let Ok(key) = Key::try_from(btn) {
                        return Some((key, Released));
                    }
                }
                _ => continue,
            }
        }
        None
    }
}
