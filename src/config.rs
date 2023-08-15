use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub delay: u64,
    pub set_brightness_cmd: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            delay: 30,
            set_brightness_cmd: String::from("xbacklight -set"),
        }
    }
}
pub fn load_config() -> Config {
    confy::load("rlight", None).expect(
        "Couldn't load a config.\n
        TIP: You might be able to fix it by deleting the existing config.",
    )
}
