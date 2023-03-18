use bismuth_parser::Parser;
use std::path::PathBuf;

mod render;
mod template;
pub mod write;

pub use crate::render::{Render, Renderer};

// Expose a api to just render a parser
// + Make something to render a list of parsers concurrently

pub fn render_one(parser: Parser) -> Option<String> {
    let mut renderer = render::Renderer::new(parser);
    renderer.render(&PathBuf::new())
}
pub fn render_list(parsers: Vec<Parser>) -> Vec<render::Renderer> {
    parsers
        .iter()
        .map(|p| {
            let mut renderer = render::Renderer::new(p.clone());
            let _ = renderer.render(&PathBuf::new());
            renderer
        })
        .collect::<Vec<render::Renderer>>()
}
