#![allow(dead_code)]
use crate::plugin::Plugin;
use bismuth_parser::custom::CustomElm;
use bismuth_parser::tree::{Element, Kind};
use std::collections::HashMap;

pub const NAME1: &str = "blog list";
pub const NAME2: &str = "blogs";
pub const NAME3: &str = "bloglist";

pub const ITEM_NAME: &str = "builtin_blogitem";
pub const ITEM: &str = include_str!("../../data/blog_item.html");

pub const WRAPPER_NAME: &str = "builtin_blog_wrapper";
pub const WRAPPER: &str = include_str!("../../data/blog_wrapper.html");

#[derive(Debug, Default)]
pub struct BlogList {
    pub values: HashMap<String, String>,
    pub dir: String,
    pub id: u32,
}

impl BlogList {
    fn get_posts<'a>(
        &self,
        files: &[Option<&'a bismuth_parser::Parser>],
    ) -> Vec<&'a bismuth_parser::Parser> {
        let mut output_files = vec![];
        for file in files {
            if file.is_some() {
                let file = file.unwrap();
                let file_path = file
                    .metadata
                    .frontmatter
                    .get_path()
                    .cloned()
                    .unwrap_or_default();
                if file_path == self.dir
                    || format!("{file_path}/") == self.dir
                    || format!("/{file_path}") == self.dir
                {
                    output_files.push(file);
                }
            }
        }
        output_files
    }

    fn gen_templates(&self, files: &[&bismuth_parser::Parser]) -> Vec<Element> {
        let mut customs = vec![];
        for file in files {
            // get info
            let frontmatter = &file.metadata.frontmatter;
            let html_title = frontmatter.get_title().unwrap();
            let path = frontmatter.get_path().unwrap();
            let title = frontmatter.get_value("title").unwrap();
            let date = frontmatter.get_value("date").unwrap();

            let full_path = format!("{path}/{html_title}.html");

            // make custom
            let mut custom = CustomElm::new();
            custom.name = ITEM_NAME.to_string();
            custom.values.insert("title".to_string(), title.to_string());
            custom.values.insert("date".to_string(), date.to_string());
            custom
                .values
                .insert("url".to_string(), full_path.to_string());

            let element = Element::new(Kind::CustomElement(custom));

            customs.push(element);
        }
        customs
    }
}

impl Plugin for BlogList {
    fn pre_load(&mut self, _: &bismuth_parser::Parser, custom: &crate::Custom) {
        self.values = custom.data.clone();
        self.dir = self.values.get("dir").cloned().unwrap_or_default();
        self.id = custom.id;
    }

    fn run(
        &mut self,
        target: &mut bismuth_parser::Parser,
        files: &[Option<&bismuth_parser::Parser>],
    ) {
        let mod_element = target.ast.find_mut(self.id).unwrap();

        let posts = self.get_posts(files);
        let mut customs = self.gen_templates(&posts);

        let mut wrapper = CustomElm::new();
        wrapper.name = WRAPPER_NAME.to_string();
        let mut wrapper_elm = Element::new(Kind::CustomElement(wrapper));
        wrapper_elm.elements.append(&mut customs);

        mod_element.elements.push(wrapper_elm);
    }
}
