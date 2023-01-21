use std::fs;
use std::path::PathBuf;

use crate::{MarkdownFile, MarkdownFileError};

pub fn load_from_dir(path: &PathBuf) -> Result<Vec<MarkdownFile>, MarkdownFileError> {
    let mut files = vec![];
    if !path.is_dir() {
        return Err(MarkdownFileError::NotDirectoryError(
            path.to_string_lossy().to_string(),
        ));
    }

    let paths = fs::read_dir(path).expect("Should be directory");
    for file in paths {
        let file_path = file.unwrap().path();
        let rel = file_path
            .to_string_lossy()
            .replace(&path.to_string_lossy().to_string(), ".");
        let file_rel = PathBuf::from(rel);
        if file_path.is_dir() {
            if let Ok(mut m) = load_from_dir(&file_path) {
                files.append(&mut m)
            }
        } else if file_path.is_file() {
            if let Ok(m) = MarkdownFile::load_file(&file_path, &file_rel) {
                files.push(m)
            }
        }
    }

    Ok(files)
}

// # Tests:

#[cfg(test)]
mod test {
    use super::*;
    use std::path::PathBuf;

    fn snapshot(path: &str) -> String {
        let path = PathBuf::from(path).canonicalize().unwrap();
        println!("{:?}", path);
        format!("{:#?}", load_from_dir(&path).unwrap())
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
