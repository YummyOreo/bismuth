use serde_yaml::Error;
use crate::custom::CustomElmError;

#[derive(Debug)]
pub enum ParseError {
    GetToken(usize),
    Move(usize),
    Peek(usize),
    WhitespaceError,
    CouldNotFindPattern,

    CustomElementError(CustomElmError),
    FontMatterError(Error),
}

#[derive(Debug, PartialEq)]
pub enum ElementError {
    GetAttrError(String),
    GetTextError,
}
