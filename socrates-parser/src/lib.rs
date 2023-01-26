#![allow(dead_code)]
use socrates_lexer::Lexer;

mod fontmatter;
mod item;
use crate::{
    fontmatter::FontMatter,
    item::{Item, ItemKind},
};

pub struct Parser {
    pub lexer: Lexer,

    current_token_index: usize,

    current_item: Item,

    fontmatter: FontMatter,

    pub ast: Vec<Item>,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        let fontmatter = FontMatter::new(&lexer.file.path, None);
        Parser {
            lexer,

            current_token_index: 0,

            current_item: Item {
                kind: ItemKind::Root,
                children: vec![],
            },

            fontmatter,

            ast: vec![],
        }
    }
}
