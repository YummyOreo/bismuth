use std::collections::HashMap;

use crate::{custom, error::ElementError};

#[derive(Default)]
pub struct Ast {
    elements: Vec<Node>,
}

pub enum Node {
    Text(Element),
    Link(Element),
    FilePrev(Element),

    Italic(Element),
    Bold(Element),

    Blockquote(Element),

    List(Element),
    ListElement(Element, i8),

    OrderedListElement(Element, i8, u32),

    InlineCode(Element),
    BlockCode(Element),

    InlineLaTeX(Element),
    BlockLaTeX(Element),

    CustomElement(custom::CustomElm),

    Header(Element, i8),

    HorizontalRule,

    EndOfLine,
    LineBreak,
}

#[derive(Default)]
pub struct Element {
    elements: Vec<Node>,
    text: Option<String>,
    attrs: HashMap<String, String>,
}

impl Element {
    pub fn new() -> Self {
        Element {
            ..Default::default()
        }
    }

    pub fn append_node(&mut self, node: Node) -> &Node {
        self.elements.push(node);
        self.elements.last().expect("Should be there")
    }

    pub fn get_attr(&self, attr: &str) -> Result<&String, ElementError> {
        self.attrs
            .get(attr)
            .ok_or(ElementError::GetAttrError(attr.to_string()))
    }

    pub fn get_text(&self) -> Result<&String, ElementError> {
        self.text.as_ref().ok_or(ElementError::GetTextError)
    }

    pub fn get_elements(&self) -> &Vec<Node> {
        &self.elements
    }
}
