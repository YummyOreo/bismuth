use crate::render::code::highlight;
use crate::render::Render;

#[derive(Clone)]
pub struct HtmlElement {
    inside: Vec<HtmlElement>,
    kind: ElementKind,
}

impl HtmlElement {
    pub fn new(kind: ElementKind, inside: Vec<HtmlElement>) -> Self {
        Self {
            kind,
            inside,
        }
    }
}

#[derive(Clone)]
pub enum ElementKind {
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

impl Render for ElementKind {
    // have to re impl this in element and rewrite inside too
    fn render(&mut self) -> String {
        // let first = content.first();
        // let inside = match first {
        //     Some(r) => {
        //         let mut other = content.to_vec();
        //         other.remove(0);
        //         r.clone().render(&other)
        //     }
        //     None => String::new(),
        // };
        let inside = "".to_string();

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

    // #[test]
    // fn test() {
    //     let mut element = ElementKind::Paragraph;
    //     let inside_elements = vec![
    //         ElementKind::Text {
    //             text: "this is a test".to_string(),
    //         },
    //         ElementKind::Link {
    //             text: "test".to_string(),
    //             link: "example.com".to_string(),
    //         },
    //     ];
    //     let rendered = element.render(&inside_elements);
    //     snapshot!(rendered);
    // }
    //
    // #[test]
    // fn test_2() {
    //     let inside_inside = vec![HtmlElement {
    //         inside: vec![],
    //         kind: ElementKind::Text {
    //             text: "test".to_string(),
    //         },
    //     }];
    //     let inside = vec![HtmlElement {
    //         inside: inside_inside,
    //         kind: ElementKind::Bold,
    //     }];
    //     let inside_2 = vec![HtmlElement {
    //         inside: vec![],
    //         kind: ElementKind::Text {
    //             text: "Blockquote inside".to_string(),
    //         },
    //     }];
    //     let elements = vec![
    //         HtmlElement {
    //             inside,
    //             kind: ElementKind::Paragraph,
    //         },
    //         HtmlElement {
    //             inside: inside_2,
    //             kind: ElementKind::Blockquote,
    //         },
    //     ];
    //     let mut full_str = String::new();
    //     for mut element in elements {
    //         full_str.push_str(&element.render::<HtmlElement>(&[]));
    //         full_str.push_str("\n<br>\n");
    //     }
    //     snapshot!(full_str);
    // }
    //
    // #[test]
    // fn test_3() {
    //     let inside_inside = vec![HtmlElement {
    //         inside: vec![],
    //         kind: ElementKind::Text {
    //             text: "test".to_string(),
    //         },
    //     }];
    //     let inside = vec![HtmlElement {
    //         inside: vec![],
    //         kind: ElementKind::Text {
    //             text: "Blockquote inside".to_string(),
    //         },
    //     }];
    //     let elements = vec![
    //         HtmlElement {
    //             inside: vec![],
    //             kind: ElementKind::Blockcode {
    //                 code: "fn test() {\n\tprintln!(\"Test\")\n}".to_string(),
    //                 lang: "rust".to_string(),
    //             },
    //         },
    //         HtmlElement {
    //             inside,
    //             kind: ElementKind::Blockquote,
    //         },
    //     ];
    //     let mut full_str = String::new();
    //     for mut element in elements {
    //         full_str.push_str(&element.render::<HtmlElement>(&[]));
    //         full_str.push_str("\n<br>\n");
    //     }
    //     snapshot!(full_str);
    // }
}
