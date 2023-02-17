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

#[derive(Debug)]
pub struct Custom {
    name: String,
    id: u32,
    data: HashMap<String, String>,
    template: Option<template::Template>,
    plugin: Option<Box<dyn plugin::Plugin>>,
}

impl Custom {
    pub fn new(name: String, data: HashMap<String, String>, id: u32) -> Self {
        Custom {
            name,
            id,
            data,
            template: None,
            plugin: None,
        }
    }

    pub fn from_elm(elm: &CustomElm, id: u32) -> Self {
        Self::new(elm.name.clone(), elm.values.clone(), id)
    }

    fn find_plugin(&mut self) -> Option<Box<dyn plugin::Plugin>> {
        // REDO THIS WHEN YOU IMPLEMENT PLUGINS
        builtin::match_plugin(&self.name)
    }

    fn find_template(&mut self) -> Option<template::Template> {
        // REDO THIS WHEN YOU IMPLEMENT TEMPLATE
        builtin::match_template(&self.name)
    }

    fn pre_load(&mut self, target: &Parser) {
        if let Some(mut p) = self.plugin.take() {
            p.pre_load(target, self);
            self.plugin = Some(p);
        }
    }

    fn run(&mut self, target: &mut Parser, others: &[&Parser]) {
        if let Some(mut p) = self.plugin.take() {
            p.run(target, others);
            self.plugin = Some(p);
        }
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
    let mut customs: Vec<Custom> = custom_elms
        .iter()
        .filter_map(|e| {
            if let Kind::CustomElement(c) = &e.kind {
                let mut custom = Custom::from_elm(c, e.get_id());
                custom.find();
                custom.pre_load(&target);
                Some(custom)
            } else {
                None
            }
        })
        .collect();
    println!("{customs:#?}");
    for custom in &mut customs {
        custom.run(&mut target, &others);
    }
    target
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

    #[test]
    fn test() {
        let mut parser =
            bismuth_parser::Parser::new_test("/test/", "%{{\nname: navbar\nother: key\n}}");
        parser.parse().unwrap();

        let customs = format!("{:#?}", parse_custom(parser, vec![]));
        let re = Regex::new(r"id: \d+").unwrap();
        let customs = re.replace_all(&customs, "id: [redacted]").to_string();
        // panic!("");
        snapshot!(customs);
    }
}
