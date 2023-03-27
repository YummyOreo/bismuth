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

#[derive(Deserialize, Default, Debug, PartialEq)]
pub struct WebsiteConfig {
    name: String,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug, PartialEq, Default)]
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
#[derive(Deserialize, Debug, PartialEq)]
pub struct Addons {
    templates: Option<String>,
    plugins: Option<String>,
}

#[allow(dead_code)]
#[derive(Deserialize, Default, Debug, PartialEq)]
pub struct TomlConfig {
    website: WebsiteConfig,
    theme: Option<Theme>,
    addons: Option<Addons>,
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
        let content = fs::read_to_string(dir.join("bismuth.toml")).unwrap();
        let toml_config = Self::new_toml_config(&content);

        // TODO: add the theme and addons
        Config {
            directory: dir,
            name: toml_config.website.name,
        }
    }

    fn new_toml_config(content: &str) -> TomlConfig {
        let mut config: TomlConfig = toml::from_str(&content).unwrap();
        if let Some(theme) = config.theme {
            config.theme = Some(theme.fill_default());
        }
        config
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn simple_config() {
        let content = r#"
[website]
name = "test"
"#;

        let expected = TomlConfig {
            website: WebsiteConfig {
                name: String::from("test"),
            },
            ..Default::default()
        };
        let result = Config::new_toml_config(&content);
        assert_eq!(expected, result)
    }

    // background_1: Option<String>,
    // background_2: Option<String>,
    // background_3: Option<String>,
    // text_1: Option<String>,
    // text_2: Option<String>,
    // link: Option<String>,
    // link_hover: Option<String>,
    #[test]
    fn simple_config_2() {
        let content = r####"
[website]
name = "test"

[theme]
background_1 = "#fefefe"
link = "#fefefe"
text_1 = "#fefefe"

"####;

        let color = String::from("#fefefe");

        let theme = Theme {
            background_1: Some(color.clone()),
            link: Some(color.clone()),
            text_1: Some(color.clone()),
            ..Default::default()
        }
        .fill_default();

        let expected = TomlConfig {
            website: WebsiteConfig {
                name: String::from("test"),
            },
            theme: Some(theme),
            ..Default::default()
        };
        let result = Config::new_toml_config(&content);
        assert_eq!(expected, result)
    }

    #[test]
    #[should_panic]
    fn simple_error() {
        // should panic bc there has to be a name
        let content = r#"
[website]
"#;

        let _ = Config::new_toml_config(&content);
    }
}
