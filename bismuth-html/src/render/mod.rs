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
pub struct Renderer {
    pub parser: Parser,
    pos: usize,

    output: String,
    path: PathBuf,
}

impl Renderer {
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
        }
    }
}

impl Render for Renderer {
    fn render(&mut self) -> String {
        while self.pos < self.parser.ast.elements.len() {
            let current = self.parser.ast.elements.get(self.pos);
            let mut s = String::new();
            if let Some(e) = current {
                s = e.clone().render();
                self.output.push_str(&format!("{s}\n"));
            }

            self.pos += 1;
        }
        self.output.clone()
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
            Kind::BoldItalic => (String::from("<b><i>"), String::from("</i></b>")),
            Kind::Blockquote => (String::from("<blockquote>"), String::from("</blockquote>")),
            Kind::Header => {
                let mut num = self.get_attr("level").cloned().unwrap_or(String::from("6"));
                if num.parse::<i8>().unwrap_or_default() > 6_i8 {
                    num = String::from("6");
                }
                (format!("<h{num}>"), format!("</h{num}>"))
            }
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
                            .parse::<usize>()
                            .unwrap()
                            + 1_usize
                    )
                ),
                String::from("</li>"),
            ),
            Kind::OrderedListElement => (
                format!(
                    r#"<li class="num-list">{}{}."#,
                    String::from("\t").repeat(
                        self.get_attr("level")
                            .cloned()
                            .unwrap_or(String::from("1"))
                            .parse::<usize>()
                            .unwrap()
                            + 1_usize
                    ),
                    self.get_attr("num").cloned().unwrap_or(String::from("0"))
                ),
                String::from("</li>"),
            ),

            Kind::InlineCode => (
                format!(
                    r#"<div class="inline-code">{}"#,
                    self.text.clone().unwrap_or_default()
                ),
                String::from("</div>"),
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
            Kind::EndOfLine => (String::from("<br>"), Default::default()),

            // TOOD: LATEX
            _ => Default::default(),
        };
        format!("{start}{inside}{end}")
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs;
    fn snapshot(content: &str) -> String {
        let mut parser = Parser::new_test("/test/", content);
        parser.parse();
        let mut render = Renderer::new(parser);
        render.render()
    }

    macro_rules! snapshot {
        ($name:tt, $content:tt) => {
            #[test]
            fn $name() {
                let mut settings = insta::Settings::clone_current();
                settings.set_snapshot_path("../../testdata/output/render/");
                settings.bind(|| {
                    insta::assert_snapshot!(snapshot($content));
                });
            }
        };
    }

    macro_rules! snapshot_path {
        ($name:tt, $path:tt) => {
            #[test]
            fn $name() {
                let path = PathBuf::from($path);
                println!("{path:?}");
                let content = fs::read_to_string(&path).unwrap();
                let mut settings = insta::Settings::clone_current();
                settings.set_snapshot_path("../../testdata/output/render/");
                settings.bind(|| {
                    insta::assert_snapshot!(snapshot(&content));
                });
            }
        };
    }

    snapshot!(
        test,
        "test *test* \n```rust\nfn test() {\n\tprintln!(\"test\")\n}\n```"
    );

    snapshot!(
        test_2,
        "# hearder\n- 1\n    - 2\n1. list item\nthis is a *__good test__*!! \n `inline?`\n---\n> blockquote"
    );

    snapshot!(test_br, "test test \n\n\ntest test\ntest\n");

    snapshot_path!(test_path, "./testdata/test/render/test.md");
}
