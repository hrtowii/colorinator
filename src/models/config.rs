use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
#[derive(Serialize, Deserialize, Debug)]
struct Config {
    wallpaper_path: String,
    paths: Vec<String>,
}

impl Config {
    fn new() -> Config {
        return Config {
            wallpaper_path: String::from("~/.config/wallpapers/angel.png"),
            paths: vec!["skibidi".to_string()],
        };
    }

    pub fn parse(path: String) -> Config {
        let mut config_toml = String::new();

        let mut file = match File::open(&path) {
            Ok(file) => file,
            Err(_) => {
                println!("Could not find config file, using default!");
                return Config::new();
            }
        };

        fs::read_to_string(&mut config_toml)
            .unwrap_or_else(|err| panic!("Error while reading config: [{}]", err));

        match toml::from_str::<Config>(&config_toml) {
            Ok(config) => {
                println!("Successfully parsed config: {:#?}", config);
                dbg!(&config);
                config
            }
            Err(e) => {
                eprintln!("Failed to parse TOML: {}", e);
                Config::new()
            }
        }
    }
}
