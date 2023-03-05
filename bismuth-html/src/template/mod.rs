#![allow(unused, dead_code)]
use bismuth_parser::{
    custom::CustomElm,
    tree::{Element, Kind},
};
use std::collections::HashMap;
use std::convert::TryFrom;

use crate::render::Render;

/// Data/Values: Will be replaced (as a string) at {key}
/// Reserved keys: `body` and `elements`
///
/// Body: Will be replaced at {body}
/// Elements: Will be replaced at {elements}
#[derive(Debug, PartialEq)]
pub struct Template<'a> {
    template: &'a String,
    values: &'a HashMap<String, String>,
    body: &'a Option<String>,
    elements: &'a Vec<Element>,
}
// ^ should all be references to stuff

impl<'a> TryFrom<&'a Element> for Template<'a> {
    type Error = ();

    /// Try to convert a element that's kind == CustomElement to a Template
    fn try_from(elm: &'a Element) -> Result<Self, Self::Error> {
        if let Kind::CustomElement(c) = &elm.kind {
            if let Some(t) = &c.template {
                return Ok(Self {
                    template: t,
                    values: &c.values,
                    body: &c.body,
                    elements: &elm.elements,
                });
            }
        }
        Err(())
    }
}

impl Render for Template<'_> {
    fn render(&mut self) -> String {
        todo!()
    }
}
