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

#[test]
fn test_md_file() {
    insta::assert_snapshot!(MarkdownFile::load_file(&PathBuf::from("../docs/example/post.md"))
        .unwrap()
        .content
        .as_str(),
        @r###"
    ---
    title: "This is a test post"
    description: "Lorem ipsum dolor sit amet, qui minim labore adipisicing minim sint cillum sint consectetur cupidatat."
    type: "article"
    ---

    # This is a test post
    Lorem ipsum dolor sit amet, officia excepteur ex fugiat reprehenderit enim labore culpa sint ad nisi Lorem pariatur mollit ex esse exercitation amet. Nisi anim cupidatat excepteur officia. Reprehenderit nostrud nostrud ipsum Lorem est aliquip amet voluptate voluptate dolor minim nulla est proident. Nostrud officia pariatur ut officia. Sit irure elit esse ea nulla sunt ex occaecat reprehenderit commodo officia dolor Lorem duis laboris cupidatat officia voluptate. Culpa proident adipisicing id nulla nisi laboris ex in Lorem sunt duis officia eiusmod. Aliqua reprehenderit commodo ex non excepteur duis sunt velit enim. Voluptate laboris sint cupidatat ullamco ut ea consectetur et est culpa et culpa duis.
    "###
    );
}
