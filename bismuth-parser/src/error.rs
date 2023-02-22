use crate::custom::CustomElmError;
use serde_yaml::Error;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("could not get token")]
    GetToken(usize),
    #[error("could not move to {0}")]
    Move(usize),
    #[error("could not peek at {0}")]
    Peek(usize),
    #[error("error parsing whitespace")]
    WhitespaceError,
    #[error("could not find a pattern")]
    CouldNotFindPattern,

    #[error("{0}")]
    CustomElementError(CustomElmError),
    #[error("error parsing frontmatter: {0}")]
    FrontMatterError(Error),

    #[error("math error")]
    MathError,
}

#[derive(Debug, PartialEq, Error)]
pub enum ElementError {
    #[error("could not find attr: {0}")]
    GetAttrError(String),
    #[error("no text")]
    GetTextError,
}
