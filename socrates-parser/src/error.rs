use crate::custom::CustomElmError;

#[derive(Debug, PartialEq)]
pub enum ParseError {
    GetToken(usize),
    Move(usize),
    Peek(usize),
    WhitespaceError,
    CouldNotFindPattern,

    CustomElementError(CustomElmError),
}

#[derive(Debug, PartialEq)]
pub enum ElementError {
    GetAttrError(String),
    GetTextError,
}
