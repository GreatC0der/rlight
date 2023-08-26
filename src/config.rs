use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub camera: u8,
    pub delay: u64,
    pub set_brightness_cmd: String,
    pub get_brightness_cmd: String,
    pub light_sensetivity: f32,
    pub mid_sensetivity: f32,
    pub dark_sensetivity: f32,
    pub adaptive_sensetivity: bool,
    pub learning_coefficient: f32,
    pub step: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            camera: 0,
            delay: 60,
            set_brightness_cmd: String::from("xbacklight -set"),
            get_brightness_cmd: String::from("xbacklight -get"),
            light_sensetivity: 0.4,
            mid_sensetivity: 0.4,
            dark_sensetivity: 0.4,
            adaptive_sensetivity: true,
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
