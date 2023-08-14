use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub delay: u64,
    pub set_brightness_cmd: String,
    pub darkness_sensetivity: f64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            delay: 20,
            set_brightness_cmd: String::from("xbacklight -set"),
            darkness_sensetivity: 1.7,
        }
    }
}
pub fn load_config() -> Config {
    confy::load("rlight", None).expect(
        "Couldn't load a config.\n
        TIP: You might be able to fix it by deleting the existing config.",
    )
}
