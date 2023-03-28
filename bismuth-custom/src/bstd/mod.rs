pub mod bloglist;
pub mod footer;
pub mod navbar;

use crate::{plugin::Plugin, template::Template};

pub fn match_template(name: &str) -> Option<Template> {
    match name {
        footer::NAME => Some(Template {
            content: "<h1>Test template</h1>".to_string(),
        }),

        // Wrapper
        bloglist::ITEM_NAME => Some(Template::new(bloglist::ITEM.to_string().replace('\r', ""))),
        bloglist::WRAPPER_NAME => Some(Template::new(
            bloglist::WRAPPER.to_string().replace('\r', ""),
        )),

        // Navbar
        navbar::ITEM_NAME => Some(Template::new(navbar::ITEM.to_string().replace('\r', ""))),
        navbar::WRAPPER_NAME => Some(Template::new(navbar::WRAPPER.to_string().replace('\r', ""))),
        _ => None,
    }
}

pub fn match_plugin(name: &str) -> Option<Box<dyn Plugin>> {
    match name {
        #[allow(clippy::box_default)]
        navbar::NAME => Some(Box::new(navbar::Navbar::default())),

        #[allow(clippy::box_default)]
        bloglist::NAME1 | bloglist::NAME2 | bloglist::NAME3 => {
            Some(Box::new(bloglist::BlogList::default()))
        }
        _ => None,
    }
}
