mod config;
mod gamepad_manager;
mod launcher;
mod qml_bridge;

pub use config::*;
use cstr::cstr;
use gamepad_manager::{QmlGamepadManager, QmlPowerStatus};
use launcher::QmlLauncher;
use qmetaobject::{qml_register_enum, qml_register_type, qrc, QmlEngine};

qrc!(load_resources, "res" as "" {"main.qml", "bg.svg"});

pub const CONFIG_FILE_NAME: &str = "gpcl.toml";

fn main() {
    qmetaobject::init_qt_to_rust();
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .init();

    let import_uri = cstr!("gpcl");
    qml_register_type::<QmlLauncher>(import_uri, 1, 0, cstr!("Launcher"));
    qml_register_type::<QmlGamepadManager>(import_uri, 1, 0, cstr!("GamepadManager"));
    qml_register_enum::<QmlPowerStatus>(import_uri, 1, 0, cstr!("PowerStatus"));

    load_resources();

    let mut engine = QmlEngine::new();
    engine.load_file("qrc:/main.qml".into());
    engine.exec();
}
