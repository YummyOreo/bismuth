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
    fn render<T: Render + Clone>(&mut self, content: &[T]) -> String;
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

    pub fn element_to_htmlelm(&mut self, element: Element) -> HtmlElement {
        let mut inside = element
            .elements
            .iter()
            .map(|e| self.element_to_htmlelm(e.clone()))
            .collect::<Vec<HtmlElement>>();
        match element.kind {
            Kind::Paragraph => HtmlElement::new(ElementKind::Paragraph, inside),
            Kind::Text => HtmlElement::new(
                ElementKind::Text {
                    text: element.text.unwrap_or_default(),
                },
                inside,
            ),
            Kind::Link => HtmlElement::new(
                ElementKind::Link {
                    text: element.text.clone().unwrap_or_default(),
                    link: element.get_attr("link").cloned().unwrap_or_default(),
                },
                inside,
            ),
            Kind::Bold => HtmlElement::new(ElementKind::Bold, inside),
            Kind::Header => HtmlElement::new(
                ElementKind::Header {
                    level: element
                        .get_attr("level")
                        .cloned()
                        .unwrap_or_default()
                        .parse::<i8>()
                        .unwrap_or(1),
                },
                inside,
            ),
            _ => HtmlElement::new(
                ElementKind::Text {
                    text: "".to_string(),
                },
                inside,
            ),
        }
    }

    pub fn render_htmlelm(&mut self, element: HtmlElement) {}
}

impl Render for Renderer {
    fn render<T: Render + Clone>(&mut self, _: &[T]) -> String {
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
