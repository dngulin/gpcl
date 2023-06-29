use crate::config::AppIconConfig;
use crate::AppIconModel;

use slint::Image;
use std::path::Path;

pub struct Executable {
    pub program: String,
    pub args: Vec<String>,
}

impl Executable {
    fn new(path: &String) -> Self {
        let mut tokens = path.split_whitespace();

        let program = match tokens.next() {
            Some(program) => program.into(),
            None => {
                log::error!("Failed to parse program name from: {}", path);
                String::default()
            }
        };

        let args = tokens.map(|str| str.to_string()).collect();

        Self { program, args }
    }
}

pub fn config_entry_into_item(config: &AppIconConfig) -> (AppIconModel, Executable) {
    let image = match Image::load_from_path(Path::new(config.icon.as_str())) {
        Ok(image) => image,
        Err(_) => {
            log::error!("Filed to load image: `{}`", config.icon);
            Image::default()
        }
    };
    let name = (&config.name).into();

    let model = AppIconModel { image, name };
    let executable = Executable::new(&config.exec);

    (model, executable)
}
