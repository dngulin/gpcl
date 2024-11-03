use serde_derive::Deserialize;

#[derive(Default, Deserialize)]
pub struct Config {
    pub layout: Option<LayoutConfig>,
    pub style: Option<StyleConfig>,

    #[serde(default)]
    pub items: Vec<AppIconConfig>,
}

#[derive(Default, Deserialize)]
pub struct LayoutConfig {
    pub top_panel_height: Option<f32>,
    pub clock_height: Option<f32>,
    pub icon_size: Option<f32>,
}

#[derive(Default, Deserialize)]
pub struct StyleConfig {
    pub font: Option<String>,
    pub font_weight: Option<i32>,
}

#[derive(Deserialize)]
pub struct AppIconConfig {
    pub name: String,
    pub icon: String,
    pub exec: String,
}
