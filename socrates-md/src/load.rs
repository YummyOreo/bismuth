use std::fs;
use std::path::PathBuf;

use crate::{MarkdownFile, MarkdownFileError};

pub fn load_from_dir(path: &PathBuf) -> Result<Vec<MarkdownFile>, MarkdownFileError> {
    println!("{:?}", path.canonicalize());
    let mut files = vec![];
    if !path.is_dir() {
        return Err(MarkdownFileError::NotDirectoryError(
            path.to_string_lossy().to_string(),
        ));
    }

    let paths = fs::read_dir(path).expect("Should be directory");
    for file in paths {
        let file_path = file.unwrap().path();
        if file_path.is_dir() {
            match load_from_dir(&file_path) {
                Ok(mut m) => files.append(&mut m),
                Err(e) => {
                    return Err(e);
                }
            }
        } else if file_path.is_file() {
            match MarkdownFile::load_file(&file_path) {
                Ok(m) => files.push(m),
                Err(e) => {
                    return Err(e);
                }
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

    #[test]
    fn test_load_files() {
        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path("./testdata/output/");
        let snapshot = load_from_dir(&PathBuf::from("./testdata/tests/")).unwrap();
        settings.bind(|| {
            insta::assert_snapshot!(format!("{:?}", snapshot));
        });
    }
}
