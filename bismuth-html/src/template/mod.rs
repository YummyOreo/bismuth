#![allow(unused, dead_code)]
use bismuth_parser::{
    custom::CustomElm,
    tree::{Element, Kind},
};
use regex::Regex;
use std::collections::HashMap;
use std::convert::TryFrom;

use crate::render::Render;

pub mod builtin;

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
    pub elements: &'a Vec<Element>,
}
// ^ should all be references to stuff

impl<'a> TryFrom<&'a Element> for Template<'a> {
    type Error = ();

    /// Try to convert a element that's kind == CustomElement to a Template
    /// If it is a custom element, it will also try to get the template
    /// If it can't get it, it will return a Error
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

impl<'a> Template<'a> {
    pub fn new(
        template_str: &'a String,
        values: &'a HashMap<String, String>,
        body: &'a Option<String>,
        elements: &'a Vec<Element>,
    ) -> Self {
        Self {
            template: template_str,
            values,
            body,
            elements,
        }
    }
}

impl Render for Template<'_> {
    fn render(&mut self) -> String {
        let mut output = self.template.clone();
        // First replace {elements} w/ rendered elements
        let mut elements_str = self
            .elements
            .iter()
            .map(|e| e.clone().render())
            .collect::<String>();
        let e_rg = Regex::new(r"\{(?i)elements\}").expect("Should be valid regex");
        output = e_rg.replace(&output, elements_str).to_string();

        // Next do the body
        let b_rg = Regex::new(r"\{(?i)body\}").expect("Should be valid regex");
        let body_default = String::new();
        output = b_rg
            .replace(&output, self.body.clone().unwrap_or_default())
            .to_string();

        // next do each value
        for (key, value) in self.values {
            let rg = Regex::new(&format!(r"\{{(?i){}\}}", key)).expect("Should be valid");
            output = rg.replace(&output, value).to_string();
        }
        output
    }
}
