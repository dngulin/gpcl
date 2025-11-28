mod model;

use crate::launcher::model::config_entry_into_item;
use crate::{config::AppIconConfig, AppIconModel};
use model::Executable;

use slint::VecModel;
use std::process::{Child, Command};
use std::rc::Rc;

pub struct Launcher {
    items: Vec<Executable>,
    item_icons: Rc<VecModel<AppIconModel>>,
    child_process: Option<Child>,
}

impl Launcher {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            item_icons: Rc::new(VecModel::default()),
            child_process: None,
        }
    }

    pub fn reset_items(&mut self, items: &[AppIconConfig]) {
        self.items.clear();
        self.item_icons.clear();

        for (icon, item) in items.iter().map(config_entry_into_item) {
            self.items.push(item);
            self.item_icons.push(icon);
        }
    }

    pub fn model(&self) -> Rc<VecModel<AppIconModel>> {
        self.item_icons.clone()
    }

    pub fn exec_item(&mut self, idx: usize) {
        if self.check_if_child_is_running() {
            log::warn!("Try to run more than one application at once");
            return;
        }

        if let Some(exec) = self.items.get(idx) {
            let child = match Command::new(&exec.program).args(&exec.args).spawn() {
                Ok(child) => Some(child),
                Err(error) => {
                    log::error!(
                        "Failed to execute the command `{}`: {}",
                        exec.program,
                        error
                    );
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
