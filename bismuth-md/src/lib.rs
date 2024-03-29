use std::fs;
use std::path::{Path, PathBuf};

pub mod load;

#[derive(Debug, Clone)]
pub struct MarkdownFile {
    pub content: String,
    pub path: PathBuf,
}

#[derive(Debug)]
pub enum MarkdownFileError {
    IsFileError(String),
    NotMarkdownError(String),
    ErrorLoadingFile(String),

    NotDirectoryError(String),
}

impl MarkdownFile {
    pub fn load_file(path: &PathBuf, rel: &Path) -> Result<Self, MarkdownFileError> {
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
                    path: rel.to_path_buf(),
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

    fn snapshot(path: &str) -> String {
        let path = PathBuf::from(path);
        let mut file = MarkdownFile::load_file(&path, &path).unwrap();
        file.path = PathBuf::from(
            file.path
                .to_string_lossy()
                .replace('\\', "/")
                .to_lowercase(),
        );
        format!("{file:#?}")
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

    snapshot!(test_load_file, "./testdata/tests/test.markdown");

    snapshot!(test_load_file_1, "./testdata/tests/test1.md");
}
