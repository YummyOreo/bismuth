#![allow(unused)]
use socrates_md::MarkdownFile;

pub enum Token {
    Italic([usize; 2]),
    Bold([usize; 2]),

    Heading1([usize; 2]),
    Heading2([usize; 2]),
    Heading3([usize; 2]),
    Heading4([usize; 2]),

    Link([usize; 2]),
    File([usize; 2]),

    Blockquote([usize; 2]),

    ListItem([usize; 2]),

    InlineCode([usize; 2]),
    CodeBlock([usize; 2]),

    CustomList([usize; 2]),

    Fontmatter([usize; 2]),
}

#[derive(Default)]
pub struct State {
    current_line: usize,
    current_column: usize,
}

pub struct Lexer<'a> {
    file: &'a MarkdownFile,

    state: State,
    tokens: Vec<Token>,
}

impl<'a> Lexer<'a> {
    pub fn new(file: &'a MarkdownFile) -> Self {
        Lexer {
            file,
            state: Default::default(),
            tokens: vec![],
        }
    }

    pub fn next_line() {}
    pub fn next_char() {}

    pub fn read_line() {}
    pub fn read_char() {}
}
