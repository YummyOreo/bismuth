#![allow(dead_code)]
use socrates_lexer::Lexer;
use std::path::{Path, PathBuf};

mod fontmatter;
mod item;
use crate::{
    fontmatter::FontMatter,
    item::{Item, ItemKind},
};

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

    current_item: Item,

    metadata: Metadata,

    pub ast: Vec<Item>,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        let metadata = Metadata::new(&lexer.file.path);
        Parser {
            lexer,

            current_token_index: 0,

            current_item: Item {
                kind: ItemKind::Root,
                children: vec![],
            },

            metadata,

            ast: vec![],
        }
    }
}
