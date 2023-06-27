mod launcher_impl;

use launcher_impl::Launcher;
use log::error;

pub struct QmlLauncher {
    launcher: Launcher,
}

impl QmlLauncher {
    fn init(&mut self) -> bool {
        match Launcher::new() {
            Ok(launcher) => {
                let item_count = launcher.items.len() as i32;

                self.launcher = launcher;

                /*                if item_count > 0 {
                    self.begin_insert_rows(0, item_count - 1);
                    self.end_insert_rows();
                }*/

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
