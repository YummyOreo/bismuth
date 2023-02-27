#![allow(dead_code, unused)]
use bismuth_parser::{
    error,
    tree::{Ast, Element, Kind},
    Parser,
};
use std::path::PathBuf;

mod code;
use crate::render::code::highlight;

pub trait Render {
    fn render(&mut self) -> String;
}

#[derive(Clone)]
pub struct Renderer<'a> {
    pub parser: Parser,
    pos: usize,

    output: String,
    path: PathBuf,

    // the start of the current line line (ie. the kind might be: Paragraph)
    head: Option<&'a Element>,
}

impl Renderer<'_> {
    pub fn new(parser: Parser) -> Self {
        let path = PathBuf::from(
            parser
                .metadata
                .frontmatter
                .get_path()
                .expect("Should have a path"),
        );
        Self {
            parser,
            pos: 0,
            output: String::new(),
            path,
            head: None,
        }
    }
}

impl Render for Renderer<'_> {
    fn render(&mut self) -> String {
        // TODO: change this to not use HtmlElement, just use parser element, no need to use the
        // other ones

        // Steps
        // 1. Construct a from each entry in self.parser.ast.elements
        // 1.1. Collapse 2 EOL to a <br>
        // 2. Render the line and append it to a string
        // 3. Return the string

        while self.pos < self.parser.ast.elements.len() {
            let current = self.parser.ast.elements.get(self.pos);

            self.pos += 1;
        }

        todo!();
    }
}

/// Will return (Url, Should be _blank)
fn parse_url(url: &str) -> (String, bool) {
    // TODO: Make it so these V (inside the website) will be moved to /assets/
    if url.starts_with('/') || url.starts_with('\\') || url.starts_with('.') {
        return (url.to_string(), false);
    }
    (url.to_string(), true)
}

impl Render for Element {
    fn render(&mut self) -> String {
        let mut inside = self
            .elements
            .iter()
            .map(|e| e.clone().render())
            .collect::<String>();

        let (start, end) = match self.kind {
            Kind::Paragraph => (String::from("<p>"), String::from("</p>")),
            Kind::Bold => (String::from("<b>"), String::from("</b>")),
            Kind::Italic => (String::from("<i>"), String::from("</i>")),
            Kind::Blockquote => (String::from("<blockquote>"), String::from("</blockquote>")),
            Kind::Text => (self.text.clone().unwrap_or_default(), Default::default()),

            Kind::Link => {
                let (url, blank) = parse_url(&self.get_attr("link").cloned().unwrap_or_default());
                let blank = {
                    if blank {
                        String::from(r#"target="blank""#)
                    } else {
                        Default::default()
                    }
                };
                (
                    format!(r#"<a target="{}" {}>"#, url, blank),
                    Default::default(),
                )
            }
            Kind::FilePrev => (
                format!(
                    r#"<img src="{}" alt="{}">"#,
                    self.get_attr("link").cloned().unwrap_or_default(),
                    self.text.clone().unwrap_or_default()
                ),
                Default::default(),
            ),

            Kind::ListItem => (
                format!(
                    r#"<li class="item">{}"#,
                    String::from("\t").repeat(
                        self.get_attr("level")
                            .cloned()
                            .unwrap_or(String::from("1"))
                            .parse()
                            .unwrap()
                    )
                ),
                String::from("</li>"),
            ),
            Kind::ListItem => (
                format!(
                    r#"<li class="num-list">{}{}."#,
                    String::from("\t").repeat(
                        self.get_attr("level")
                            .cloned()
                            .unwrap_or(String::from("1"))
                            .parse()
                            .unwrap()
                    ),
                    self.get_attr("num").cloned().unwrap_or(String::from("0"))
                ),
                String::from("</li>"),
            ),

            Kind::InlineCode => (
                highlight(
                    String::from("plaintext"),
                    self.text.clone().unwrap_or_default(),
                )
                .unwrap(),
                Default::default(),
            ),
            Kind::BlockCode => (
                highlight(
                    self.get_attr("lang")
                        .cloned()
                        .unwrap_or(String::from("plaintext")),
                    self.text.clone().unwrap_or_default(),
                )
                .unwrap(),
                Default::default(),
            ),

            Kind::HorizontalRule => (String::from("<hr>"), Default::default()),
            Kind::EndOfLine => {
                if inside.starts_with("<hr>") {
                    inside.replacen("<hr>", "", 1);
                }

                (String::from("<hr>"), Default::default())
            }

            // TOOD: LATEX
            _ => Default::default(),
        };
        format!("{start}{inside}{end}")
    }
}

#[cfg(test)]
mod test {
    use super::*;
    macro_rules! snapshot {
        ($content:tt) => {
            let mut settings = insta::Settings::clone_current();
            settings.set_snapshot_path("../../testdata/output/render/");
            settings.bind(|| {
                insta::assert_snapshot!($content);
            });
        };
    }
}
