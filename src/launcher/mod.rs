mod model;

use model::Executable;
use std::ops::Deref;

use crate::slint_models::ExtVecModel;
use crate::{config::AppIconConfig, AppIconModel};

use crate::launcher::model::config_entry_into_item;
use std::process::{Child, Command};
use std::rc::Rc;

pub struct Launcher {
    items: Rc<ExtVecModel<AppIconModel, Executable>>,
    child_process: Option<Child>,
}

impl Launcher {
    pub fn new() -> Self {
        Self {
            items: Rc::new(ExtVecModel::new()),
            child_process: None,
        }
    }

    pub fn reset_items(&self, items: &[AppIconConfig]) {
        self.items.clear();
        for item in items.iter().map(config_entry_into_item) {
            self.items.add(item);
        }
    }

    pub fn model(&self) -> Rc<ExtVecModel<AppIconModel, Executable>> {
        self.items.clone()
    }

    pub fn exec_item(&mut self, idx: usize) {
        if self.check_if_child_is_running() {
            log::warn!("Try to run more than one application at once");
            return;
        }

        if let Some(item_ref) = self.items.get_ref(idx) {
            let (model, exec) = item_ref.deref();

            let child = match Command::new(&exec.program).args(&exec.args).spawn() {
                Ok(child) => Some(child),
                Err(error) => {
                    log::error!("Failed to execute the command `{}`: {}", model.name, error);
                    None
                }
            };

            self.child_process = child;
        } else {
            log::error!("Bad model index to run: {}", idx);
        }
    }

    pub fn check_if_child_is_running(&mut self) -> bool {
        if let Some(child) = &mut self.child_process {
            if let Ok(exit_status) = child.try_wait() {
                if exit_status.is_none() {
                    return true;
                }
            }
        }

        false
    }
}
