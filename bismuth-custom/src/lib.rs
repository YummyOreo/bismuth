#![allow(dead_code)]
use std::collections::HashMap;

use bismuth_parser::{
    custom::CustomElm,
    tree::{Ast, Element, Kind},
    Metadata, Parser,
};

pub mod plugin;
pub mod template;

pub struct Custom {
    name: String,
    data: HashMap<String, String>,
    template: Option<template::Template>,
    plugin: Option<plugin::Plugin>,
}

impl Custom {
    pub fn new(name: String, data: HashMap<String, String>) -> Self {
        Custom {
            name,
            data,
            template: None,
            plugin: None,
        }
    }

    pub fn from_elm(elm: &CustomElm) -> Self {
        Self::new(elm.name.clone(), elm.values.clone())
    }

    pub fn set_template(&mut self, t: template::Template) {
        self.template = Some(t)
    }

    pub fn set_plugin(&mut self, p: plugin::Plugin) {
        self.plugin = Some(p)
    }
}

fn get_customs(ast: &Ast) -> Vec<&Element> {
    let elements: Vec<&Element> = ast.elements.iter().collect();
    elements
        .iter()
        .filter_map(|&e| {
            if let Kind::CustomElement(_) = &e.kind {
                Some(e)
            } else {
                None
            }
        })
        .collect::<Vec<&Element>>()
}

pub fn parse_custom(mut target: Parser, others: Vec<&Parser>) -> Parser {
    let custom_elms = get_customs(&target.ast);
    let customs: Vec<Custom> = custom_elms
        .iter()
        .filter_map(|e| {
            if let Kind::CustomElement(c) = &e.kind {
                Some(Custom::from_elm(c))
            } else {
                None
            }
        })
        .collect();
    todo!()
}

#[cfg(test)]
mod test_utils {
    use super::*;
    use regex::Regex;

    macro_rules! snapshot {
        ($content:tt) => {
            let mut settings = insta::Settings::clone_current();
            settings.set_snapshot_path("../testdata/output/");
            settings.bind(|| {
                insta::assert_snapshot!($content);
            });
        };
    }
    #[test]
    fn get_customs_test() {
        let mut parser =
            bismuth_parser::Parser::new_test("/test/", "%{{\nname: test\nother: key\n}}");
        parser.parse().unwrap();

        let customs = format!("{:#?}", get_customs(&parser.ast));
        let re = Regex::new(r"id: \d+").unwrap();
        let customs = re.replace_all(&customs, "id: [redacted]").to_string();
        snapshot!(customs);
    }
}
