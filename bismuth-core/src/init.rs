use std::fs::{create_dir_all, write};
use std::path::PathBuf;

const SRC_DIR: &str = "src/";
const ASSETS_DIR: &str = "assets/";

pub fn init_folder(name: &str) -> Result<(), std::io::Error> {
    let dir = format!("./{name}/");
    let src = PathBuf::from(format!("{dir}{SRC_DIR}"));
    let assets = PathBuf::from(format!("{dir}{ASSETS_DIR}"));

    let index_file = src.join("index.md");
    let config_file = PathBuf::from(format!("{dir}/bismuth.toml"));

    create_dir_all(src)?;
    create_dir_all(assets)?;
    write(index_file, "# This is the entry for your website!")?;
    write(config_file, include_str!("../config.toml"))?;
    Ok(())
}
