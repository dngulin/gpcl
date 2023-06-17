mod filter_axis_to_dpad_buttons;
mod filter_dpad_button_events;
mod gamepad_list_model;
mod gamepad_manager_impl;
mod keymap;
mod q_gui_app_event;

use gamepad_list_model::convert_power_info;
use gamepad_manager_impl::{GamepadManager, StateChange};
use gilrs::GamepadId;
use keymap::{Key, KeyState};

use log::error;
use qmetaobject::prelude::*;

pub use gamepad_list_model::QmlPowerStatus;

#[derive(QObject, Default)]
pub struct QmlGamepadManager {
    base: qt_base_class!(trait QAbstractListModel),

    init: qt_method!(fn(&self) -> bool),
    poll: qt_method!(fn(&mut self)),

    manager: Option<GamepadManager>,
    gamepads: Vec<gamepad_list_model::Item>,
}

impl QmlGamepadManager {
    fn init(&mut self) -> bool {
        match GamepadManager::new() {
            Ok(manager) => {
                let gamepads = manager.gamepads();

                self.begin_insert_rows(0, gamepads.len().saturating_sub(1) as i32);
                for gamepad in gamepads {
                    if gamepad.is_connected() {
                        self.gamepads.push(gamepad.into());
                    }
                }
                self.end_insert_rows();

                self.manager = Some(manager);
                true
            }
            Err(message) => {
                error!("{}", message);
                false
            }
        }
    }

    fn poll(&mut self) {
        if self.manager.is_none() {
            return;
        }

        while let Some(state_change) = self.poll_manager() {
            match state_change {
                StateChange::UpdateKey(key, key_state) => {
                    Self::send_key_event(key, key_state);
                }
                StateChange::AddGamepad(gamepad) => {
                    let item = gamepad.into();
                    let idx = self.gamepads.len() as i32;

                    self.begin_insert_rows(idx, idx);
                    self.gamepads.push(item);
                    self.end_insert_rows();
                }
                StateChange::RemoveGamepad(gamepad_id) => {
                    if let Some(idx) = self.get_item_index(gamepad_id) {
                        self.begin_remove_rows(idx as i32, idx as i32);
                        self.gamepads.remove(idx);
                        self.end_remove_rows();
                    }
                }
            }
        }

        self.update_items();
    }

    fn poll_manager(&mut self) -> Option<StateChange> {
        self.manager.as_mut()?.poll()
    }

    fn send_key_event(key: Key, key_state: KeyState) {
        let key_code = key as i32;
        match key_state {
            KeyState::Pressed(is_auto_repeat) => {
                q_gui_app_event::send_key_press(key_code, is_auto_repeat);
            }
            KeyState::Released => {
                q_gui_app_event::send_key_release(key_code);
            }
        }
    }

    fn get_item_index(&self, id: GamepadId) -> Option<usize> {
        self.gamepads.iter().position(|item| item.id == id)
    }

    fn update_items(&mut self) {
        for idx in 0..self.gamepads.len() {
            let item = &mut self.gamepads[idx];
            if item.get_seconds_since_last_update() < 0.5 {
                continue;
            }

            if let Some(power_info) = self.manager.as_ref().map(|m| m.get_power_info(item.id)) {
                let (status, charge) = convert_power_info(power_info);

                item.reset_update_time();

                if item.status != status || item.charge != charge {
                    item.status = status;
                    item.charge = charge;

                    let model_idx_from = self.row_index(idx as i32);
                    let model_idx_to = self.row_index(idx as i32);
                    self.data_changed(model_idx_from, model_idx_to);
                }
            } else {
                error!("Failed to get power info for `{}`", item.name)
            }
        }
    }
}
