use serde_derive::Deserialize;

#[derive(Default, Deserialize)]
pub struct Config {
    pub items: Vec<AppItem>,
}

#[derive(Deserialize)]
pub struct AppItem {
    pub name: String,
    pub icon: String,
    pub exec: String,
}
