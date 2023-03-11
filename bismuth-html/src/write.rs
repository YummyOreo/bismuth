#![allow(dead_code)]
// use crate::render::Renderer;

const BUILD: &str = "./build/";
const BUILD_ASSETS: &str = "./build/assets";
const ASSETS: &str = "./assets";

pub mod utils {
    use super::*;
    use std::{
        fs,
        io::Error,
        path::{Path, PathBuf},
    };

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
    /// Will make the `./build/` folder if it does not exitst
    /// But if it does not, you *should* call `make_build()`
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
    /// The path should not have a . in the begining
    pub fn write_html_file(content: &str, path: &PathBuf) -> Result<(), Error> {
        // Makes build dir if it does not exitst
        make_build()?;

        let path_build = Path::new(BUILD);
        let full_path = path_build.join(path);

        fs::write(full_path, content)
    }

    /// Moves a asset from the `./assets/` folder to the `./bulid/assets/` folder
    /// The path should not have a . in the begining
    pub fn move_asset(path: &PathBuf) -> Result<(), Error> {
        // Makes both the `./assets/` folder and the `./build/assets/` folder if the do not exitst
        make_assets()?;
        make_build_assets()?;

        let new_path = Path::new(BUILD).join(&path);
        let old_full = Path::new("./").canonicalize()?.join(&path);

        fs::copy(old_full, new_path).map(|_| ())
    }
}
