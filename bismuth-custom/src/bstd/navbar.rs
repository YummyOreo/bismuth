use bismuth_parser::{
    custom::CustomElm,
    tree::{Element, Kind},
    Parser,
};
use std::collections::HashMap;

use crate::plugin::Plugin;

pub const NAME: &str = "navbar";

pub const WRAPPER_NAME: &str = "bismuth_navbar_wrapper";
pub const WRAPPER: &str = include_str!("../../data/navbar_wrapper.html");

pub const ITEM_NAME: &str = "bismuth_navbar_item";
pub const ITEM: &str = include_str!("../../data/navbar_item.html");

#[derive(Debug)]
pub struct PageInfo<'a> {
    pub path: String,
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
    fn get_pages<'a>(&self, page: &'a Parser, pages: &[Option<&'a Parser>]) -> Vec<&'a Parser> {
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

    fn get_info<'a>(&self, pages: &[&'a Parser]) -> Vec<PageInfo<'a>> {
        let mut info: Vec<PageInfo<'a>> = vec![];
        for page in pages {
            let frontmatter = &page.metadata.frontmatter;
            let is_current = frontmatter.get_path().cloned().unwrap() != self.path;

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

            let mut path = frontmatter.get_path().unwrap().clone();
            if path.starts_with('/') {
                path.remove(0);
            }
            let path = format!("{}/{}.html", path, frontmatter.get_title().unwrap());
            info.push(PageInfo {
                title,
                path,
                order,
                is_current,
            });
        }
        info.sort_by(|a, b| a.order.cmp(&b.order));
        info
    }

    fn gen_elements<'a>(&self, pages: &'a [PageInfo<'a>]) -> Vec<Element> {
        let customs = pages
            .iter()
            .map(|page| {
                let title = page.title.clone();
                let url = page.path.clone();
                let enabled = page.is_current.clone().to_string();

                let mut custom = CustomElm::new();
                custom.name = String::from(ITEM_NAME);
                custom.values.insert(String::from("title"), title);
                custom.values.insert(String::from("enabled"), enabled);
                custom.values.insert(String::from("url"), url);

                Element::new(Kind::CustomElement(custom))
            })
            .collect::<Vec<Element>>();
        customs
    }
}

impl Plugin for Navbar {
    fn pre_load(&mut self, page: &Parser, custom: &crate::Custom) {
        self.values = page.metadata.frontmatter.get_values().unwrap();
        self.path = page.metadata.frontmatter.get_path().cloned().unwrap();
        self.id = custom.id;
    }

    fn run(&mut self, target: &mut Parser, pages: &[Option<&Parser>]) {
        let page = target.clone();
        let mod_element = target.ast.find_mut(self.id).unwrap();

        let pages = self.get_pages(&page, pages);
        let infos = self.get_info(&pages);
        let mut elements = self.gen_elements(&infos);

        let mut wrapper = CustomElm::new();
        wrapper.name = WRAPPER_NAME.to_string();
        let mut wrapper_elm = Element::new(Kind::CustomElement(wrapper));
        wrapper_elm.elements.append(&mut elements);

        mod_element.elements.push(wrapper_elm);
    }
}
