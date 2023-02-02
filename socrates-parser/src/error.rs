#[derive(Debug, PartialEq)]
pub enum ParseError {
    GetChar(usize),
    Move(usize),
    Peek(usize),
}
