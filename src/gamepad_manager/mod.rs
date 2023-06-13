mod keymap;
mod q_gui_app_event;

use qmetaobject::prelude::*;

#[derive(QObject, Default)]
pub struct QmlGamepadManager {
    base: qt_base_class!(trait QObject),

    init: qt_method!(fn(&self) -> bool),
    poll: qt_method!(fn(&mut self)),
}

impl QmlGamepadManager {
    fn init(&mut self) -> bool {
        todo!() // init impl
    }

    fn poll(&mut self) {
        todo!() // Iterate over impl
    }
}
