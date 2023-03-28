#![allow(dead_code)]
use crate::plugin::Plugin;
use bismuth_parser::custom::CustomElm;
use bismuth_parser::tree::{Element, Kind};
use std::collections::HashMap;

pub const NAME1: &str = "blog list";
pub const NAME2: &str = "blogs";
pub const NAME3: &str = "bloglist";

pub const BLOGITEM_NAME: &str = "builtin_blogitem";
pub const BLOGITEM: &str = include_str!("../../data/blogitem.html");

pub const BLOGWRAPPER_NAME: &str = "builtin_blogwrapper";
pub const BLOGWRAPPER: &str = include_str!("../../data/blogwrapper.html");

#[derive(Debug, Default)]
pub struct BlogList {
    pub values: HashMap<String, String>,
    pub dir: String,
    pub id: u32,
}

impl Plugin for BlogList {
    fn pre_load(&mut self, _: &bismuth_parser::Parser, custom: &crate::Custom) {
        self.values = custom.data.clone();
        self.dir = self.values.get("dir").cloned().unwrap_or_default();
        self.id = custom.id;
    }

    fn run(&mut self, target: &mut bismuth_parser::Parser, _: &[Option<&bismuth_parser::Parser>]) {
        let mod_element = target.ast.find_mut(self.id).unwrap();
        let mut values = self.values.iter().collect::<Vec<(&String, &String)>>();
        values.sort();
        for (key, value) in values {
            let mut custom = CustomElm::new();
            custom.name = "navbar".to_string();
            custom.values.insert(key.to_string(), value.to_string());
            let mut element = Element::new(Kind::CustomElement(custom));

            element.text = Some(format!("{}: {}", key, value));
            mod_element.elements.push(element);
        }
    }
}
