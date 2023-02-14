#![allow(dead_code, unused)]
use std::collections::HashMap;

use bismuth_parser::{
    custom::CustomElm,
    tree::{Ast, Element, Kind},
    Metadata, Parser,
};

mod builtin;
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

    fn find_plugin(&mut self) -> Option<plugin::Plugin> {
        // REDO THIS WHEN YOU IMPLEMENT PLUGINS
        match self.name.to_string().as_str() {
            "blog list" | "blogs" | "bloglist" => {}
            "navbar" => {}
            "footer" => {}
            _ => {}
        }
        None
    }
    fn find_template(&mut self) -> Option<template::Template> {
        // REDO THIS WHEN YOU IMPLEMENT TEMPLATE
        match self.name.to_string().as_str() {
            "blog list" | "blogs" | "bloglist" => {}
            "navbar" => {}
            "footer" => {}
            _ => {}
        }
        None
    }

    pub fn find(&mut self) {
        self.plugin = self.find_plugin();
        self.template = self.find_template();
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
                let mut custom = Custom::from_elm(c);
                custom.find();
                Some(custom)
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
