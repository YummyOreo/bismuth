use crate::plugin::Plugin;
use bismuth_parser::tree::{Element, Kind};
use std::collections::HashMap;

pub const NAME: &str = "navbar";

pub const WRAPPER_NAME: &str = "bismuth_navbar_wrapper";
pub const WRAPPER: &str = include_str!("../../data/navbar_wrapper.html");

pub const ITEM_NAME: &str = "bismuth_navbar_item";
pub const ITEM: &str = include_str!("../../data/navbar_item.html");

#[derive(Debug, Default)]
pub struct Navbar {
    pub values: HashMap<String, String>,
    pub id: u32,
}

impl Plugin for Navbar {
    fn pre_load(&mut self, _: &bismuth_parser::Parser, custom: &crate::Custom) {
        self.values = custom.data.clone();
        self.id = custom.id;
    }

    fn run(&mut self, target: &mut bismuth_parser::Parser, _: &[Option<&bismuth_parser::Parser>]) {
        let mod_element = target.ast.find_mut(self.id).unwrap();
        let mut values = self.values.iter().collect::<Vec<(&String, &String)>>();
        values.sort();
        for (key, value) in values {
            let mut element = Element::new(Kind::Text);
            element.text = Some(format!("{}: {}", key, value));
            mod_element.elements.push(element);
        }
    }
}
