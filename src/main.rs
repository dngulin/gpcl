mod config;
mod qml_types;

pub use config::*;
use cstr::cstr;
use qmetaobject::{qml_register_type, qrc, QmlEngine};
use qml_types::*;

qrc!(load_resources, "res" as "" {"main.qml", "bg.svg"});

pub const CONFIG_FILE_NAME: &str = "gpcl.toml";

fn main() {
    qmetaobject::init_qt_to_rust();
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .init();

    let import_uri = cstr!("GpcLauncherTypes");
    qml_register_type::<LauncherApp>(import_uri, 1, 0, cstr!("LauncherApp"));

    load_resources();

    let mut engine = QmlEngine::new();
    engine.load_file("qrc:/main.qml".into());
    engine.exec();
}
