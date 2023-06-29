mod config;
mod gamepad_manager;
mod launcher;
mod slint_models;

use config::Config;
use gamepad_manager::GamepadManager;
use launcher::Launcher;

use slint::{Timer, TimerMode};
use std::fs;
use std::time::Duration;

slint::include_modules!();

pub const CONFIG_FILE_NAME: &str = "gpcl.toml";

fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .init();

    let window = MainWindow::new().unwrap();

    let _gp_timer = setup_gamepad_manager(&window);
    setup_launcher(&window);

    take_focus_hack(&window);
    window.run().unwrap();
}

fn setup_gamepad_manager(window: &MainWindow) -> Timer {
    let mut gamepad_manager = GamepadManager::new().unwrap();
    window.set_gamepad_list(gamepad_manager.model().into());

    let window_weak = window.as_weak();
    let gamepad_poll_timer = Timer::default();

    gamepad_poll_timer.start(TimerMode::Repeated, Duration::from_millis(16), move || {
        if let Some(window) = window_weak.upgrade() {
            gamepad_manager.poll(window.window());
        }
    });

    gamepad_poll_timer
}

fn setup_launcher(window: &MainWindow) {
    let xdg_dirs = xdg::BaseDirectories::new().unwrap();

    let config_path = xdg_dirs.get_config_file(CONFIG_FILE_NAME);
    let contents = fs::read_to_string(config_path).unwrap();

    let config = toml::from_str::<Config>(&contents).unwrap();

    let mut launcher = Launcher::new(&config.items);

    window.set_app_list(launcher.model().into());
    window.on_app_icon_activated(move |idx| launcher.exec_item(idx as usize));
}

// Workaround for https://github.com/slint-ui/slint/issues/2201
fn take_focus_hack(window: &MainWindow) {
    window
        .as_weak()
        .upgrade_in_event_loop(move |window| {
            window.invoke_take_focus_workaround();
        })
        .unwrap();
}
