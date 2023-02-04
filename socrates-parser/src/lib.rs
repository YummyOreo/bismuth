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
use crate::{
    fontmatter::FontMatter,
    tree::{Ast, Element, Kind},
};

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

#[derive(Default)]
struct State {
    pub new_line: bool,
    pub indent_level: i32,
}

type ParseReturn = Result<(), error::ParseError>;

pub struct Parser {
    pub lexer: Lexer,

    current_token_index: usize,

    metadata: Metadata,

    current_element: Option<Element>,

    state: State,

    pub ast: Ast,
}

// Utils

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        let metadata = Metadata::new(&lexer.path);
        Parser {
            lexer,

            current_token_index: 0,

            metadata,

            current_element: None,

            ast: Default::default(),
            state: Default::default(),
        }
    }

    fn current_token(&self) -> Result<&Token, error::ParseError> {
        self.lexer
            .tokens
            .get(self.current_token_index)
            .ok_or(error::ParseError::GetToken(self.current_token_index))
    }

    fn current_token_chars(&self) -> Result<&Vec<char>, error::ParseError> {
        Ok(&self.current_token()?.text)
    }

    fn current_token_type(&self) -> Result<&TokenType, error::ParseError> {
        Ok(&self.current_token()?.kind)
    }

    fn current_token_diff(&self) -> Result<usize, error::ParseError> {
        Ok((self.current_token()?.end - self.current_token()?.start) + 1)
    }

    fn current_element(&self) -> Option<&Element> {
        self.current_element.as_ref()
    }

    fn append_element(&mut self, elm: Element) {
        if self.current_element().is_some() {
            let curr_elm = self.current_element.as_mut().expect("Should be there");

            if curr_elm.kind != Kind::EndOfLine {
                self.state.new_line = false;
                curr_elm.append_node(elm);
                return;
            } else {
                self.reset_state();
                self.state.new_line = true;
            }
        }
        self.set_current_elm(elm);
    }

    /// ## Tthsethe *bold* tsthsthsnth [th]()
    /// Header(
    /// [
    /// text,
    /// bold,
    /// text,
    /// link
    /// ]
    /// )
    /// thsetheoasutnhaoe *test* snteahuasenthu
    /// Paragraph(
    /// text,
    /// bold,
    /// text
    /// )
    ///

    fn set_current_elm(&mut self, elm: Element) {
        self.current_element = Some(elm);
    }

    fn advance_token(&mut self) -> Result<&Token, error::ParseError> {
        self.advance_n_token(1)
    }

    fn advance_n_token(&mut self, n: usize) -> Result<&Token, error::ParseError> {
        if self.current_token_index + n >= self.lexer.tokens.len() {
            return Err(error::ParseError::Move(self.current_token_index + n));
        }
        self.current_token_index += n;

        self.current_token()
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

    fn make_text_at_token(&self) -> Result<Element, error::ParseError> {
        if self.state.new_line {
            let mut elm = Element::new(Kind::Paragraph);
            let mut elm_txt = Element::new(Kind::Text);
            elm_txt.text = Some(self.current_token_chars()?.iter().collect::<String>());
            elm.append_node(elm_txt);
            Ok(elm)
        } else {
            let mut elm = Element::new(Kind::Text);
            elm.text = Some(self.current_token_chars()?.iter().collect::<String>());
            Ok(elm)
        }
    }

    fn reset_state(&mut self) {
        self.state.indent_level = 0;
        self.state.new_line = false;
    }
}

// Parsing

impl Parser {
    fn handle_tab(&mut self) -> ParseReturn {
        let tabs = self.current_token_diff()?;
        self.handle_tab_whitespace(tabs)
    }

    fn handle_whitespace(&mut self) -> ParseReturn {
        let diff = self.current_token_diff()?;
        if diff % 4 != 0 {
            let mut elm = Element::new(Kind::Text);
            elm.text = Some(self.current_token_chars()?.iter().collect::<String>());
            return Ok(());
        }
        let tabs = diff / 4;
        self.handle_tab_whitespace(tabs)
    }

    fn handle_tab_whitespace(&mut self, num: usize) -> ParseReturn {
        self.state.indent_level = num as i32;
        Ok(())
    }

    // should only appear at start of line, so should be handled after eol
    fn handle_hash(&mut self) -> ParseReturn {
        let num = self.current_token_diff()?;
        let mut elm = Element::new(Kind::Header);
        elm.add_attr("level", &num.to_string());
        self.append_element(elm);
        Ok(())
    }

    fn handle_greaterthan(&mut self) -> ParseReturn {
        if self.state.indent_level == 1 {
            let elm = Element::new(Kind::Blockquote);
            self.append_element(elm);
        } else {
            self.append_element(self.make_text_at_token()?);
        }
        Ok(())
    }

    fn handle_dash(&mut self) -> ParseReturn {
        let diff = self.current_token_diff()?;
        if self.state.new_line {
            if self.state.indent_level > 0 && diff == 1 {
                let mut elm = Element::new(Kind::ListItem);
                elm.add_attr("level", &diff.to_string());
                self.append_element(elm);

                return Ok(());
            } else if diff == 3 && self.peek_after(1)?.kind == TokenType::EndOfLine {
                let elm = Element::new(Kind::HorizontalRule);
                self.append_element(elm);

                return Ok(());
            }
        }

        self.append_element(self.make_text_at_token()?);
        Ok(())
    }

    fn handle_precent(&mut self) -> ParseReturn {
        todo!()
    }

    fn handle_container(&mut self, kind: TokenType) -> ParseReturn {
        todo!()
    }

    // should only appear at start of line, so should be handled after eol
    fn handle_num(&mut self) -> ParseReturn {
        todo!()
    }

    // Somewhat same as *
    fn handle_bracket(&mut self) -> ParseReturn {
        todo!()
    }

    // just bracket with diff type and checks
    fn handle_exclamation(&mut self) -> ParseReturn {
        todo!()
    }

    fn handle_fontmatter(&mut self) -> ParseReturn {
        todo!()
    }

    fn parse_current(&mut self) -> ParseReturn {
        match self.current_token_type()? {
            TokenType::Text => {
                self.append_element(self.make_text_at_token()?);
                Ok(())
            }
            TokenType::EndOfLine => {
                let elm = Element::new(Kind::EndOfLine);
                self.append_element(elm);
                Ok(())
            }
            TokenType::Tab => self.handle_tab(),
            TokenType::Whitespace => self.handle_whitespace(),

            TokenType::Hash => self.handle_hash(),
            TokenType::Percent => self.handle_precent(),
            TokenType::Dash => self.handle_dash(),
            TokenType::ListNumber => self.handle_num(),

            TokenType::GreaterThan => self.handle_greaterthan(),

            TokenType::Asterisk => self.handle_container(TokenType::Asterisk),
            TokenType::Backtick => self.handle_container(TokenType::Backtick),
            TokenType::DollarSign => self.handle_container(TokenType::DollarSign),
            TokenType::Underscore => self.handle_container(TokenType::Underscore),

            TokenType::BracketLeft => self.handle_bracket(),
            TokenType::Exclamation => self.handle_exclamation(),

            TokenType::FontmatterStart => self.handle_fontmatter(),
            _ => Ok(()),
        }
    }

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

            self.parse_current()?;
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
