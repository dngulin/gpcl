use super::filter_axis_to_dpad_buttons::left_axis_to_dpad_btn;
use super::filter_dpad_button_events::filter_wrong_dpad_events;
use super::keymap::{Key, KeyState};

use gilrs::ev::filter::{axis_dpad_to_button, deadzone, Jitter, Repeat};
use gilrs::{EventType, Filter, Gilrs, GilrsBuilder};
use std::time::Duration;

pub struct GamepadManager {
    gilrs: Gilrs,
}

impl GamepadManager {
    pub fn new() -> Result<Self, String> {
        let gilrs = GilrsBuilder::new()
            .with_default_filters(false)
            .set_update_state(false)
            .build()
            .map_err(|error| format!("Failed to init gamepad input backend: {}", error))?;

        Ok(Self { gilrs })
    }

    pub fn next_event(&mut self) -> Option<(Key, KeyState)> {
        let gilrs = &mut self.gilrs;

        let jitter = Jitter::new();
        let repeat_filter = Repeat {
            after: Duration::from_millis(600),
            every: Duration::from_millis(50),
        };

        while let Some(event) = gilrs
            .next_event()
            .filter_ev(&axis_dpad_to_button, gilrs)
            .filter_ev(&deadzone, gilrs)
            .filter_ev(&jitter, gilrs)
            .filter_ev(&left_axis_to_dpad_btn, gilrs)
            .filter_ev(&filter_wrong_dpad_events, gilrs)
            .filter_ev(&repeat_filter, gilrs)
        {
            gilrs.update(&event);

            match event.event {
                EventType::ButtonPressed(btn, _) => {
                    if let Ok(key) = Key::try_from(btn) {
                        return Some((key, KeyState::Pressed(false)));
                    }
                }
                EventType::ButtonRepeated(btn, _) => {
                    if let Ok(key) = Key::try_from(btn) {
                        log::info!("repeat: {:?}", key);
                        return Some((key, KeyState::Pressed(true)));
                    }
                }
                EventType::ButtonReleased(btn, _) => {
                    if let Ok(key) = Key::try_from(btn) {
                        return Some((key, KeyState::Released));
                    }
                }
                _ => continue,
            }
        }
        None
    }
}
