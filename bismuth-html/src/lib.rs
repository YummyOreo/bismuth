use bismuth_parser::Parser;
use std::path::PathBuf;

mod render;
mod template;
mod utils;
pub mod write;

use crate::render::Render;

// Expose a api to just render a parser
// + Make something to render a list of parsers concurrently

pub fn render_one(parser: Parser) -> Option<String> {
    let mut renderer = render::Renderer::new(parser);
    renderer.render(&PathBuf::new())
}
pub fn render_list() {
    todo!()
}
