mod filter_axis_to_dpad_buttons;
mod filter_dpad_button_events;
mod gamepad_item;
mod keymap;

use filter_axis_to_dpad_buttons::left_axis_to_dpad_btn;
use filter_dpad_button_events::filter_wrong_dpad_events;
use std::rc::Rc;

use gilrs::ev::filter::{axis_dpad_to_button, deadzone, Jitter, Repeat};
use gilrs::{EventType, Filter, GamepadId, Gilrs, GilrsBuilder};
use slint::platform::WindowEvent;
use slint::{MapModel, Model, VecModel, Window};
use std::time::Duration;

use crate::GamepadModel;
use gamepad_item::GamepadItem;

type RcVec<T> = Rc<VecModel<T>>;
type RcMap<T, F> = Rc<MapModel<T, F>>;

pub struct GamepadManager {
    gilrs: Gilrs,
    gamepads: RcVec<GamepadItem>,
}

impl GamepadManager {
    pub fn new() -> Result<Self, String> {
        let gilrs = GilrsBuilder::new()
            .with_default_filters(false)
            .set_update_state(false)
            .build()
            .map_err(|error| format!("Failed to init gamepad input backend: {}", error))?;

        let gamepads: VecModel<GamepadItem> = gilrs
            .gamepads()
            .map(|(_, g)| g)
            .filter(|g| g.is_connected())
            .map(|g| g.into())
            .collect::<Vec<GamepadItem>>()
            .into();
        let gamepads = Rc::new(gamepads);

        Ok(Self { gilrs, gamepads })
    }

    pub fn model(&self) -> RcMap<RcVec<GamepadItem>, impl Fn(GamepadItem) -> GamepadModel> {
        let model = MapModel::new(self.gamepads.clone(), |i| i.into());
        Rc::new(model)
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
                    self.gamepads.push(gilrs.gamepad(event.id).into());
                }
                EventType::Disconnected => {
                    if let Some(idx) = find_index(&self.gamepads, event.id) {
                        self.gamepads.remove(idx);
                    }
                }
                _ => continue,
            }
        }
    }
}

fn find_index(gamepads: &RcVec<GamepadItem>, id: GamepadId) -> Option<usize> {
    for (idx, item) in gamepads.iter().enumerate() {
        if item.id == id {
            return Some(idx);
        }
    }
    None
}
