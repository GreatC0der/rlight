use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub delay: u64,
    pub set_brightness_cmd: String,
    pub get_brightness_cmd: String,
    pub light_sensitivity: f32,
    pub mid_sensitivity: f32,
    pub dark_sensitivity: f32,
    pub adaptive_sensitivity: bool,
    pub learning_coefficient: f32,
    pub step: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            delay: 30,
            set_brightness_cmd: String::from("xbacklight -set"),
            get_brightness_cmd: String::from("xbacklight -get"),
            light_sensitivity: 0.4,
            mid_sensitivity: 0.4,
            dark_sensitivity: 0.4,
            adaptive_sensitivity: true,
            learning_coefficient: 0.003,
            step: 64,
        }
    }
}

impl Config {
    pub fn save(&self) {
        confy::store("rlight", None, self).expect("Couldn't save the config.");
    }
}

pub fn load_config() -> Config {
    confy::load("rlight", None).expect(
        "Couldn't load a config.\n
        TIP: You might be able to fix it by deleting the existing config.",
    )
}
