pub mod bloglist;
pub mod footer;
pub mod navbar;

use crate::{plugin::Plugin, template::Template};

pub fn match_template(name: &str) -> Option<Template> {
    match name {
        navbar::NAME => {}
        footer::NAME => {}
        bloglist::NAME1 | bloglist::NAME2 | bloglist::NAME3 => {}
        _ => {}
    }
    None
}

pub fn match_plugin(name: &str) -> Option<Box<dyn Plugin>> {
    match name {
        navbar::NAME => {}
        footer::NAME => {}
        bloglist::NAME1 | bloglist::NAME2 | bloglist::NAME3 => {}
        _ => {}
    }
    None
}
