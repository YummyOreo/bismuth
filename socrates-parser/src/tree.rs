#[derive(Default)]
pub struct Ast {
    elements: Vec<Node>,
}

pub enum Node {
    Text(Element),
    Link(Element),
    FilePrev(Element),

    Italic(Element),
    Bold(Element),

    Blockquote(Element),

    List(Element),
    ListElement(Element, i8),

    OrderedListElement(Element, i8, i8),

    InlineCode(Element),
    BlockCode(Element),

    InlineLaTeX(Element),
    BlockLaTeX(Element),

    CustomData(Element),

    Header(Element, i8),

    HorizontalRule,

    EndOfLine,
    LineBreak,
}

pub struct Element {
    elements: Vec<Node>,
    text: Option<String>,
    attrs: Vec<(String, String)>,
}
