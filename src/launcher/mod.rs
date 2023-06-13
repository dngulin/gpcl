mod app_list_model;
mod launcher_impl;

use launcher_impl::Launcher;
use log::error;
use qmetaobject::prelude::*;

#[derive(QObject, Default)]
pub struct QmlLauncher {
    base: qt_base_class!(trait QAbstractListModel),

    init: qt_method!(fn(&mut self) -> bool),
    exec_item: qt_method!(fn(&self, idx: usize) -> bool),
    has_running_item: qt_method!(fn(&mut self) -> bool),

    launcher: Launcher,
}

impl QmlLauncher {
    fn init(&mut self) -> bool {
        match Launcher::new() {
            Ok(launcher) => {
                self.begin_insert_rows(0, (launcher.items.len() - 1) as i32);
                self.launcher = launcher;
                self.end_insert_rows();
                true
            }
            Err(message) => {
                error!("{}", message);
                false
            }
        }
    }

    fn exec_item(&mut self, idx: usize) -> bool {
        if self.launcher.has_running_item() {
            return false;
        }

        if let Err(message) = self.launcher.exec_item(idx) {
            error!("{}", message);
            return false;
        }

        true
    }

    fn has_running_item(&mut self) -> bool {
        self.launcher.has_running_item()
    }
}
