use std::fs;
use std::path::{Path, PathBuf};

use crate::{MarkdownFile, MarkdownFileError};

pub fn load_from_dir(path: &Path, dir: &Path) -> Result<Vec<MarkdownFile>, MarkdownFileError> {
    let path_dir = &mut PathBuf::from(dir.to_string_lossy().to_string().replace('\\', "/"));

    let mut files = vec![];
    if !path.is_dir() {
        return Err(MarkdownFileError::NotDirectoryError(
            path.to_string_lossy().to_string(),
        ));
    }

    let paths = fs::read_dir(path).expect("Should be directory");
    for file in paths {
        let file_path = &mut file.unwrap().path();

        if file_path.is_dir() {
            if let Ok(mut m) = load_from_dir(file_path, dir) {
                files.append(&mut m)
            }
        } else if file_path.is_file() {
            let rel = file_path
                .to_string_lossy()
                .replace('\\', "/")
                .replace(&path_dir.to_string_lossy().to_string(), "");

            let file_rel = PathBuf::from(rel);
            if let Ok(m) = MarkdownFile::load_file(file_path, &file_rel) {
                files.push(m)
            }
        }
    }

    Ok(files)
}

// # Tests:
//
// TODO: Fix this, broken because:
// Could load in any way order, should account for this

#[cfg(test)]
mod test {
    use super::*;
    use std::path::PathBuf;

    fn snapshot(path: &str) -> String {
        let path = PathBuf::from(path).canonicalize().unwrap();
        let mut files =
            load_from_dir(&path, &PathBuf::from(&path).canonicalize().unwrap()).unwrap();
        for mut file in &mut files {
            let new_path = file
                .path
                .to_string_lossy()
                .replace('\\', "/")
                .to_lowercase();
            file.path = PathBuf::from(new_path);
        }
        files.sort_by(|a, b| a.path.cmp(&b.path));
        format!("{files:#?}")
    }

    macro_rules! snapshot {
        ($name:tt, $path:tt) => {
            #[test]
            fn $name() {
                let mut settings = insta::Settings::clone_current();
                settings.set_snapshot_path("../testdata/output/");
                settings.bind(|| {
                    insta::assert_snapshot!(snapshot($path));
                });
            }
        };
    }

    snapshot!(test_load, "./testdata/tests/");
}
