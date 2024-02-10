mod filter_axis_to_dpad_buttons;
mod filter_dpad_button_events;
mod keymap;
mod model;

use filter_axis_to_dpad_buttons::left_axis_to_dpad_btn;
use filter_dpad_button_events::filter_wrong_dpad_events;
use model::{gamepad_to_model_item, TrackingState, UpdatePowerInfo};

use gilrs::ev::filter::{axis_dpad_to_button, deadzone, Jitter, Repeat};
use gilrs::{EventType, Filter, GamepadId, Gilrs, GilrsBuilder, PowerInfo};
use slint::platform::WindowEvent;
use slint::Window;
use std::rc::Rc;
use std::time::Duration;

use crate::slint_models::ExtVecModel;
use crate::winit::WinitWindow;
use crate::GamepadModel;

pub struct GamepadManager {
    gilrs: Gilrs,
    gamepads: Rc<ExtVecModel<GamepadModel, TrackingState>>,
}

impl GamepadManager {
    pub fn new() -> Result<Self, String> {
        let gilrs = GilrsBuilder::new()
            .with_default_filters(false)
            .set_update_state(false)
            .build()
            .map_err(|error| format!("Failed to init gamepad input backend: {}", error))?;

        let gamepads = gilrs
            .gamepads()
            .filter(|(_, g)| g.is_connected())
            .map(|(_, g)| gamepad_to_model_item(g))
            .collect();
        let gamepads = Rc::new(ExtVecModel::with_items(gamepads));

        Ok(Self { gilrs, gamepads })
    }

    pub fn model(&self) -> Rc<ExtVecModel<GamepadModel, TrackingState>> {
        self.gamepads.clone()
    }

    pub fn poll(&mut self, window: &Window) {
        let has_focus = window.has_focus();

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
            let gamepad_id = event.id;

            match event.event {
                EventType::ButtonPressed(btn, _) if has_focus => {
                    if let Some(key) = keymap::btn_to_key(btn) {
                        window.dispatch_event(WindowEvent::KeyPressed { text: key.into() });
                    }
                }
                EventType::ButtonRepeated(btn, _) if has_focus => {
                    if let Some(key) = keymap::btn_to_key(btn) {
                        window.dispatch_event(WindowEvent::KeyPressRepeated { text: key.into() });
                    }
                }
                EventType::ButtonReleased(btn, _) if has_focus => {
                    if let Some(key) = keymap::btn_to_key(btn) {
                        window.dispatch_event(WindowEvent::KeyReleased { text: key.into() });
                    }
                }
                EventType::Connected => {
                    let item = gamepad_to_model_item(gilrs.gamepad(gamepad_id));
                    self.gamepads.add(item);
                }
                EventType::Disconnected => {
                    self.gamepads
                        .remove(|(_, tracking_state)| tracking_state.gamepad_id == gamepad_id);
                }
                _ => continue,
            }
        }

        self.update_power_info();
    }

    fn update_power_info(&mut self) {
        self.gamepads.update_items(|model, tracking_state| {
            if tracking_state.get_seconds_since_last_update() < 0.5 {
                return false;
            }

            tracking_state.reset_update_time();

            if let Some(info) = get_power_info(&self.gilrs, tracking_state.gamepad_id) {
                return model.update_power_info(info);
            }

            log::error!("Failed to get power info for `{}`", model.name);
            false
        });
    }
}

fn get_power_info(gilrs: &Gilrs, gamepad_id: GamepadId) -> Option<PowerInfo> {
    gilrs.connected_gamepad(gamepad_id).map(|g| g.power_info())
}
