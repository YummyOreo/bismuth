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

#[test]
fn test_loading() {
    let content: String = {
        let files = load_from_dir(&PathBuf::from("../docs/example/")).unwrap();
        let mut s = "".to_string();

        for file in files {
            s.push_str(file.path.to_str().unwrap());
            s.push('\n');
            s.push_str(&file.content);
            s.push_str("\n\n");
        }
        s
    };
    insta::assert_snapshot!(&content, @r###"
    ../docs/example/blog-list.md
    ---
    title: "Blogs"
    type: "list"
    ---

    # Blog Posts:

    ```list
    directory: "./posts/"
    ```


    ../docs/example/post.md
    ---
    title: "This is a test post"
    description: "Lorem ipsum dolor sit amet, qui minim labore adipisicing minim sint cillum sint consectetur cupidatat."
    type: "article"
    ---

    # This is a test post
    Lorem ipsum dolor sit amet, officia excepteur ex fugiat reprehenderit enim labore culpa sint ad nisi Lorem pariatur mollit ex esse exercitation amet. Nisi anim cupidatat excepteur officia. Reprehenderit nostrud nostrud ipsum Lorem est aliquip amet voluptate voluptate dolor minim nulla est proident. Nostrud officia pariatur ut officia. Sit irure elit esse ea nulla sunt ex occaecat reprehenderit commodo officia dolor Lorem duis laboris cupidatat officia voluptate. Culpa proident adipisicing id nulla nisi laboris ex in Lorem sunt duis officia eiusmod. Aliqua reprehenderit commodo ex non excepteur duis sunt velit enim. Voluptate laboris sint cupidatat ullamco ut ea consectetur et est culpa et culpa duis.


    "###);
}
