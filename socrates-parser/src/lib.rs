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

    current_token: Option<Token>,

    metadata: Metadata,

    pub ast: Ast,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        let metadata = Metadata::new(&lexer.path);
        let current_token = lexer.tokens.get(0).cloned();
        Parser {
            lexer,

            current_token_index: 0,
            current_token,

            metadata,

            ast: Default::default(),
        }
    }

    fn advance_token(&mut self) -> Option<&Token> {
        self.advance_n_token(1)
    }

    fn advance_n_token(&mut self, n: usize) -> Option<&Token> {
        self.current_token_index += n;
        self.current_token = self.lexer.tokens.get(self.current_token_index).cloned();
        self.current_token.as_ref()
    }

    fn peek(&self, n: usize) -> Option<&Token> {
        self.lexer.tokens.get(n)
    }

    fn peek_after(&self, n: usize) -> Option<&Token> {
        self.peek(self.current_token_index + n)
    }

    fn peek_before(&self, n: usize) -> Option<&Token> {
        self.peek(self.current_token_index - n)
    }

    fn peek_till_kind(&mut self, kind: TokenType) -> Option<Vec<Token>> {
        let tokens_after = self.lexer.tokens.split_at(self.current_token_index).1;
        let end = tokens_after.iter().position(|t| t.kind == kind)?;
        Some(tokens_after.split_at(end).0.to_vec())
    }

    pub fn parse(&mut self) {
        while let Some(current_token) = self.advance_token() {}
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
        assert_eq!(None, parser.advance_token());
    }

    #[test]
    fn peek_till_test() {
        let lexer = init_lexer("this is a test []");
        let mut parser = Parser::new(lexer);
        // skip start of file
        parser.advance_token();

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
