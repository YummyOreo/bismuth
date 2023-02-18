use crate::plugin::Plugin;
use bismuth_parser::custom::CustomElm;
use bismuth_parser::tree::{Element, Kind};
use std::collections::HashMap;

pub const NAME1: &str = "blog list";
pub const NAME2: &str = "blogs";
pub const NAME3: &str = "bloglist";

#[derive(Debug)]
pub struct BlogList {
    pub values: HashMap<String, String>,
    pub id: u32,
}

impl Plugin for BlogList {
    fn pre_load(&mut self, _: &bismuth_parser::Parser, custom: &crate::Custom) {
        self.values = custom.data.clone();
        self.id = custom.id;
    }

    fn run(&mut self, target: &mut bismuth_parser::Parser, _: &[&bismuth_parser::Parser]) {
        println!("{:#?}", self);
        let mut mod_element = target.ast.find_mut(self.id).unwrap();
        for (key, value) in self.values.iter() {
            let mut custom = CustomElm::new();
            custom.name = "navbar".to_string();
            custom.values.insert(key.to_string(), value.to_string());
            let mut element = Element::new(Kind::CustomElement(custom));

            element.text = Some(format!("{}: {}", key, value));
            mod_element.elements.push(element);
        }
    }
}
