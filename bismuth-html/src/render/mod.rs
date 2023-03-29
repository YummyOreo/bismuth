use bismuth_parser::{
    tree::{Element, Kind},
    Parser,
};
use regex::Regex;
use std::convert::TryFrom;
use std::path::{Path, PathBuf};

mod code;
use crate::render::code::highlight;
use crate::template::Template;
use crate::write::{move_assets, utils::write_html_file};

const URL_CHECK: &str =
    r"^(http(s)://.)?[-a-zA-Z0-9@:%._\+~#=]{2,256}\.[a-z]{2,6}\b([-a-zA-Z0-9@:%_\+.~#?&//=]*)$";

pub trait Render {
    fn render(&mut self, path: &Path) -> Option<String>;
}

#[derive(Clone, Debug)]
pub struct Renderer {
    pub parser: Parser,
    pub asset_list: Vec<PathBuf>,

    output: String,

    /// This is the path that the file will be placed to
    /// Ie `.../output/test/index.html`
    path: PathBuf,
}

impl Renderer {
    /// You should pass parser through bismuth_custom::parse_custom() first
    /// Then put the output into this
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
            asset_list: vec![],
            output: String::new(),
            path,
        }
    }

    /// Will move the assets if self.output has stuff in it, and if the asset_list is not empty
    /// Will return Ok(true) if it attempted to move it
    /// Will return Ok(false) if self.output is empty
    pub fn move_assets(&self) -> Result<bool, std::io::Error> {
        if self.output.is_empty() {
            return Ok(false);
        } else if self.asset_list.is_empty() {
            return Ok(true);
        }
        move_assets(&self.asset_list)?;
        Ok(true)
    }

    pub fn write(&self) -> Result<(), std::io::Error> {
        self.move_assets()?;
        write_html_file(
            &self.output,
            &self.path,
            &self
                .parser
                .metadata
                .frontmatter
                .get_title()
                .cloned()
                .unwrap(),
        )
    }
}

/// This will set self.output for you
/// asset_list will be populated with the assets that are needed to be moved
impl Render for Renderer {
    fn render(&mut self, _path: &Path) -> Option<String> {
        let kind = self.parser.metadata.frontmatter.get_kind()?;

        let mut values = self
            .parser
            .metadata
            .frontmatter
            .get_values()
            .unwrap_or_default();
        if let Some(title) = self.parser.metadata.frontmatter.get_title().cloned() {
            values.insert(String::from("title"), title);
        }

        let elements = &self.parser.ast.elements;
        let mut template = Template::new_from_name(kind, &values, None, elements)?;

        self.output = template.render(&self.path)?;

        // --- HORRIBLE PLS REPLACE WITH GOOD STUFF ---
        // replace all double br's + w/ <double br>
        let replace_double_br = Regex::new(r"(<br>\n*){2}").unwrap();
        self.output = replace_double_br
            .replace_all(&self.output, "<double br>\n")
            .to_string();

        // Remove all other br's
        let remove_br = Regex::new(r"<br>\n?").unwrap();
        self.output = remove_br.replace_all(&self.output, "\n").to_string();

        // Replace all double br's w/ single br's
        let replace_br = Regex::new(r"<double br>\n").unwrap();
        self.output = replace_br.replace_all(&self.output, "<br>\n").to_string();

        self.asset_list.append(&mut template.asset_list);
        Some(self.output.clone())
    }
}

/// Returns (Html, File to move)
fn handle_file_url(url: &str, text: &str, _path: &Path) -> (String, Option<PathBuf>) {
    let valid_url = Regex::new(URL_CHECK).expect("Should be valid regex");

    if valid_url.is_match(url) {
        return (format!(r#"<img src="{url}" alt="{text}">"#), None);
    } else {
        let picture_rg = Regex::new(r"^.+\.(png|jpeg|apng|avif|gif|jpg|jfif|pjpeg|pjp|svg|webp)$")
            .expect("Should be valid regex");
        let video_rg = Regex::new(r"^.+\.(webm|mp4)$").expect("Should be valid regex");

        if picture_rg.is_match(url) {
            return (
                format!(r#"<img src="{url}" alt="{text}">"#),
                Some(PathBuf::from(url)),
            );
        } else if video_rg.is_match(url) {
            let format = video_rg
                .captures_iter(url)
                .next()
                .expect("Should have 2 capture groups")
                .get(1)
                .expect("Should have 1 capture")
                .as_str();
            return (
                format!(r#"<source src="{url}" type="video/{format}">"#),
                Some(PathBuf::from(url)),
            );
        }
    }
    Default::default()
}

/// This will be relitive to the base dir
fn handle_link(url: &str, text: &str) -> (String, String) {
    let valid_url = Regex::new(URL_CHECK).expect("Should be valid regex");

    if valid_url.is_match(url) {
        (
            format!(r#"<a href="{}" target="_blank">{}"#, url, text),
            r"</a>".to_string(),
        )
    } else {
        (
            format!(r#"<a href="{}">{}"#, url, text),
            r"</a>".to_string(),
        )
    }
}

impl Render for Element {
    fn render(&mut self, path: &Path) -> Option<String> {
        let mut inside = self
            .elements
            .iter()
            .map(|e| e.clone().render(path).expect("This should not fail"))
            .collect::<String>();

        // Gets the html of the kind. Some kinds (like Text) may not have a end
        let (start, end) = match &self.kind {
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
                let rg = regex::Regex::new(r"\s+").unwrap();
                let pre_text = html_escape::encode_text(inside.trim());
                let text = rg.replace_all(&pre_text, "-").to_lowercase();
                (
                    format!(r##"<h{num} id="{text}"><a href="#{text}">"##),
                    format!("</a></h{num}>"),
                )
            }
            Kind::Text => (self.text.clone().unwrap_or_default(), Default::default()),

            Kind::Link => handle_link(
                &self.get_attr("link").cloned().unwrap_or_default(),
                &self.get_text().cloned().unwrap_or_default(),
            ),
            Kind::FilePrev => {
                let (html, asset) = handle_file_url(
                    &self.get_attr("link").cloned().unwrap_or_default(),
                    &self.text.clone().unwrap_or_default(),
                    path,
                );
                if let Some(asset) = asset {
                    self.asset_list.push(asset);
                }
                (html, Default::default())
            }

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
            Kind::EndOfLine => (String::from("\n<br>\n"), Default::default()),

            Kind::InlineLaTeX => (
                katex::render_with_opts(
                    &self.get_text().cloned().unwrap_or_default(),
                    katex::Opts::builder()
                        .output_type(katex::OutputType::Mathml)
                        .build()
                        .unwrap(),
                )
                .unwrap(),
                Default::default(),
            ),
            Kind::BlockLaTeX => (
                format!(
                    "<br>\n<div class=\"latex\">{}",
                    katex::render_with_opts(
                        &self.get_text().cloned().unwrap_or_default(),
                        katex::Opts::builder()
                            .output_type(katex::OutputType::Mathml)
                            .display_mode(true)
                            .build()
                            .unwrap(),
                    )
                    .unwrap()
                ),
                String::from("\n</div>\n<br>"),
            ),
            Kind::CustomElement(_c) => {
                if let Ok(mut t) = Template::try_from(&self.to_owned()) {
                    // Remove already rendered elements
                    inside = String::new();
                    let html = t.render(path)?;
                    self.asset_list.append(&mut t.asset_list);
                    (html, Default::default())
                } else {
                    Default::default()
                }
            }
            _ => Default::default(),
        };
        Some(format!("{start}{inside}{end}"))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs;

    use bismuth_custom::parse_custom;

    fn snapshot(content: &str) -> String {
        let mut parser = Parser::new_test("/test/test.md", content);
        parser.parse().unwrap();
        let parser = parse_custom(parser, &vec![]);
        let mut render = Renderer::new(parser);
        render.render(&PathBuf::new()).unwrap()
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

    snapshot!(test_3, "***test?***");

    snapshot!(test_br, "test test \n\n\ntest test\ntest\n");
    snapshot!(
        test_latex,
        "this is a test for inline latex using katex: $E = mc^2$"
    );

    snapshot!(
        test_latex_1,
        "this is a test for block latex using katex:\n$$E = mc^2$$"
    );

    snapshot_path!(test_path, "./testdata/test/render/test.md");
}
