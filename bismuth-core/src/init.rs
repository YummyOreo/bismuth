use std::fs::{create_dir_all, write};
use std::path::PathBuf;

use crate::config::CONFIG_FILE;

const SRC_DIR: &str = "src/";
const ASSETS_DIR: &str = "assets/";

pub fn init_folder(name: &str) -> Result<(), std::io::Error> {
    println!("Creating project...");

    let dir = format!("./{name}/");
    let src = PathBuf::from(format!("{dir}{SRC_DIR}"));
    let assets = PathBuf::from(format!("{dir}{ASSETS_DIR}"));

    let index_file = src.join("index.md");
    let config_file = PathBuf::from(format!("{dir}/bismuth.toml"));

    create_dir_all(src)?;
    create_dir_all(assets)?;
    write(index_file, "# This is the entry for your website!")?;

    let config_file_contents = CONFIG_FILE.replace("{name}", name);
    write(config_file, config_file_contents)?;

    println!("Project created in `./{name}/`");
    Ok(())
}
