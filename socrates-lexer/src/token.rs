#[derive(Clone, Debug)]
pub struct Token {
    pub start: usize,
    pub end: usize,
    pub kind: TokenType,

    pub text: Vec<char>,
}

impl Token {
    pub fn new(kind: TokenType, text: Vec<char>, start: usize, end: usize) -> Self {
        Token {
            start,
            end,
            kind,

            text,
        }
    }

    // pub fn push(&mut self, c: &'a char) {
    //     self.text.push(c);
    // }
    //
    // pub fn append(&mut self, mut c: Vec<&'a char>) {
    //     self.text.append(&mut c);
    // }
    pub fn append(&mut self, mut c: Vec<char>) {
        self.text.append(&mut c);
        self.end += 1;
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
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

    FontmatterStart,
    FontmatterInside,
    FontmatterEnd,
}
