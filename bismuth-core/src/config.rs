use serde::Deserialize;
use std::{fs, path::Path};
use toml;

use bismuth_tui::prompt::{builtin::YesNo, Input};

pub const CONFIG_FILE: &str = include_str!("../config.toml");

pub fn check_for_config(dir: &Path) -> bool {
    let file_path = dir.join("bismuth.toml");
    file_path.exists()
}

pub fn make_config(dir: &Path) -> Result<(), std::io::Error> {
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
    std: bool,
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
            text_2: Some(self.text_2.unwrap_or(String::from("#a89984"))),
            link: Some(self.link.unwrap_or(String::from("#fe8018"))),
            link_hover: Some(self.link_hover.unwrap_or(String::from("#d65d0e"))),
        }
    }
}

#[allow(dead_code)]
#[derive(Deserialize, Debug, PartialEq, Default)]
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

#[derive(Debug)]
pub struct Config<'a> {
    pub name: String,
    pub theme: Theme,
    pub addons: Addons,
    pub directory: &'a Path,
    pub bstd: bool,
}

macro_rules! replace_css {
    ($css:expr, $theme:expr, $replace:tt) => {
        $css.replace(
            &format!("{{{}}}", stringify!($replace)),
            &$theme.$replace.clone().unwrap(),
        )
    };
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

        Config {
            directory: dir,
            name: toml_config.website.name,
            addons: toml_config.addons.unwrap_or_default(),
            theme: toml_config.theme.unwrap_or_default(),
            bstd: toml_config.website.std,
        }
    }

    fn new_toml_config(content: &str) -> TomlConfig {
        let mut config: TomlConfig = toml::from_str(content).unwrap();
        config.theme = Some(config.theme.unwrap_or_default().fill_default());
        config
    }

    pub fn gen_colors(&self) -> String {
        let base_css = r":root {
    --background-1: {background_1};
    --background-2: {background_2};
    --background-3: {background_3};
    --text-1: {text_1};
    --text-2: {text_2};
    --link: {link};
    --link-hover: {link_hover};
}";
        let base_css = replace_css!(base_css, self.theme, background_1);
        let base_css = replace_css!(base_css, self.theme, background_2);
        let base_css = replace_css!(base_css, self.theme, background_3);
        let base_css = replace_css!(base_css, self.theme, text_1);
        let base_css = replace_css!(base_css, self.theme, text_2);
        let base_css = replace_css!(base_css, self.theme, link);
        replace_css!(base_css, self.theme, link_hover)
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
std = true
"#;

        let theme = Theme::default().fill_default();
        let expected = TomlConfig {
            website: WebsiteConfig {
                name: String::from("test"),
                std: true,
            },
            theme: Some(theme),
            ..Default::default()
        };
        let result = Config::new_toml_config(&content);
        assert_eq!(expected, result)
    }

    #[test]
    fn simple_config_2() {
        let content = r####"
[website]
name = "test"
std = true

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
                std: true,
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

    #[test]
    fn gen_css() {
        let content = r#"
[website]
name = "test"
std = true
"#;

        let toml = Config::new_toml_config(&content);
        let theme = toml.theme.unwrap_or_default().fill_default();
        let path = Path::new("./");
        let result = Config {
            name: String::new(),
            theme,
            addons: Default::default(),
            directory: &path,
            bstd: true,
        }
        .gen_colors();

        let expected = String::from(
            r":root {
    --background-1: #282828;
    --background-2: #3c3836;
    --background-3: #1d2021;
    --text-1: #ebdbb2;
    --text-2: #a89984;
    --link: #fe8018;
    --link-hover: #d65d0e;
}",
        );
        assert_eq!(expected, result)
    }
}
