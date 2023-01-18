#[derive(Clone)]
pub struct Token<'a> {
    pub start: usize,
    pub end: usize,
    pub kind: TokenType,

    pub text: Vec<&'a char>,
}

impl<'a> Token<'a> {
    pub fn new(kind: TokenType, text: Vec<&'a char>, start: usize, end: usize) -> Self {
        Token {
            start,
            end,
            kind,

            text,
        }
    }

    pub fn push(&mut self, c: &'a char) {
        self.text.push(c);
    }

    pub fn append(&mut self, mut c: Vec<&'a char>) {
        self.text.append(&mut c);
    }
}

#[derive(PartialEq, Eq, Clone)]
pub enum TokenType {
    Text,

    EndOfFile,
    EndOfLine,

    StartOfFile,

    DollarSign,

    // more than one space is a whitespace
    // Whitespace,
    Tab,

    Asterisk,
    Underscore,

    Hash,

    BracketLeft,
    BracketRight,
    ParenthesisLeft,
    ParenthesisRight,
    Exclamation,

    GreaterThan,

    Dash,

    Backtick,

    CurlybraceLeft,
    CurlybraceRight,

    Fontmatter,
}
