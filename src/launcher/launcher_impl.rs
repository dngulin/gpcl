use crate::{AppItem, Config, CONFIG_FILE_NAME};
use std::fs;
use std::process::{Child, Command};

#[derive(Default)]
pub struct Launcher {
    pub items: Vec<AppItem>,
    child: Option<Child>,
}

impl Launcher {
    pub fn new() -> Result<Self, String> {
        let xdg_dirs = xdg::BaseDirectories::new()
            .map_err(|error| format!("Failed to get XDG directories {}", error))?;

        let config_path = xdg_dirs.get_config_file(CONFIG_FILE_NAME);
        let contents = fs::read_to_string(config_path)
            .map_err(|error| format!("Failed to open the config file: {}", error))?;

        let config = toml::from_str::<Config>(&contents)
            .map_err(|error| format!("Failed to parse the config file: {}", error))?;

        Ok(Self {
            items: config.items,
            child: None,
        })
    }

    pub fn exec_item(&mut self, idx: usize) -> Result<(), String> {
        let item = self
            .items
            .get(idx)
            .ok_or(format!("Fail to get an exec item by the index: {}", idx))?;

        let mut tokens = item.exec.split_whitespace();
        let program = tokens.next().ok_or(format!(
            "Failed to get the program form the exec string: {}",
            item.exec
        ))?;

        let args: Vec<&str> = tokens.collect();

        let child = Command::new(program)
            .args(args)
            .spawn()
            .map_err(|error| format!("Failed to execute the command `{}`: {}", item.exec, error))?;

        self.child = Some(child);

        Ok(())
    }

    pub fn has_running_item(&mut self) -> bool {
        if let Some(child) = &mut self.child {
            if let Ok(None) = child.try_wait() {
                return true;
            }
        }

        false
    }
}
