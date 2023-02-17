use rand::Rng;
use std::collections::HashMap;

use crate::{custom, error::ElementError};

#[derive(Default, Debug)]
pub struct Ast {
    pub elements: Vec<Element>,
}

impl Ast {
    pub fn find(&self, id: u32) -> Option<&Element> {
        for element in &self.elements {
            if let Some(elm) = Self::find_in_element(element, id) {
                return Some(elm);
            }
        }
        None
    }

    fn find_in_element(elm: &Element, id: u32) -> Option<&Element> {
        if elm.id == id {
            return Some(elm);
        } else {
            for element in &elm.elements {
                if let Some(elm) = Self::find_in_element(element, id) {
                    return Some(elm);
                }
            }
        }
        None
    }

    pub fn find_mut(&mut self, id: u32) -> Option<&mut Element> {
        for element in &mut self.elements {
            if let Some(elm) = Self::find_in_element_mut(element, id) {
                return Some(elm);
            }
        }
        None
    }

    fn find_in_element_mut(elm: &mut Element, id: u32) -> Option<&mut Element> {
        if elm.id == id {
            return Some(elm);
        } else {
            for element in &mut elm.elements {
                if let Some(elm) = Self::find_in_element_mut(element, id) {
                    return Some(elm);
                }
            }
        }
        None
    }
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn find() {
        let mut element1 = Element::new(Kind::Paragraph);
        let mut element2 = Element::new(Kind::Header);
        element2.id = 1;
        element1.id = 10;
        element1.elements = vec![Element::new(Kind::Text), element2.clone()];
        let mut ast = Ast {
            elements: vec![Element::new(Kind::Text), Element::new(Kind::Text), element1],
        };

        assert_eq!(&mut element2, ast.find_mut(1).unwrap());
        assert_eq!(&element2, ast.find(1).unwrap());
    }
}
