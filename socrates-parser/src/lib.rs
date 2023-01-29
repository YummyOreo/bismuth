#![allow(dead_code)]
use socrates_lexer::Lexer;
use std::path::{Path, PathBuf};

mod fontmatter;
mod tree;
mod custom;
use crate::{fontmatter::FontMatter, tree::Ast};

#[derive(Default)]
pub struct Metadata {
    absolute_path: PathBuf,
    fontmatter: FontMatter,
}

impl Metadata {
    pub fn new(path: &Path) -> Self {
        Metadata {
            absolute_path: path.to_path_buf(),
            fontmatter: FontMatter::new(path),
        }
    }
}

pub struct Parser {
    pub lexer: Lexer,

    current_token_index: usize,

    metadata: Metadata,

    pub ast: Ast,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        let metadata = Metadata::new(&lexer.file.path);
        Parser {
            lexer,

            current_token_index: 0,

            metadata,

            ast: Default::default(),
        }
    }
}
