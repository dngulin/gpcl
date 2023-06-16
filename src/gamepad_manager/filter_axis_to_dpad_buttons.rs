use gilrs::ev::Code;
use gilrs::{Axis, Button, Event, EventType, Gamepad, Gilrs};

struct ButtonMap {
    pub neg: Button,
    pub pos: Button,
}

impl ButtonMap {
    fn neg_pos(neg: Button, pos: Button) -> Self {
        Self { neg, pos }
    }
}

struct ButtonData {
    pub btn: Button,
    pub code: Code,
}

impl ButtonData {
    fn new(btn: Button, code: Code) -> Self {
        Self { btn, code }
    }
}

pub fn left_axis_to_dpad_btn(ev: Option<Event>, gilrs: &mut Gilrs) -> Option<Event> {
    let event = ev?;
    let gamepad = gilrs.gamepad(event.id);

    if let EventType::AxisChanged(axis, new_value, _) = event.event {
        match axis {
            Axis::LeftStickX => {
                let delta = (gamepad.value(axis), new_value);
                let buttons = ButtonMap::neg_pos(Button::DPadLeft, Button::DPadRight);
                if let Some(btn_event) = get_mapped_event(&gamepad, &delta, &buttons) {
                    gilrs.insert_event(btn_event);
                }
            }
            Axis::LeftStickY => {
                let delta = (gamepad.value(axis), new_value);
                let buttons = ButtonMap::neg_pos(Button::DPadDown, Button::DPadUp);
                if let Some(btn_event) = get_mapped_event(&gamepad, &delta, &buttons) {
                    gilrs.insert_event(btn_event);
                }
            }
            _ => {}
        }
    }

    ev
}

fn get_mapped_event(gp: &Gamepad, delta: &(f32, f32), buttons: &ButtonMap) -> Option<Event> {
    if let Some((neg, pos)) = append_btn_codes(gp, buttons) {
        if let Some(event_type) = axis_to_btn(delta, &neg, &pos) {
            return Some(Event::new(gp.id(), event_type));
        }
    }

    None
}

fn append_btn_codes(gamepad: &Gamepad, buttons: &ButtonMap) -> Option<(ButtonData, ButtonData)> {
    let code_neg = gamepad.button_code(buttons.neg)?;
    let code_pos = gamepad.button_code(buttons.pos)?;
    Some((
        ButtonData::new(buttons.neg, code_neg),
        ButtonData::new(buttons.pos, code_pos),
    ))
}

fn axis_to_btn(delta: &(f32, f32), neg: &ButtonData, pos: &ButtonData) -> Option<EventType> {
    const PRESS: f32 = 0.6;
    const RELEASE: f32 = 0.5;

    if delta.0 > -PRESS && delta.1 <= -PRESS {
        return Some(EventType::ButtonPressed(neg.btn, neg.code));
    }
    if delta.0 <= -RELEASE && delta.1 > -RELEASE {
        return Some(EventType::ButtonReleased(neg.btn, neg.code));
    }

    if delta.0 < PRESS && delta.1 >= PRESS {
        return Some(EventType::ButtonPressed(pos.btn, pos.code));
    }
    if delta.0 >= RELEASE && delta.1 < RELEASE {
        return Some(EventType::ButtonReleased(pos.btn, pos.code));
    }

    None
}
