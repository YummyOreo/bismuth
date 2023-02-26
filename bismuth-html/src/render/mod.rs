#![allow(dead_code, unused)]
use bismuth_parser::{
    error,
    tree::{Ast, Element, Kind},
    Parser,
};
use std::path::PathBuf;

mod code;
mod element;
use crate::render::element::{ElementKind, HtmlElement};

pub trait Render {
    fn render(&mut self) -> String;
}

#[derive(Clone)]
pub struct Renderer {
    pub parser: Parser,
    pos: usize,

    output: String,
    path: PathBuf,

    current_line: Vec<HtmlElement>,
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
            pos: 0,
            output: String::new(),
            path,
            current_line: vec![],
        }
    }
}

impl Render for Renderer {
    fn render(&mut self) -> String {
        // TODO: change this to not use HtmlElement, just use parser element, no need to use the
        // other ones

        // Steps
        // 1. Construct a from each entry in self.parser.ast.elements
        // 1.1. Collapse 2 EOL to a <br>
        // 2. Render the line and append it to a string
        // 3. Return the string

        while self.pos < self.parser.ast.elements.len() {
            let current = self.parser.ast.elements.get(self.pos);

            self.pos += 1;
        }

        todo!();
    }
}

impl Render for Element {
    fn render(&mut self) -> String {
        let inside = self
            .elements
            .iter()
            .map(|e| e.clone().render())
            .collect::<String>();

        let (start, end) = match self.kind {
            _ => ("".to_string(), "".to_string()),
        };
        format!("{start}{inside}{end}")
    }
}
