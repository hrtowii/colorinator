mod models;
use clap::Parser;
use home::home_dir;
use models::config;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::str::FromStr;
#[derive(Parser)]
struct Cli {
    config_path: Option<PathBuf>,
}

fn homeify(path: PathBuf) -> PathBuf {
    if let Some(mut home_dir) = home_dir() {
        if path.starts_with("~") {
            let mut meow = path.components();
            meow.next();
            home_dir.push(meow.as_path());
        }
        // dbg!(&home_dir);
        home_dir
    } else {
        path.into()
    }
}

fn main() {
    let mut args = Cli::parse();
    if args.config_path.is_none() {
        println!("defaulting to ~/.config/colorinator/config.toml");
        args.config_path = Some(homeify(
            PathBuf::from_str("~/.config/colorinator/config.toml").unwrap(),
        ));
    }
    if !Path::exists(&args.config_path.as_ref().unwrap().parent().unwrap()) {
        println!("creating file");
        let _ = fs::create_dir_all(args.config_path.unwrap().parent().unwrap());
    }
    println!("Hello, world!");
}
