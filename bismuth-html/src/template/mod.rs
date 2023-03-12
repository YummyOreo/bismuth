#![allow(unused, dead_code)]
use bismuth_parser::{
    custom::CustomElm,
    tree::{Element, Kind},
    Parser,
};
use regex::Regex;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::path::PathBuf;

use crate::render::{Render, Renderer};

pub mod builtin;

/// Data/Values: Will be replaced (as a string) at {key}
/// Reserved keys: `body` and `elements` (Sometimes title if a full page template)
///
/// Body: Will be replaced at {body}
/// Elements: Will be replaced at {elements}
#[derive(Debug, PartialEq)]
pub struct Template<'a> {
    template: &'a str,
    values: &'a HashMap<String, String>,
    body: Option<&'a String>,
    pub elements: &'a Vec<Element>,
    pub asset_list: Vec<PathBuf>,
}

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
                    body: c.body.as_ref(),
                    elements: &elm.elements,
                    asset_list: vec![],
                });
            }
        }
        Err(())
    }
}

impl<'a> Template<'a> {
    pub fn new(
        template_str: &'a str,
        values: &'a HashMap<String, String>,
        body: Option<&'a String>,
        elements: &'a Vec<Element>,
    ) -> Self {
        Self {
            template: template_str,
            values,
            body,
            elements,
            asset_list: vec![],
        }
    }

    pub fn get_template(name: &str) -> Option<&str> {
        match name.to_lowercase().as_str() {
            "test" => Some(builtin::TEST),
            "default" => Some(builtin::DEFAULT),
            _ => None,
        }
    }

    pub fn new_from_name(
        name: &'a str,
        values: &'a HashMap<String, String>,
        body: Option<&'a String>,
        elements: &'a Vec<Element>,
    ) -> Option<Self> {
        let template_str = Self::get_template(name)?;

        Some(Self::new(template_str, values, body, elements))
    }
}

impl Render for Template<'_> {
    fn render(&mut self, path: &PathBuf) -> Option<String> {
        let mut output = self.template.to_string();
        // First replace {elements} w/ rendered elements
        let mut elements_str = self
            .elements
            .iter()
            .map(|e| {
                let mut element = e.clone();
                let html = element.render(path).expect("Should not fail");
                self.asset_list.append(&mut element.asset_list);
                html
            })
            .collect::<String>();
        let e_rg = Regex::new(r"\{(?i)elements\}").expect("Should be valid regex");
        output = e_rg.replace(&output, elements_str).to_string();

        // Next do the body
        let b_rg = Regex::new(r"\{(?i)body\}").expect("Should be valid regex");
        let body_default = String::new();
        output = b_rg
            .replace(&output, self.body.cloned().unwrap_or_default())
            .to_string();

        // next do each value
        for (key, value) in self.values {
            let rg = Regex::new(&format!(r"\{{(?i){}\}}", key)).expect("Should be valid");
            output = rg.replace(&output, value).to_string();
        }
        Some(output)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use bismuth_parser::Parser;

    fn init_parser<'a>(content: &'a str, frontmatter: &'a str) -> Parser {
        let mut parser = Parser::new_test("/test/", content);
        parser
            .metadata
            .frontmatter
            .update_from_str(frontmatter)
            .unwrap();

        parser.parse();
        parser
    }

    macro_rules! init_template {
        ($parser:tt, $template:expr, $body:expr) => {
            Template {
                template: $template,
                body: $body,
                elements: &$parser.ast.elements,
                values: &$parser.metadata.frontmatter.get_values().unwrap(),
                asset_list: vec![],
            }
        };
    }

    macro_rules! init_template_name {
        ($parser:tt, $name:expr, $body:expr) => {
            Template {
                template: Template::get_template(&$name).unwrap(),
                body: $body,
                elements: &$parser.ast.elements,
                values: &$parser.metadata.frontmatter.get_values().unwrap(),
                asset_list: vec![],
            }
        };
    }

    macro_rules! snapshot {
        ($content:tt) => {
            let mut settings = insta::Settings::clone_current();
            settings.set_snapshot_path("../../testdata/output/template/");
            settings.bind(|| {
                insta::assert_snapshot!($content);
            });
        };
    }

    #[test]
    fn test() {
        let parser = init_parser(
            "{value} {another_value}",
            "
            values:
                - value: test
                - another_value: test another value
            ",
        );
        let mut template = init_template!(parser, &String::from("test:\n {elements}"), None);

        let s = template.render(&PathBuf::new()).unwrap();
        snapshot!(s);
    }

    #[test]
    fn test_1() {
        let parser = init_parser(
            "{value}} {hmm}",
            "
            values:
                - value}: test
                - another_value: test another value
            ",
        );
        let mut template = init_template!(parser, &String::from("test:\n {elements}"), None);

        let s = template.render(&PathBuf::new()).unwrap();
        snapshot!(s);
    }

    #[test]
    fn test_2() {
        let parser = init_parser(
            "*test*",
            "
            kind: test
            values:
                - value_1: test
                - value_2: test another value
            ",
        );
        let name = parser.metadata.frontmatter.get_kind().unwrap();
        let mut template = init_template_name!(parser, name, None);

        let s = template.render(&PathBuf::new()).unwrap();
        snapshot!(s);
    }
}
