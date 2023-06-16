mod filter_axis_to_dpad_buttons;
mod filter_dpad_button_events;
mod gamepad_list_model;
mod gamepad_manager_impl;
mod keymap;
mod q_gui_app_event;

use gamepad_manager_impl::GamepadManager;
use keymap::KeyState;

use log::error;
use qmetaobject::prelude::*;

pub use gamepad_list_model::QmlPowerStatus;

#[derive(QObject, Default)]
pub struct QmlGamepadManager {
    base: qt_base_class!(trait QAbstractListModel),

    init: qt_method!(fn(&self) -> bool),
    poll: qt_method!(fn(&mut self)),

    manager: Option<GamepadManager>,
    gamepad_list: Vec<gamepad_list_model::Item>,
}

impl QmlGamepadManager {
    fn init(&mut self) -> bool {
        match GamepadManager::new() {
            Ok(manager) => {
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
        if let Some(manager) = &mut self.manager {
            while let Some((key, key_state)) = manager.next_event() {
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
        }
    }
}
