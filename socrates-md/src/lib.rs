#![allow(dead_code)]
use std::fs;
use std::path::Path;

pub mod load;

#[derive(Debug)]
pub struct MarkdownFile<'a> {
    pub content: String,
    path: &'a Path,
}

#[derive(Debug)]
pub enum MarkdownFileError<'a> {
    IsFileError(&'a Path),
    NotMarkdownError(&'a Path),
    ErrorLoadingFile(&'a Path),
}

impl<'a> MarkdownFile<'a> {
    pub fn load_file(path: &'a Path) -> Result<Self, MarkdownFileError> {
        if !path.is_file() {
            return Err(MarkdownFileError::IsFileError(path));
        }

        if let Some(s) = path.extension() {
            let s = s.to_ascii_lowercase();
            let s = s.to_str().expect("Should be string");

            if matches!(s, "md" | "markdown") {
                return Ok(MarkdownFile {
                    path,
                    content: fs::read_to_string(path).expect("file should be there"),
                });
            }
        }
        return Err(MarkdownFileError::NotMarkdownError(path));
    }
}

#[test]
fn test_md_file() {
    insta::assert_snapshot!(MarkdownFile::load_file(Path::new("../docs/example/post.md"))
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
