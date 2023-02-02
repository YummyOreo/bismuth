#[derive(Debug, PartialEq)]
pub enum ParseError {
    GetToken(usize),
    Move(usize),
    Peek(usize),
    WhitespaceError,
}

#[derive(Debug, PartialEq)]
pub enum ElementError {
    GetAttrError(String),
    GetTextError,
}
