use std::{
    fs,
    path::{Path, PathBuf},
};

use bismuth_tui::prompt::{builtin::YesNo, Input};

pub const CONFIG_FILE: &str = include_str!("../config.toml");

pub fn check_for_config(dir: &PathBuf) -> bool {
    let file_path = dir.join("bismuth.toml");
    file_path.exists()
}

pub fn make_config(dir: &PathBuf) -> Result<(), std::io::Error> {
    let full_dir = dir.canonicalize().unwrap();
    let name = full_dir
        .components()
        .last()
        .expect("Should have last")
        .as_os_str()
        .to_str()
        .unwrap_or_default();

    let config_file = full_dir.join("bismuth.toml");

    let config_file_contents = CONFIG_FILE.replace("{name}", name);
    fs::write(config_file, config_file_contents)
}

fn read_config() {}

pub struct Config<'a> {
    pub name: String,
    pub directory: &'a Path,
}

impl<'a> Config<'a> {
    pub fn new(dir: &'a Path) -> Self {
        let pb = dir.to_path_buf();
        if !check_for_config(&pb) {
            let mut should_make = YesNo::new(
                String::from("Would you like to create one? (Y/n): "),
                String::from("There is no bismuth.toml."),
                None,
            );
            should_make.run();
            if let Some(true) = should_make.result {
                println!("Making config file...");
                make_config(&pb).unwrap();
            } else {
                println!("Exiting...");
                std::process::exit(1);
            }
        }
        todo!()
    }
}
