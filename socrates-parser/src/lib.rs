#![allow(dead_code)]
use socrates_lexer::{
    token::{Token, TokenType},
    Lexer,
};
use std::path::{Path, PathBuf};

mod custom;
mod error;
mod fontmatter;
mod tree;
use crate::{fontmatter::FontMatter, tree::Ast};

#[derive(Default)]
pub struct Metadata {
    absolute_path: PathBuf,
    fontmatter: FontMatter,
}

impl Metadata {
    pub fn new(path: &Path) -> Self {
        Metadata {
            absolute_path: path.to_path_buf(),
            fontmatter: FontMatter::new(path),
        }
    }
}

pub struct Parser {
    pub lexer: Lexer,

    current_token_index: usize,

    metadata: Metadata,

    pub ast: Ast,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        let metadata = Metadata::new(&lexer.path);
        Parser {
            lexer,

            current_token_index: 0,

            metadata,

            ast: Default::default(),
        }
    }

    fn current(&mut self) -> Result<&Token, error::ParseError> {
        self.lexer
            .tokens
            .get(self.current_token_index)
            .ok_or(error::ParseError::GetChar(self.current_token_index))
    }

    fn advance_token(&mut self) -> Result<&Token, error::ParseError> {
        self.advance_n_token(1)
    }

    fn advance_n_token(&mut self, n: usize) -> Result<&Token, error::ParseError> {
        if self.current_token_index + n >= self.lexer.tokens.len() {
            return Err(error::ParseError::Move(self.current_token_index + n));
        }
        self.current_token_index += n;

        self.current()
    }

    fn peek(&self, n: usize) -> Result<&Token, error::ParseError> {
        self.lexer.tokens.get(n).ok_or(error::ParseError::Peek(n))
    }

    fn peek_after(&self, n: usize) -> Result<&Token, error::ParseError> {
        self.peek(self.current_token_index + n)
    }

    fn peek_before(&self, n: usize) -> Result<&Token, error::ParseError> {
        self.peek(self.current_token_index - n)
    }

    fn peek_till_kind(&mut self, kind: TokenType) -> Result<Vec<Token>, error::ParseError> {
        let tokens_after = self.lexer.tokens.split_at(self.current_token_index).1;
        let end = tokens_after
            .iter()
            .position(|t| t.kind == kind)
            .ok_or(error::ParseError::Peek(0))?;

        Ok(tokens_after.split_at(end).0.to_vec())
    }

    pub fn parse_current(&mut self) {}

    pub fn parse(&mut self) -> Result<(), error::ParseError> {
        while let Ok(token) = {
            match self.advance_token() {
                Ok(t) => Ok(t),
                Err(e) => match e {
                    error::ParseError::Move(_) => Err(e),
                    _ => {
                        panic!("{e:#?}");
                    }
                },
            }
        } {
            if token.kind == TokenType::EndOfFile {
                break;
            }

            self.parse_current();
        }
        Ok(())
    }
}

#[cfg(test)]
mod test_utils {
    use super::*;
    use std::path::PathBuf;

    fn init_lexer(content: &str) -> Lexer {
        let content = content.to_string();
        let path = PathBuf::from("/test/");
        let mut l = Lexer::new_test(path, content);
        l.run_lexer().unwrap();
        l
    }

    #[test]
    fn advance_token_test() {
        let lexer = init_lexer("this is a test []");
        let mut parser = Parser::new(lexer);
        assert_eq!(
            &Token::new(
                TokenType::Text,
                "this is a test ".chars().collect::<Vec<char>>(),
                0,
                14
            ),
            parser.advance_token().unwrap()
        );
        assert_eq!(
            &Token::new(TokenType::BracketRight, vec![']'], 16, 16),
            parser.advance_n_token(2).unwrap()
        );
        assert_eq!(
            &Token::new(TokenType::EndOfFile, Vec::new(), 17, 17),
            parser.advance_token().unwrap()
        );
        assert_eq!(Err(error::ParseError::Move(5)), parser.advance_token());
    }

    #[test]
    fn peek_till_test() {
        let lexer = init_lexer("this is a test []");
        let mut parser = Parser::new(lexer);
        // skip start of file
        parser.advance_token().unwrap();

        let r = parser.peek_till_kind(TokenType::BracketRight).unwrap();
        let l = vec![
            Token::new(
                TokenType::Text,
                "this is a test ".chars().collect::<Vec<char>>(),
                0,
                14,
            ),
            Token::new(TokenType::BracketLeft, vec!['['], 15, 15),
        ];

        assert_eq!(l, r);
    }
}
