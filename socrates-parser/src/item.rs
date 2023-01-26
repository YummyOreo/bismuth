use std::path::PathBuf;

pub struct Item {
    pub kind: ItemKind,
    pub children: Vec<Item>
}

pub enum ItemKind {
    Root,
    Text(TextItem),
    Url(UrlItem),
    File(FileItem),
    Header(HeaderItem),
    Blockquote,
    List,
    OrderedList,
    ListItem(TextItem),
    HorizontalRule,
    InlineCode(TextItem),
    CodeBlock(CodeBlockItem),
    Italic(TextItem),
    Bold(TextItem),
    LineBreak,
}

pub struct TextItem {
    text: String,
}

pub struct UrlItem {
    text: TextItem,
    url: String,
}

pub struct FileItem {
    file: PathBuf,
}

pub struct HeaderItem {
    level: i8
}

pub struct CodeBlockItem {
    text: TextItem,
    lang: String
}
