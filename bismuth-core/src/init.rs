use std::fs::create_dir_all;
use std::path::PathBuf;

const SRC_DIR: &str = "str/";
const ASSETS_DIR: &str = "assets/";

pub fn init_folder(name: &str) -> Result<(), std::io::Error> {
    let dir = format!("./{name}/");
    let src = PathBuf::from(format!("{dir}{SRC_DIR}"));
    let assets = PathBuf::from(format!("{dir}{ASSETS_DIR}"));

    create_dir_all(src)?;
    create_dir_all(assets)?;
    Ok(())
}
