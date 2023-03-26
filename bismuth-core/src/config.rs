use serde::Deserialize;
use std::{
    fs,
    path::{Path, PathBuf},
};
use toml;

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

#[derive(Deserialize)]
pub struct WebsiteConfig {
    name: String,
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct Theme {
    background_1: Option<String>,
    background_2: Option<String>,
    background_3: Option<String>,
    text_1: Option<String>,
    text_2: Option<String>,
    link: Option<String>,
    link_hover: Option<String>,
}

impl Theme {
    pub fn fill_default(self) -> Self {
        Self {
            background_1: Some(self.background_1.unwrap_or(String::from("#282828"))),
            background_2: Some(self.background_2.unwrap_or(String::from("#3c3836"))),
            background_3: Some(self.background_3.unwrap_or(String::from("#1d2021"))),
            text_1: Some(self.text_1.unwrap_or(String::from("#ebdbb2"))),
            text_2: Some(self.text_2.unwrap_or(String::from("#d5c4a1"))),
            link: Some(self.link.unwrap_or(String::from("#fe8018"))),
            link_hover: Some(self.link_hover.unwrap_or(String::from("#d65d0e"))),
        }
    }
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct Addons {
    templates: Option<String>,
    plugins: Option<String>,
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct TomlConfig {
    website: WebsiteConfig,
    theme: Option<Theme>,
    addons: Option<Addons>,
}

fn read_config(path: &PathBuf) -> Result<TomlConfig, std::io::Error> {
    let content = fs::read_to_string(path)?;
    let config: TomlConfig = toml::from_str(&content).unwrap();
    Ok(config)
}

pub struct Config<'a> {
    pub name: String,
    pub directory: &'a Path,
}

impl<'a> Config<'a> {
    pub fn new(dir: &'a Path) -> Self {
        let pb = dir.to_path_buf();
        if !check_for_config(&pb) {
            let mut should_make = YesNo::new(
                String::from("bismuth.toml does not exist. Would you like to create one? (Y/n): "),
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

        let toml_config = read_config(&dir.join("bismuth.toml")).unwrap();

        Config {
            directory: dir,
            name: toml_config.website.name,
        }
    }
}
