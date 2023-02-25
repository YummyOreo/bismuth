#![allow(dead_code, unused)]
use bismuth_parser::{
    error,
    tree::{Ast, Element, Kind},
    Parser,
};
use std::path::PathBuf;

mod code;
mod element;

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

impl Render for Renderer {
    fn render<T: Render + Clone>(&mut self, _: &[T]) -> String {
        // Steps
        // 1. Construct a from each entry in self.parser.ast.elements
        // 1.1. Collapse 2 EOL to a <br>
        // 2. Render the line and append it to a string
        // 3. Return the string
        todo!();
    }
}
