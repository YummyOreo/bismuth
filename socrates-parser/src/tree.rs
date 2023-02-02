use std::collections::HashMap;

use crate::{custom, error::ElementError};

#[derive(Default)]
pub struct Ast {
    pub elements: Vec<Option<Element>>,
}

#[derive(Clone, PartialEq)]
pub enum Kind {
    Text,
    Link,
    FilePrev,

    Italic,
    Bold,

    Blockquote,

    List,
    ListElement,

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

#[derive(Clone)]
pub struct Element {
    pub kind: Kind,
    pub elements: Vec<Option<Element>>,
    pub text: Option<String>,
    pub attrs: HashMap<String, String>,
}

impl Element {
    pub fn new(kind: Kind) -> Self {
        Element {
            kind,
            elements: vec![],
            text: Default::default(),
            attrs: Default::default(),
        }
    }

    pub fn append_node(&mut self, elm: Element) -> Option<&Element> {
        self.elements.push(Some(elm));
        self.elements.last().expect("Should be there").as_ref()
    }

    pub fn add_attr(&mut self, key: &str, value: &str) {
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

    pub fn get_elements(&self) -> &Vec<Option<Element>> {
        &self.elements
    }
}
