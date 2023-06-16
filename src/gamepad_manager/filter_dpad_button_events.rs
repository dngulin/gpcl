use gilrs::{Button, Event, EventType, Gilrs};

pub fn filter_wrong_dpad_events(ev: Option<Event>, gilrs: &mut Gilrs) -> Option<Event> {
    let event = ev?;
    let gamepad = gilrs.gamepad(event.id);

    match event.event {
        EventType::ButtonPressed(btn, _) if is_dpad_btn(btn) && gamepad.is_pressed(btn) => {
            Some(event.drop())
        }
        EventType::ButtonReleased(btn, _) if is_dpad_btn(btn) && !gamepad.is_pressed(btn) => {
            Some(event.drop())
        }
        _ => ev,
    }
}

fn is_dpad_btn(btn: Button) -> bool {
    matches!(
        btn,
        Button::DPadUp | Button::DPadDown | Button::DPadLeft | Button::DPadRight
    )
}
