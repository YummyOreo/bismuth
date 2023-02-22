use crate::render::Render;

#[derive(Clone)]
pub enum HtmlElement {
    Paragraph,
    Text { text: String },
    Bold { text: String },
    Italic { text: String },
    Blockquote { text: String },

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
            _ => ("".to_owned(), "".to_owned()),
        };
        todo!();
    }
}
