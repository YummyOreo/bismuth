use crate::render::Render;

pub enum HtmlElement {
    Paragraph { text: String },
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
    fn render(&mut self) -> String {
        todo!();
    }
}
