use rand::Rng;
use std::collections::HashMap;

use crate::{custom, error::ElementError};

#[derive(Default, Debug)]
pub struct Ast {
    pub elements: Vec<Option<Element>>,
}

#[derive(Clone, PartialEq, Debug)]
pub enum Kind {
    Paragraph,

    Text,
    Link,
    FilePrev,

    Italic,
    Bold,

    Blockquote,

    ListItem,

    OrderedListElement,

    InlineCode,
    BlockCode,

    InlineLaTeX,
    BlockLaTeX,

    CustomElement(custom::CustomElm),

    Header,

    HorizontalRule,

    EndOfLine,
    LineBreak,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Element {
    pub kind: Kind,
    pub elements: Vec<Element>,
    pub text: Option<String>,
    pub attrs: HashMap<String, String>,

    id: u32,
}

impl Element {
    pub fn new(kind: Kind) -> Self {
        let mut rng = rand::thread_rng();

        Element {
            kind,
            elements: vec![],
            text: Default::default(),
            attrs: Default::default(),

            id: rng.gen::<u32>(),
        }
    }

    pub fn append_node(&mut self, elm: Element) -> &Element {
        self.elements.push(elm);
        self.elements.last().expect("Should be there")
    }

    pub fn add_attr<T: ToString>(&mut self, key: &str, value: &T) {
        self.attrs.insert(key.to_string(), value.to_string());
    }

    pub fn get_attr(&self, attr: &str) -> Result<&String, ElementError> {
        self.attrs
            .get(attr)
            .ok_or(ElementError::GetAttrError(attr.to_string()))
    }

    pub fn get_attr_mut(&mut self, attr: &str) -> Result<&mut String, ElementError> {
        self.attrs
            .get_mut(attr)
            .ok_or(ElementError::GetAttrError(attr.to_string()))
    }

    pub fn get_text(&self) -> Result<&String, ElementError> {
        self.text.as_ref().ok_or(ElementError::GetTextError)
    }

    pub fn get_elements(&self) -> &Vec<Element> {
        &self.elements
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }
}
