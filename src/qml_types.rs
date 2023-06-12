use crate::{Config, CONFIG_FILE_NAME};
use cpp::cpp;
use gilrs::{Button, EventType, Gilrs};
use log::error;
use qmetaobject::prelude::*;
use std::fs;
use std::process::{Child, Command};

#[derive(QObject, Default)]
pub struct LauncherApp {
    base: qt_base_class!(trait QObject),

    load_config: qt_method!(fn(&mut self) -> bool),

    get_item_count: qt_method!(fn(&self) -> usize),
    get_item_icon: qt_method!(fn(&self, idx: usize) -> QString),
    get_item_name: qt_method!(fn(&self, idx: usize) -> QString),

    exec_item: qt_method!(fn(&self, idx: usize) -> bool),
    has_running_child: qt_method!(fn(&mut self) -> bool),

    init_gamepad_polling: qt_method!(fn(&self) -> bool),
    poll_gamepad: qt_method!(fn(&mut self)),

    config: Config,
    child: Option<Child>,

    gamepad_holder: Option<Gilrs>,
}

// Load config
impl LauncherApp {
    fn load_config(&mut self) -> bool {
        match Self::try_load_config() {
            Ok(config) => {
                self.config = config;
                true
            }
            Err(message) => {
                error!("{}", message);
                false
            }
        }
    }

    fn try_load_config() -> Result<Config, String> {
        let xdg_dirs = xdg::BaseDirectories::new()
            .map_err(|error| format!("Failed to get XDG directories {}", error))?;

        let config_path = xdg_dirs.get_config_file(CONFIG_FILE_NAME);
        let contents = fs::read_to_string(config_path)
            .map_err(|error| format!("Failed to open the config file: {}", error))?;

        let config = toml::from_str::<Config>(&contents)
            .map_err(|error| format!("Failed to parse the config file: {}", error))?;

        Ok(config)
    }
}

// Get items
impl LauncherApp {
    fn get_item_count(&self) -> usize {
        self.config.items.len()
    }

    fn get_item_icon(&self, idx: usize) -> QString {
        if idx >= self.config.items.len() {
            error!("Try to get an item icon by the invalid index: {}", idx);
            return QString::from("");
        }

        self.config.items[idx].icon.as_str().into()
    }

    fn get_item_name(&self, idx: usize) -> QString {
        if idx >= self.config.items.len() {
            error!("Try to get an item icon by the invalid index: {}", idx);
            return QString::from("");
        }

        self.config.items[idx].name.as_str().into()
    }
}

// Applications running
impl LauncherApp {
    fn has_running_child(&mut self) -> bool {
        if let Some(child) = &mut self.child {
            if let Ok(None) = child.try_wait() {
                return true;
            }
        }

        false
    }

    fn exec_item(&mut self, idx: usize) -> bool {
        if self.has_running_child() {
            return false;
        }

        match self.try_exec_item(idx) {
            Ok(child) => {
                self.child = Some(child);
                true
            }
            Err(message) => {
                error!("{}", message);
                false
            }
        }
    }

    fn try_exec_item(&self, idx: usize) -> Result<Child, String> {
        let item = self
            .config
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

        Ok(child)
    }
}

// Gamepad polling
impl LauncherApp {
    fn init_gamepad_polling(&mut self) -> bool {
        match Gilrs::new() {
            Ok(gamepads) => {
                self.gamepad_holder = Some(gamepads);
                true
            }
            Err(error) => {
                error!("Failed to init gamepad polling: {}", error);
                false
            }
        }
    }

    fn poll_gamepad(&mut self) {
        if let Some(gamepad_holder) = &mut self.gamepad_holder {
            while let Some(event) = gamepad_holder.next_event() {
                match event.event {
                    EventType::ButtonPressed(btn, _) => {
                        if let Some(key) = map_button(btn) {
                            push_event(key, ControlKeyState::Pressed(false));
                        }
                    }
                    EventType::ButtonRepeated(btn, _) => {
                        if let Some(key) = map_button(btn) {
                            push_event(key, ControlKeyState::Pressed(true));
                        }
                    }
                    EventType::ButtonReleased(btn, _) => {
                        if let Some(key) = map_button(btn) {
                            push_event(key, ControlKeyState::Released);
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}

#[derive(Debug)]
enum ControlKey {
    Up,
    Right,
    Down,
    Left,
    Enter,
}

#[derive(Debug)]
enum ControlKeyState {
    Pressed(bool),
    Released,
}

fn map_button(btn: Button) -> Option<ControlKey> {
    match btn {
        Button::South => Some(ControlKey::Enter),
        Button::DPadUp => Some(ControlKey::Up),
        Button::DPadDown => Some(ControlKey::Down),
        Button::DPadLeft => Some(ControlKey::Left),
        Button::DPadRight => Some(ControlKey::Right),
        _ => None,
    }
}

cpp! {{
    #include <QtGui/QKeyEvent>
    #include <QtGui/QGuiApplication>
    #include <QtGui/QWindow>
}}

fn push_event(key: ControlKey, state: ControlKeyState) {
    let code: i32 = match key {
        ControlKey::Up => 0x01000013,
        ControlKey::Right => 0x01000014,
        ControlKey::Down => 0x01000015,
        ControlKey::Left => 0x01000012,
        ControlKey::Enter => 0x01000004,
    };
    match state {
        ControlKeyState::Pressed(auto_repeat) => {
            cpp!(unsafe [code as "int", auto_repeat as "bool"] {
                QWindow* window = QGuiApplication::focusWindow();
                if (window != nullptr)
                {
                    QKeyEvent evt(QEvent::KeyPress, code, Qt::NoModifier, QString(), auto_repeat);
                    QGuiApplication::sendEvent(window, &evt);
                }
            });
        }
        ControlKeyState::Released => {
            cpp!(unsafe [code as "int"] {
                QWindow* window = QGuiApplication::focusWindow();
                if (window != nullptr)
                {
                    QKeyEvent evt(QEvent::KeyRelease, code, Qt::NoModifier);
                    QGuiApplication::sendEvent(window, &evt);
                }
            });
        }
    }
}
