use std::rc::Rc;

use slint::VecModel;

mod config;
mod gamepad_manager;
mod launcher;

slint::include_modules!();

pub const CONFIG_FILE_NAME: &str = "gpcl.toml";

fn main() {
    let model = Rc::new(VecModel::from(vec![GamepadModel {
        name: "PS5 Controller".into(),
        status: "Charging".into(),
        charge: 42,
    }]));

    let window = MainWindow::new().unwrap();

    window.set_model(model.clone().into());

    model.push(GamepadModel {
        name: "PS5 Controller".into(),
        status: "Discharging".into(),
        charge: 33,
    });

    model.push(GamepadModel {
        name: "PS5 Controller".into(),
        status: "Wired".into(),
        charge: 33,
    });

    model.push(GamepadModel {
        name: "PS5 Controller".into(),
        status: "Unknown".into(),
        charge: 33,
    });

    window.run().unwrap();
}
