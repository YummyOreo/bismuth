#[derive(Debug, PartialEq)]
pub enum ParseError {
    GetChar(usize),
    Move(usize),
    Peek(usize),
}

#[derive(Debug, PartialEq)]
pub enum ElementError {
    GetAttrError(String),
    GetTextError,
}
