pub mod navbar;
pub mod footer;
pub mod bloglist;

use crate::{template::Template, plugin::Plugin};

pub fn match_template(name: &str) -> Option<Template> {
    match name {
        navbar::NAME => {},
        footer::NAME => {},
        bloglist::NAME1 | bloglist::NAME2 | bloglist::NAME3 => {},
        _ => {}
    }
    None
}

pub fn match_plugin(name: &str) -> Option<Plugin> {
    match name {
        navbar::NAME => {},
        footer::NAME => {},
        bloglist::NAME1 | bloglist::NAME2 | bloglist::NAME3 => {},
        _ => {}
    }
    None
}
