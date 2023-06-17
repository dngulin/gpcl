use super::filter_axis_to_dpad_buttons::left_axis_to_dpad_btn;
use super::filter_dpad_button_events::filter_wrong_dpad_events;
use super::keymap::{Key, KeyState};

use gilrs::ev::filter::{axis_dpad_to_button, deadzone, Jitter, Repeat};
use gilrs::{EventType, Filter, Gamepad, GamepadId, Gilrs, GilrsBuilder, PowerInfo};
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

    pub fn gamepads(&self) -> Vec<Gamepad> {
        self.gilrs
            .gamepads()
            .map(|(_, g)| g)
            .filter(|g| g.is_connected())
            .collect()
    }

    pub fn get_power_info(&self, gamepad_id: GamepadId) -> Option<PowerInfo> {
        self.gilrs
            .connected_gamepad(gamepad_id)
            .map(|g| g.power_info())
    }
}

pub enum StateChange<'a> {
    UpdateKey(Key, KeyState),
    AddGamepad(Gamepad<'a>),
    RemoveGamepad(GamepadId),
}

impl GamepadManager {
    pub fn poll(&mut self) -> Option<StateChange> {
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
                        return Some(StateChange::UpdateKey(key, KeyState::Pressed(false)));
                    }
                }
                EventType::ButtonRepeated(btn, _) => {
                    if let Ok(key) = Key::try_from(btn) {
                        log::info!("repeat: {:?}", key);
                        return Some(StateChange::UpdateKey(key, KeyState::Pressed(true)));
                    }
                }
                EventType::ButtonReleased(btn, _) => {
                    if let Ok(key) = Key::try_from(btn) {
                        return Some(StateChange::UpdateKey(key, KeyState::Released));
                    }
                }
                EventType::Connected => {
                    return Some(StateChange::AddGamepad(gilrs.gamepad(event.id)))
                }
                EventType::Disconnected => return Some(StateChange::RemoveGamepad(event.id)),
                _ => continue,
            }
        }
        None
    }
}
