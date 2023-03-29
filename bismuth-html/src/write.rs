use std::io::Error;
use std::path::PathBuf;
// use crate::render::Renderer;

const BUILD: &str = "./build";
const BUILD_ASSETS: &str = "./build/assets";
const BUILD_ASSETS_CSS: &str = "./build/assets/css";
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
            fs::create_dir_all(build)?
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
            fs::create_dir_all(assets)?
        }
        Ok(())
    }

    /// Makes the `./assets/` folder
    pub fn make_assets() -> Result<(), Error> {
        let assets = Path::new(ASSETS);
        if !assets.exists() {
            fs::create_dir_all(assets)?
        }
        Ok(())
    }

    pub fn make_css() -> Result<(), Error> {
        make_build_assets()?;

        let css = Path::new(BUILD_ASSETS_CSS);
        if !css.exists() {
            fs::create_dir_all(css)?
        }
        Ok(())
    }

    /// Writes a html file to the `./build/` folder
    /// The path should not have a . in the begining
    pub fn write_html_file(content: &str, path: &Path, name: &String) -> Result<(), Error> {
        // Makes build dir if it does not exitst
        make_build()?;
        let mut path_str = path.to_string_lossy().to_string();
        path_str.remove(0);
        let path = &mut PathBuf::from(path_str);

        let path_build = Path::new(BUILD);
        let path = path.join(format!("{}.html", name));
        let full_path = path_build.join(path);

        let mut dir = full_path.clone();
        dir.pop();

        fs::create_dir_all(dir)?;
        fs::write(full_path, content)
    }

    /// Moves a asset from the `./assets/` folder to the `./bulid/assets/` folder
    /// The path should not have a . in the begining
    pub fn move_asset(path: &Path) -> Result<(), Error> {
        // Makes both the `./assets/` folder and the `./build/assets/` folder if the do not exitst
        make_assets()?;
        make_build_assets()?;

        let new_path = Path::new(BUILD).join(path);
        let old_full = Path::new("./").canonicalize()?.join(path);

        fs::copy(old_full, new_path).map(|_| ())
    }

    pub fn write_css(content: &str, name: &str) -> Result<(), Error> {
        make_css()?;

        let path = PathBuf::from(format!("{BUILD_ASSETS_CSS}/{name}.css"));
        fs::write(path, content)
    }
}

pub fn move_assets(assets: &[PathBuf]) -> Result<(), Error> {
    for asset in assets {
        utils::move_asset(asset)?;
    }
    Ok(())
}
