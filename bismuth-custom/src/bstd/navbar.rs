use crate::plugin::Plugin;
use bismuth_parser::tree::{Element, Kind};
use std::collections::HashMap;

pub const NAME: &str = "navbar";

pub const WRAPPER_NAME: &str = "bismuth_navbar_wrapper";
pub const WRAPPER: &str = include_str!("../../data/navbar_wrapper.html");

pub const ITEM_NAME: &str = "bismuth_navbar_item";
pub const ITEM: &str = include_str!("../../data/navbar_item.html");

#[derive(Debug)]
pub struct PageInfo<'a> {
    pub dir: &'a String,
    pub title: &'a String,
    pub order: i32,
    pub is_current: bool,
}

#[derive(Debug, Default)]
pub struct Navbar {
    pub values: HashMap<String, String>,
    pub path: String,
    pub id: u32,
}

impl Navbar {
    fn get_pages<'a>(
        &self,
        page: &'a bismuth_parser::Parser,
        pages: &[Option<&'a bismuth_parser::Parser>],
    ) -> Vec<&'a bismuth_parser::Parser> {
        let mut output_files = vec![];
        for file in pages {
            if file.is_some() {
                let file = file.unwrap();
                if file
                    .metadata
                    .frontmatter
                    .get_value("navbar_include")
                    .cloned()
                    .unwrap_or_default()
                    .to_lowercase()
                    == "true"
                {
                    output_files.push(file);
                }
            }
        }
        if self
            .values
            .get("navbar_include")
            .cloned()
            .unwrap_or_default()
            .to_lowercase()
            == "true"
        {
            output_files.push(page);
        }
        output_files
    }

    fn get_info<'a>(&self, pages: &[&'a bismuth_parser::Parser]) -> Vec<PageInfo<'a>> {
        let mut info: Vec<PageInfo<'a>> = vec![];
        for page in pages {
            let frontmatter = &page.metadata.frontmatter;
            let is_current = frontmatter.get_path().cloned().unwrap() == self.path;

            let order = frontmatter
                .get_value("navbar_order")
                .unwrap()
                .parse::<i32>()
                .unwrap();

            let title = frontmatter.get_value("navbar_title").unwrap_or(
                frontmatter
                    .get_value("title")
                    .unwrap_or(frontmatter.get_title().unwrap()),
            );
            let path = frontmatter.get_path().unwrap();
            info.push(PageInfo {
                title,
                dir: path,
                order,
                is_current,
            });
        }
        info
    }
}

impl Plugin for Navbar {
    fn pre_load(&mut self, page: &bismuth_parser::Parser, custom: &crate::Custom) {
        self.values = custom.data.clone();
        self.path = page.metadata.frontmatter.get_path().cloned().unwrap();
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
