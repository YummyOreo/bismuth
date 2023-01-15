#![allow(dead_code)]
use std::fs;
use std::path::PathBuf;

pub mod load;

#[derive(Debug)]
pub struct MarkdownFile {
    pub content: String,
    path: PathBuf,
}

#[derive(Debug)]
pub enum MarkdownFileError {
    IsFileError(String),
    NotMarkdownError(String),
    ErrorLoadingFile(String),

    NotDirectoryError(String),
}

impl MarkdownFile {
    pub fn load_file(path: &PathBuf) -> Result<Self, MarkdownFileError> {
        if !path.is_file() {
            return Err(MarkdownFileError::IsFileError(
                path.to_string_lossy().to_string(),
            ));
        }

        if let Some(s) = path.extension() {
            let s = s.to_ascii_lowercase();
            let s = s.to_str().expect("Should be string");

            if matches!(s, "md" | "markdown") {
                return Ok(MarkdownFile {
                    path: path.to_path_buf(),
                    content: fs::read_to_string(path).expect("file should be there"),
                });
            }
        }
        return Err(MarkdownFileError::NotMarkdownError(
            path.to_string_lossy().to_string(),
        ));
    }
}

// # Tests:

#[cfg(test)]
mod test {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_load() {
        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path("./testdata/output/");
        let snapshot =
            MarkdownFile::load_file(&PathBuf::from("./testdata/tests/test.markdown")).unwrap();
        settings.bind(|| {
            insta::assert_snapshot!(format!("{:?}", snapshot));
        });
    }

    #[test]
    fn test_load_1() {
        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path("./testdata/output/");
        let snapshot =
            MarkdownFile::load_file(&PathBuf::from("./testdata/tests/test1.MARKDOWN")).unwrap();
        settings.bind(|| {
            insta::assert_snapshot!(format!("{:?}", snapshot));
        });
    }

    #[test]
    fn test_load_2() {
        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path("./testdata/output/");
        let snapshot =
            MarkdownFile::load_file(&PathBuf::from("./testdata/tests/test3.md")).unwrap();
        settings.bind(|| {
            insta::assert_snapshot!(format!("{:?}", snapshot));
        });
    }

    #[test]
    fn test_load_3() {
        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path("./testdata/output/");
        let snapshot =
            MarkdownFile::load_file(&PathBuf::from("./testdata/tests/TEST4.MD")).unwrap();
        settings.bind(|| {
            insta::assert_snapshot!(format!("{:?}", snapshot));
        });
    }
}
