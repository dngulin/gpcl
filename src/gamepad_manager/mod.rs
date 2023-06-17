mod filter_axis_to_dpad_buttons;
mod filter_dpad_button_events;
mod gamepad_list_model;
mod gamepad_manager_impl;
mod keymap;
mod q_gui_app_event;

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
}
