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

    pub fn insert_template(&self, target: &mut Parser) {
        if let Some(t) = &self.template {
            let id = self.id;
            let mut elm = target.ast.find_mut(id).expect("Should be there");

            if let Kind::CustomElement(c) = &mut elm.kind {
                c.template = Some(t.content.clone())
            }
        }
    }

    pub fn find(&mut self) {
        self.plugin = self.find_plugin();
        self.template = self.find_template();
    }
}

fn get_customs(elements: Vec<Element>, mut current_list: Vec<Element>) -> Vec<Element> {
    for element in elements {
        if let Kind::CustomElement(_) = element.kind {
            current_list.push(element.clone());
        }
        if !element.elements.is_empty() {
            current_list = get_customs(element.elements, current_list);
        }
    }
    current_list
}

fn run_customs(target: &mut Parser, others: &[&Parser], custom_elms: &[Element]) {
    let mut customs: Vec<Custom> = custom_elms
        .iter()
        .filter_map(|e| {
            if let Kind::CustomElement(c) = &e.kind {
                let mut custom = Custom::from_elm(c, e.get_id());
                custom.find();
                custom.pre_load(target);
                Some(custom)
            } else {
                None
            }
        })
        .collect();

    for custom in &mut customs {
        custom.run(target, others);
        custom.insert_template(target);
    }
}

pub fn parse_custom(mut target: Parser, others: &[&Parser]) -> Parser {
    let mut old_elms: Vec<Element> = vec![];
    let mut i = 0;

    loop {
        i += 1;
        let mut new_elms = get_customs(target.ast.elements.clone(), vec![]);

        if new_elms
            .iter()
            .all(|a| old_elms.iter().any(|b| b.get_id() == a.get_id()))
            || i == 5
        {
            break;
        }

        let mut run_elms = new_elms.clone();
        run_elms.retain(|e| !old_elms.iter().any(|b| b.get_id() == e.get_id()));
        run_customs(&mut target, others, &run_elms);

        old_elms = new_elms;
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
            settings.set_snapshot_path("../testdata/output/utils/");
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

        let customs = format!("{:#?}", get_customs(parser.ast.elements, vec![]));
        let re = Regex::new(r"id: \d+").unwrap();
        let customs = re.replace_all(&customs, "id: [redacted]").to_string();
        snapshot!(customs);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use regex::Regex;

    fn snapshot(content: &str) -> String {
        let mut parser = bismuth_parser::Parser::new_test("/test/", content);
        match parser.parse() {
            Ok(_) => {}
            Err(e) => panic!("{e}"),
        }

        let customs = format!("{:#?}", parse_custom(parser, &[]));
        let re = Regex::new(r"id: \d+").unwrap();
        re.replace_all(&customs, "id: [redacted]").to_string()
    }

    macro_rules! snapshot {
        ($name:tt, $content:tt) => {
            #[test]
            fn $name() {
                let mut settings = insta::Settings::clone_current();
                settings.set_snapshot_path("../testdata/output/");
                settings.bind(|| {
                    insta::assert_snapshot!(snapshot($content));
                });
            }
        };
    }

    snapshot!(test_plugin, "%{{\nname: navbar\nother: key\n}}");
    snapshot!(test_plugin_2, "%{{\nname: bloglist\nother: key\n}}");
    snapshot!(test_plugin_3, "this is a test \n%{{\nname: navbar\nkey: value\nbloglist: true? will this work\n}}\n%{{\nname: blog list\nother: key\n}}");

    snapshot!(test_template, "%{{\nname: footer\n}}");
    snapshot!(
        test_template_2,
        "%{{\nname: footer\n}}\n, this is a test \n%{{\nname: footer\nkey: value\n}}"
    );
}
