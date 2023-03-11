#![allow(dead_code)]
use std::{
    fs,
    io::Error,
    path::{Path, PathBuf},
};

// use crate::render::Renderer;

const BUILD: &str = "./build/";
const BUILD_ASSETS: &str = "./build/assets";
const ASSETS: &str = "./assets";

pub mod utils {
    use super::*;

    /// Makes the `./build/` folder along with the `./bulid/assets/` folder
    pub fn make_build() -> Result<(), Error> {
        let build = Path::new(BUILD);
        if !build.exists() {
            fs::create_dir_all(&build)?
        }
        make_build_assets()?;
        Ok(())
    }

    /// Makes the `./bulid/assets/` folder
    pub fn make_build_assets() -> Result<(), Error> {
        let assets = Path::new(BUILD_ASSETS);
        if !assets.exists() {
            fs::create_dir_all(&assets)?
        }
        Ok(())
    }

    /// Makes the `./assets/` folder
    pub fn make_assets() -> Result<(), Error> {
        let assets = Path::new(ASSETS);
        if !assets.exists() {
            fs::create_dir_all(&assets)?
        }
        Ok(())
    }

    /// Writes a html file to the `./build/` folder
    pub fn write_html_file(content: &str, path: &PathBuf) -> Result<(), Error> {
        // Makes build dir if it does not exitst
        make_build()?;

        let path_build = Path::new(BUILD);
        let full_path = path_build.join(path);

        fs::write(full_path, content)
    }
}
