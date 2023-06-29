use serde_derive::Deserialize;

#[derive(Default, Deserialize)]
pub struct Config {
    pub items: Vec<AppIconConfig>,
}

#[derive(Deserialize)]
pub struct AppIconConfig {
    pub name: String,
    pub icon: String,
    pub exec: String,
}
