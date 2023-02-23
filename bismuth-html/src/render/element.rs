use crate::render::Render;

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

impl Render for HtmlElement {
    fn render<T: Render + Clone>(&mut self, content: &[T]) -> String {
        let mut inside = String::new();
        for (index, r) in content.iter().enumerate() {
            inside.push_str(&r.clone().render(content.split_at(index + 1).1));
        }
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
        todo!();
    }
}
