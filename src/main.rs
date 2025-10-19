mod clock;
mod config;
mod gamepad_manager;
mod launcher;
mod slint_models;
mod winit;

use config::{Config, LayoutConfig};
use gamepad_manager::GamepadManager;
use launcher::Launcher;
use winit::WinitWindow;

use crate::clock::ClockTracker;
use crate::config::StyleConfig;
use hex_color::HexColor;
use slint::{Color, Timer, TimerMode};
use std::cell::RefCell;
use std::fs;
use std::rc::Rc;
use std::time::Duration;

slint::include_modules!();

pub const CONFIG_FILE_NAME: &str = "gpcl.toml";

fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .init();

    let app = GpclApp::new().unwrap();

    let window = app.window();
    window.set_fullscreen(true);
    _ = app
        .as_weak()
        .upgrade_in_event_loop(move |app| app.window().hide_cursor());

    let launcher = Launcher::new();
    load_and_apply_config(&app, &launcher);

    let launcher = Rc::new(RefCell::new(launcher));
    setup_config_reloading(&app, launcher.clone());

    let _gp_poll_timer = setup_gamepad_manager(&app);
    let _clock_timer = setup_clock(&app);
    let _launcher_timer = setup_launcher(&app, launcher);

    app.run().unwrap();
}

fn load_config_file() -> Config {
    let xdg_dirs = xdg::BaseDirectories::new();
    let config_path = xdg_dirs.get_config_file(CONFIG_FILE_NAME).unwrap();

    let contents = match fs::read_to_string(config_path) {
        Ok(contents) => contents,
        Err(error) => {
            log::error!("Failed to open config: {}", error);
            return Config::default();
        }
    };

    toml::from_str::<Config>(&contents).unwrap_or_else(|error| {
        log::error!("Failed to parse config: {}", error);
        Config::default()
    })
}

fn load_and_apply_config(app: &GpclApp, launcher: &Launcher) {
    let config = load_config_file();

    let layout = app.global::<ScreenLayout>();
    set_window_layout(&layout, &config.layout.unwrap_or_default());

    let style = app.global::<Style>();
    set_app_style(&style, &config.style.unwrap_or_default());

    launcher.reset_items(&config.items);
}

fn set_window_layout(layout: &ScreenLayout, config: &LayoutConfig) {
    let default_panel_height = layout.get_default_top_panel_height();
    layout.set_top_panel_height(config.top_panel_height.unwrap_or(default_panel_height));

    let default_clock_height = layout.get_default_clock_height();
    layout.set_clock_height(config.clock_height.unwrap_or(default_clock_height));

    let default_icon_size = layout.get_default_icon_size();
    layout.set_icon_size(config.icon_size.unwrap_or(default_icon_size));
}

fn set_app_style(style: &Style, config: &StyleConfig) {
    let font_family = config
        .font
        .as_ref()
        .map(|s| s.into())
        .unwrap_or(style.get_default_font_family());
    style.set_font_family(font_family);

    let font_weight = config
        .font_weight
        .unwrap_or(style.get_default_font_weight());
    style.set_font_weight(font_weight);

    let bg_color = get_color(&config.bg_color).unwrap_or(style.get_default_bg_color());
    style.set_bg_color(bg_color);

    let panel_color = get_color(&config.panel_color).unwrap_or(style.get_default_panel_color());
    style.set_panel_color(panel_color);

    let text_color = get_color(&config.text_color).unwrap_or(style.get_default_text_color());
    style.set_text_color(text_color);
}

fn get_color(value: &Option<String>) -> Option<Color> {
    let value = value.as_ref()?;
    match HexColor::parse(value) {
        Ok(color) => Some(Color::from_argb_u8(color.a, color.r, color.g, color.b)),
        Err(error) => {
            log::error!("Failed to parse color `{}`: {}", value, error);
            None
        }
    }
}

fn setup_config_reloading(app: &GpclApp, launcher: Rc<RefCell<Launcher>>) {
    let app_weak = app.as_weak();
    app.on_reload_pressed(move || {
        if let Some(app) = app_weak.upgrade() {
            load_and_apply_config(&app, &launcher.borrow());
        }
    });
}

fn setup_gamepad_manager(app: &GpclApp) -> Timer {
    let mut gamepad_manager = GamepadManager::new().unwrap();
    app.set_gamepad_list(gamepad_manager.model().into());

    let app_weak = app.as_weak();
    let gamepad_poll_timer = Timer::default();

    gamepad_poll_timer.start(TimerMode::Repeated, Duration::from_millis(16), move || {
        if let Some(app) = app_weak.upgrade() {
            gamepad_manager.poll(app.window());
        }
    });

    gamepad_poll_timer
}

fn setup_clock(app: &GpclApp) -> Timer {
    let mut clock_tracker = ClockTracker::new();
    app.set_clock_text(clock_tracker.time_str().into());

    let tracker_cell = Rc::new(RefCell::new(clock_tracker));
    let app_weak = app.as_weak();
    let clock_timer = Timer::default();

    clock_timer.start(TimerMode::Repeated, Duration::from_millis(500), move || {
        if let Some(app) = app_weak.upgrade() {
            let mut tracker = tracker_cell.borrow_mut();
            if tracker.update() {
                app.set_clock_text(tracker.time_str().into());
            }
        }
    });

    clock_timer
}

fn setup_launcher(app: &GpclApp, launcher: Rc<RefCell<Launcher>>) -> Timer {
    app.set_app_list(launcher.borrow().model().into());

    {
        let launcher = launcher.clone();
        app.on_app_icon_activated(move |idx| launcher.borrow_mut().exec_item(idx as usize));
    }

    let app_weak = app.as_weak();
    let child_poll_timer = Timer::default();

    child_poll_timer.start(TimerMode::Repeated, Duration::from_millis(250), move || {
        if let Some(app) = app_weak.upgrade() {
            let is_running = launcher.borrow_mut().check_if_child_is_running();
            app.invoke_set_child_process_state(is_running);
        }
    });

    child_poll_timer
}
