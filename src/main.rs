mod models;
mod palette_generator;
use clap::Parser;
use std::{fs, path::PathBuf};

use crate::models::config;
#[derive(Parser)]
struct Cli {
    config_path: Option<PathBuf>,
}

fn homeify(path: PathBuf) -> PathBuf {
    match path.strip_prefix("~") {
        Ok(rest) => home::home_dir().map(|home| home.join(rest)).unwrap_or(path),
        Err(_) => path,
    }
}

fn main() -> std::io::Result<()> {
    let args = Cli::parse();

    let path = homeify(
        args.config_path
            .unwrap_or_else(|| PathBuf::from("~/.config/colorinator/config.toml")),
    );

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    println!("Config: {}", path.display());
    let mut config = config::Config::parse(&path);
    Ok(())
}
