mod filter_axis_to_dpad_buttons;
mod filter_dpad_button_events;
mod keymap;
mod model;

use filter_axis_to_dpad_buttons::left_axis_to_dpad_btn;
use filter_dpad_button_events::filter_wrong_dpad_events;
use model::{create_model_and_tracking_state, GamepadTrackingState, UpdatePowerInfo};

use gilrs::ev::filter::{axis_dpad_to_button, deadzone, Jitter, Repeat};
use gilrs::{EventType, Filter, GamepadId, Gilrs, GilrsBuilder};
use slint::platform::WindowEvent;
use slint::{Model, VecModel, Window};
use std::rc::Rc;
use std::time::Duration;

use crate::winit::WinitWindow;
use crate::GamepadModel;

pub struct GamepadManager {
    gilrs: Gilrs,
    states: Vec<GamepadTrackingState>,
    models: Rc<VecModel<GamepadModel>>,
}

impl GamepadManager {
    pub fn new() -> Result<Self, String> {
        let gilrs = GilrsBuilder::new()
            .with_default_filters(false)
            .set_update_state(false)
            .build()
            .map_err(|error| format!("Failed to init gamepad input backend: {}", error))?;

        let mut states = Vec::default();
        let models = VecModel::default();

        for (model, state) in gilrs
            .gamepads()
            .filter(|(_, g)| g.is_connected())
            .map(|(_, g)| create_model_and_tracking_state(g))
        {
            states.push(state);
            models.push(model);
        }

        Ok(Self {
            gilrs,
            states,
            models: Rc::new(models),
        })
    }

    pub fn model(&self) -> Rc<VecModel<GamepadModel>> {
        self.models.clone()
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
                    let (model, state) = create_model_and_tracking_state(gilrs.gamepad(gamepad_id));
                    self.states.push(state);
                    self.models.push(model);
                }
                EventType::Disconnected => {
                    if let Some(idx) = self
                        .states
                        .iter()
                        .position(|state| state.gamepad_id == gamepad_id)
                    {
                        self.states.remove(idx);
                        self.models.remove(idx);
                    }
                }
                _ => continue,
            }
        }

        self.update_power_info();
    }

    fn update_power_info(&mut self) {
        for (idx, state) in self.states.iter_mut().enumerate() {
            if state.get_seconds_since_last_update() < 0.5 {
                continue;
            }

            state.reset_update_time();
            _ = update_gamepad_model(&self.gilrs, state.gamepad_id, &self.models, idx);
        }
    }
}

fn update_gamepad_model(
    gilrs: &Gilrs,
    id: GamepadId,
    models: &Rc<VecModel<GamepadModel>>,
    idx: usize,
) -> Option<()> {
    let power_info = gilrs.connected_gamepad(id)?.power_info();

    let mut model = models.row_data(idx)?;
    if model.update_power_info(power_info) {
        models.set_row_data(idx, model);
    }

    Some(())
}
