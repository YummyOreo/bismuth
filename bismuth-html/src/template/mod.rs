#![allow(unused, dead_code)]
use bismuth_parser::{
    custom::CustomElm,
    tree::{Element, Kind},
};
use std::collections::HashMap;

use crate::render::Render;

/// Data/Values: Will be replaced (as a string) at {key}
/// Reserved keys: `body` and `elements`
///
/// Body: Will be replaced at {body}
/// Elements: Will be replaced at {elements}
pub struct Template<'a> {
    values: &'a HashMap<String, String>,
    body: &'a Option<String>,
    elements: &'a Vec<Element>,
}
// ^ should all be references to stuff

impl<'a> Template<'a> {
    pub fn new(elm: &'a Element, c: &'a CustomElm) -> Self {
        Self {
            values: &c.values,
            body: &c.body,
            elements: &elm.elements,
        }
    }
}

impl Render for Template<'_> {
    fn render(&mut self) -> String {
        todo!()
    }
}
