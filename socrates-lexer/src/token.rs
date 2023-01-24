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

    pub fn append(&mut self, mut c: Vec<char>) {
        self.text.append(&mut c);
        self.end += 1;
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // write!(f, "{:?}(start: {}, end: {})", self.kind, self.start, self.end)
        write!(f, "Token({:?}, {:?}, {} -> {})", self.kind, self.text.iter().collect::<String>(), self.start, self.end)
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
    Whitespace,
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
    ListNumber,

    Backtick,

    CurlybraceLeft,
    CurlybraceRight,

    Percent,

    FontmatterStart,
    FontmatterInside,
    FontmatterEnd,
}
