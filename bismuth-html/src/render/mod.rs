#![allow(dead_code, unused)]
use bismuth_parser::{
    error,
    tree::{Ast, Element, Kind},
    Parser,
};
use std::path::PathBuf;

mod element;

pub trait Render {
    fn render(&mut self) -> String;
}

pub struct Renderer {
    pub parser: Parser,

    output: String,
    path: PathBuf,

    /// The element to the right is inside of the element to the left,
    /// if there is no element to the left, it is the "root" element
    current_elements: Vec<element::HtmlElement>,
}

impl Renderer {
    pub fn new(parser: Parser) -> Self {
        let path = PathBuf::from(
            parser
                .metadata
                .fontmatter
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
