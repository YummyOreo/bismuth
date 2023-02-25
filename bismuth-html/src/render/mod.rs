#![allow(dead_code, unused)]
use bismuth_parser::{
    error,
    tree::{Ast, Element, Kind},
    Parser,
};
use std::path::PathBuf;

mod element;
mod code;

pub trait Render {
    fn render<T: Render + Clone>(&mut self, content: &[T]) -> String;
}

#[derive(Clone)]
pub struct Renderer {
    pub parser: Parser,

    output: String,
    path: PathBuf,

    current_elements: Vec<element::HtmlElement>,
}

impl Renderer {
    pub fn new(parser: Parser) -> Self {
        let path = PathBuf::from(
            parser
                .metadata
                .frontmatter
                .get_path()
                .expect("Should have a path"),
        );
        Self {
            parser,
            output: String::new(),
            path,
            current_elements: vec![],
        }
    }
}

