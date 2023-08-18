use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub delay: u64,
    pub set_brightness_cmd: String,
    pub get_brightness_cmd: String,
    pub sensetivity: f32,
    pub adaptive_sensetivity: bool,
    pub step: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            delay: 30,
            set_brightness_cmd: String::from("xbacklight -set"),
            get_brightness_cmd: String::from("xbacklight -get"),
            sensetivity: 0.4,
            adaptive_sensetivity: true,
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
