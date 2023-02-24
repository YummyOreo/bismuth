use crate::render::Render;

pub struct Element {
    inside: Vec<Element>,
    kind: HtmlElement,
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

    InlineCode { text: String },
    Blockcode { text: String, lang: String },

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

            // TODO: InlineCode + BLockcode + InlineLaTeX + BlockLaTeX
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
            settings.set_snapshot_path("../testdata/output/");
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
}
