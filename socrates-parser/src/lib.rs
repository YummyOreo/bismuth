#![allow(dead_code)]
use socrates_lexer::Lexer;

mod item;
use crate::item::{Item, ItemKind};

#[derive(Default)]
pub struct FontMatter {}

pub struct Parser {
    pub lexer: Lexer,

    // current_token: &'a Token,
    current_token_index: usize,

    current_item: Item,

    fontmatter: FontMatter,

    pub ast: Vec<Item>,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        Parser {
            lexer,

            current_token_index: 0,

            current_item: Item {
                kind: ItemKind::Root,
                children: vec![],
            },

            fontmatter: Default::default(),

            ast: vec![],
        }
    }
}
