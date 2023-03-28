pub mod bloglist;
pub mod footer;
pub mod navbar;

use crate::{plugin::Plugin, template::Template};

pub fn match_template(name: &str) -> Option<Template> {
    match name {
        footer::NAME => Some(Template {
            content: "<h1>Test template</h1>".to_string(),
        }),
        bloglist::BLOGITEM_NAME => Some(Template::new(bloglist::BLOGITEM.to_string())),
        bloglist::BLOGWRAPPER_NAME => Some(Template::new(bloglist::BLOGWRAPPER.to_string())),
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
