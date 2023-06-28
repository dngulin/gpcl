mod filter_axis_to_dpad_buttons;
mod filter_dpad_button_events;
mod gamepad_item;
mod keymap;

use filter_axis_to_dpad_buttons::left_axis_to_dpad_btn;
use filter_dpad_button_events::filter_wrong_dpad_events;
use gamepad_item::GamepadItem;

use gilrs::ev::filter::{axis_dpad_to_button, deadzone, Jitter, Repeat};
use gilrs::{EventType, Filter, Gamepad, Gilrs, GilrsBuilder, PowerInfo};
use slint::platform::WindowEvent;
use slint::Window;
use std::rc::Rc;
use std::time::Duration;

use crate::slint_bridge::ExtVecModel;
use crate::{GamepadModel, GamepadStatus};

pub struct GamepadManager {
    gilrs: Gilrs,
    gamepads: Rc<ExtVecModel<GamepadModel, GamepadItem>>,
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
            .map(|(id, g)| (GamepadModel::from(g), GamepadItem::new(id)))
            .collect::<Vec<(GamepadModel, GamepadItem)>>();
        let gamepads = Rc::new(ExtVecModel::new(gamepads));

        Ok(Self { gilrs, gamepads })
    }

    pub fn model(&self) -> Rc<ExtVecModel<GamepadModel, GamepadItem>> {
        self.gamepads.clone()
    }

    pub fn poll(&mut self, window: &Window) {
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
                EventType::ButtonPressed(btn, _) | EventType::ButtonRepeated(btn, _) => {
                    if let Some(key) = keymap::btn_to_key(btn) {
                        window.dispatch_event(WindowEvent::KeyPressed { text: key.into() });
                    }
                }
                EventType::ButtonReleased(btn, _) => {
                    if let Some(key) = keymap::btn_to_key(btn) {
                        window.dispatch_event(WindowEvent::KeyReleased { text: key.into() });
                    }
                }
                EventType::Connected => {
                    let id = event.id;
                    let pair = (gilrs.gamepad(id).into(), GamepadItem::new(id));
                    self.gamepads.add(pair);
                }
                EventType::Disconnected => {
                    self.gamepads
                        .remove(|(_, tracking)| tracking.id == event.id);
                }
                _ => continue,
            }
        }

        self.update_power_info();
    }

    fn update_power_info(&mut self) {
        self.gamepads.update_items(|model, tracking| {
            if tracking.get_seconds_since_last_update() < 0.5 {
                return false;
            }

            tracking.reset_update_time();

            if let Some(power_info) = self
                .gilrs
                .connected_gamepad(tracking.id)
                .map(|g| g.power_info())
            {
                let (status, charge) = convert_power_info(power_info);
                if model.status != status || model.charge != charge {
                    model.status = status;
                    model.charge = charge;
                    return true;
                }
            } else {
                log::error!("Failed to get power info for `{}`", model.name)
            }

            false
        });
    }
}

fn convert_power_info(power_info: PowerInfo) -> (GamepadStatus, i32) {
    match power_info {
        PowerInfo::Unknown | PowerInfo::Wired => (GamepadStatus::Wired, 100),
        PowerInfo::Discharging(charge) => (GamepadStatus::Discharging, charge as i32),
        PowerInfo::Charging(charge) => (GamepadStatus::Charging, charge as i32),
        PowerInfo::Charged => (GamepadStatus::Charging, 100),
    }
}

impl<'a> From<Gamepad<'a>> for GamepadModel {
    fn from(gamepad: Gamepad<'a>) -> GamepadModel {
        let name = gamepad.name().into();
        let (status, charge) = convert_power_info(gamepad.power_info());

        GamepadModel {
            name,
            status,
            charge,
        }
    }
}
