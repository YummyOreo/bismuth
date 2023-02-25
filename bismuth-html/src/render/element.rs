use crate::render::code::highlight;
use crate::render::Render;

#[derive(Clone)]
pub struct Element {
    inside: Vec<Element>,
    kind: HtmlElement,
}

impl Render for Element {
    fn render<T: Render + Clone>(&mut self, content: &[T]) -> String {
        self.kind.render(&self.inside)
    }
}

#[derive(Clone)]
pub enum HtmlElement {
    Paragraph,
    Text { text: String },
    Bold,
    Italic,
    Blockquote,

    Link { text: String, link: String },
    Filelink { text: String, link: String },

    ListItem { level: i32 },
    NumList { level: i32, num: String },

    InlineCode { code: String },
    Blockcode { code: String, lang: String },

    InlineLaTeX { tex: String },
    BlockLaTeX { tex: String },

    Header { level: i8 },

    HorizontalRule,
    LineBreak,

    // todo: Later impl this, after above
    CustomElm,
}
impl Render for String {
    fn render<T: Render + Clone>(&mut self, _: &[T]) -> String {
        self.to_string()
    }
}

impl Render for HtmlElement {
    // have to re impl this in element and rewrite inside too
    fn render<T: Render + Clone>(&mut self, content: &[T]) -> String {
        let first = content.first();
        let inside = match first {
            Some(r) => {
                let mut other = content.to_vec();
                other.remove(0);
                r.clone().render(&other)
            }
            None => String::new(),
        };

        let (start, end) = match self {
            Self::Paragraph => ("<p>".to_owned(), "</p>".to_owned()),
            Self::Text { text } => (text.clone(), "".to_owned()),
            Self::Bold => ("<b>".to_string(), "</b>".to_string()),
            Self::Italic => ("<i>".to_string(), "</i>".to_string()),
            Self::Blockquote => ("<blockquote>".to_string(), "</blockquote>".to_string()),
            Self::Link { text, link } => (
                format!("<a href=\"{}\" target=\"_blank\">{}", link, text),
                "</a>".to_string(),
            ),
            Self::Filelink { text, link } => (
                format!("<img src=\"{}\" alt=\"{}\">", link, text),
                "".to_string(),
            ),

            Self::ListItem { level } => (
                format!("<li class=\"level-{}\">", level),
                "</li>".to_string(),
            ),
            Self::NumList { level, num } => (
                format!("<li class=\"level-{}\">{}.", level, num),
                "</li>".to_string(),
            ),

            // TODO: --InlineCode + BLockcode-- + InlineLaTeX + BlockLaTeX
            Self::InlineCode { code } => (
                format!("<div class=\"inline-code\">{}", code),
                "</div>".to_string(),
            ),
            Self::Blockcode { code, lang } => (
                // TODO: remove unwrap
                highlight(lang.to_string(), code.to_string()).unwrap(),
                "".to_string(),
            ),

            Self::HorizontalRule => ("<hr>".to_string(), "".to_string()),
            Self::LineBreak => ("<br>".to_string(), "".to_string()),
            _ => ("".to_owned(), "".to_owned()),
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

    #[test]
    fn test() {
        let mut element = HtmlElement::Paragraph;
        let inside_elements = vec![
            HtmlElement::Text {
                text: "this is a test".to_string(),
            },
            HtmlElement::Link {
                text: "test".to_string(),
                link: "example.com".to_string(),
            },
        ];
        let rendered = element.render(&inside_elements);
        snapshot!(rendered);
    }

    #[test]
    fn test_2() {
        let inside_inside = vec![Element {
            inside: vec![],
            kind: HtmlElement::Text {
                text: "test".to_string(),
            },
        }];
        let inside = vec![Element {
            inside: inside_inside,
            kind: HtmlElement::Bold,
        }];
        let inside_2 = vec![Element {
            inside: vec![],
            kind: HtmlElement::Text {
                text: "Blockquote inside".to_string(),
            },
        }];
        let elements = vec![
            Element {
                inside,
                kind: HtmlElement::Paragraph,
            },
            Element {
                inside: inside_2,
                kind: HtmlElement::Blockquote,
            },
        ];
        let mut full_str = String::new();
        for mut element in elements {
            full_str.push_str(&element.render::<Element>(&[]));
            full_str.push_str("\n<br>\n");
        }
        snapshot!(full_str);
    }

    #[test]
    fn test_3() {
        let inside_inside = vec![Element {
            inside: vec![],
            kind: HtmlElement::Text {
                text: "test".to_string(),
            },
        }];
        let inside = vec![Element {
            inside: vec![],
            kind: HtmlElement::Text {
                text: "Blockquote inside".to_string(),
            },
        }];
        let elements = vec![
            Element {
                inside: vec![],
                kind: HtmlElement::Blockcode { code: "fn test() {\n\tprintln!(\"Test\")\n}".to_string(), lang: "Rust".to_string() },
            },
            Element {
                inside,
                kind: HtmlElement::Blockquote,
            },
        ];
        let mut full_str = String::new();
        for mut element in elements {
            full_str.push_str(&element.render::<Element>(&[]));
            full_str.push_str("\n<br>\n");
        }
        snapshot!(full_str);
    }
}
