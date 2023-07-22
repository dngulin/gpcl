use i_slint_backend_winit::Backend;
use i_slint_backend_winit::WinitWindowAccessor;
use slint::Window;

pub fn set_as_backend() -> Result<(), slint::platform::SetPlatformError> {
    slint::platform::set_platform(Box::new(Backend::new()))
}

pub trait WinitWindow {
    fn has_focus(&self) -> bool;
    fn hide_cursor(&self);
}

impl WinitWindow for Window {
    fn has_focus(&self) -> bool {
        self.with_winit_window(|ww| ww.has_focus()).unwrap()
    }

    fn hide_cursor(&self) {
        self.with_winit_window(|ww| ww.set_cursor_visible(false));
    }
}
