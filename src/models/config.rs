use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    wallpaper_path: String,
    paths: Vec<String>,
}

impl Config {
    pub fn new() -> Config {
        Config {
            wallpaper_path: "~/.config/wallpapers/angel.png".to_string(),
            paths: vec!["~/.config/sway/config".to_string()],
        }
    }

    pub fn parse(path: &Path) -> Config {
        let config_toml = match fs::read_to_string(path) {
            Ok(contents) => contents,
            Err(err) => {
                eprintln!("{err}");
                std::process::exit(1);
            }
        };

        match toml::from_str::<Config>(&config_toml) {
            Ok(config) => config,
            Err(err) => {
                eprintln!("Failed to parse TOML: {err}. Using defaults!");
                std::process::exit(1);
            }
        }
    }
}
