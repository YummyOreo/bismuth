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

    fn token_len(&self, token: &Token) -> usize {
        (token.end - token.start) + 1
    }

    fn current_token_len(&self) -> Result<usize, error::ParseError> {
        Ok(self.token_len(self.current_token()?))
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

    fn peek_till(&self, n: usize) -> Result<Vec<Token>, error::ParseError> {
        if n >= self.lexer.tokens.len() || n < self.current_token_index {
            return Err(error::ParseError::Peek(n));
        }

        let tokens_after = self.lexer.tokens.split_at(self.current_token_index).1;
        Ok(tokens_after.split_at(n).0.to_vec())
    }

    fn peek_till_kind(&self, kind: &TokenType) -> Result<Vec<Token>, error::ParseError> {
        let tokens_after = self.lexer.tokens.split_at(self.current_token_index).1;
        let end = tokens_after
            .iter()
            .position(|t| &t.kind == kind)
            .ok_or(error::ParseError::Peek(0))?;

        Ok(tokens_after.split_at(end).0.to_vec())
    }

    fn till_pattern(&self, kinds: &[TokenType]) -> Result<usize, error::ParseError> {
        let tokens_after = self.lexer.tokens.split_at(self.current_token_index).1;
        let mut pat_index = 0;
        for (index, token) in tokens_after.iter().enumerate() {
            if pat_index >= kinds.len() - 1 {
                return Ok((index - pat_index) + self.current_token_index);
            }
            if kinds[pat_index] == token.kind {
                pat_index += 1;
            } else {
                pat_index = 0;
            }
        }
        Err(error::ParseError::CouldNotFindPattern)
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
        let tabs = self.current_token_len()?;
        self.handle_tab_whitespace(tabs)
    }

    fn handle_whitespace(&mut self) -> ParseReturn {
        let diff = self.current_token_len()?;
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
        let num = self.current_token_len()?;
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
        let diff = self.current_token_len()?;
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
        // check if it is a custom elm
        // set inside to true
        // loop till you get to [Newline, CurlybraceRight*2, NewLine]
        // append everything to a string
        // make the element
        let peek = self.peek_after(1)?;

        let is_newline = self.state.new_line;
        let is_curlybrace = peek.kind == TokenType::CurlybraceLeft;
        let is_2_len = self.token_len(peek) == 2;

        if is_newline && is_curlybrace && is_2_len {
            // advance past %{{
            self.advance_n_token(2)?;

            // get where the \n}}\n is indicating the end
            let pattern = vec![
                TokenType::EndOfLine,
                TokenType::CurlybraceRight,
                TokenType::CurlybraceRight,
                TokenType::EndOfLine,
            ];

            let end = self.till_pattern(&pattern)?;

            // gets the tokens that are inside of the custom element
            let inside_tokens = self.peek_till(end)?;

            // appends the text of those tokens to a string
            let inside_str = inside_tokens
                .iter()
                .map(|t| t.text.iter().collect::<String>())
                .collect::<String>();

            // advances past the inside... \n}}\n (we don't need to advance pas the last \n because
            // it will auto do that for us)
            self.advance_n_token(inside_tokens.len() + 2)?;

            // makes the custom element
            let c = custom::CustomElm::from_string(&inside_str)
                .map_err(error::ParseError::CustomElementError)?;

            let elm = Element::new(Kind::CustomElement(c));
            self.append_element(elm);

            return Ok(());
        }
        self.append_element(self.make_text_at_token()?);
        Ok(())
    }

    fn handle_container(&mut self, kind: TokenType) -> ParseReturn {
        // check till token for the type, Ie [ => ] [t*t]
        // Then get the inside
        // Parse the inside (we do this because: __*text*__ would be bold(italic(text)))
        // set that as the text for the container
        let inside = self.peek_till_kind(&kind)?;

        let elm_kind = match (kind, self.token_len(self.current_token()?)) {
            (TokenType::Asterisk, 1) | (TokenType::Underscore, 1) => Kind::Italic,
            (TokenType::Asterisk, 2) | (TokenType::Underscore, 2) => Kind::Bold,
            (TokenType::DollarSign, 1) => Kind::InlineLaTeX,
            (TokenType::DollarSign, 3) => Kind::BlockLaTeX,
            (TokenType::Backtick, 1) => Kind::InlineCode,
            (TokenType::Backtick, 3) => Kind::BlockCode,
            _ => Kind::Text,
        };

        if elm_kind == Kind::Text {
            self.append_element(self.make_text_at_token()?);
            return Ok(());
        } else {
            let elm = Element::new(elm_kind);
            self.append_element(elm);
        }

        for token in &inside {
            self.parse_token(token)?;
        }
        self.advance_n_token(inside.len() + 1)?;
        Ok(())
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

    fn parse_token(&mut self, token: &Token) -> ParseReturn {
        match token.kind {
            TokenType::Text | TokenType::CurlybraceLeft | TokenType::CurlybraceRight => {
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
        while {
            match self.advance_token() {
                Ok(t) => Ok(t),
                Err(e) => match e {
                    error::ParseError::Move(_) => Err(e),
                    _ => {
                        panic!("{e:#?}");
                    }
                },
            }
            .is_ok()
        } {
            let token = self.current_token()?.clone();
            if token.kind == TokenType::EndOfFile {
                break;
            }

            self.parse_token(&token)?;
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

    #[derive(Debug, PartialEq)]
    struct TestError {}

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

        assert_eq!(
            Err(TestError {}),
            parser.advance_token().map_err(|_| TestError {})
        );
    }

    #[test]
    fn peek_till_test() {
        let lexer = init_lexer("this is a test []");
        let mut parser = Parser::new(lexer);
        // skip start of file
        parser.advance_token().unwrap();

        let r = parser.peek_till_kind(&TokenType::BracketRight).unwrap();
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
        let r = parser.peek_till(2).unwrap();

        assert_eq!(l, r);
    }

    #[test]
    fn peek_pattern_test() {
        let lexer = init_lexer("this is a test \n[]\n");
        let mut parser = Parser::new(lexer);
        // skip start of file
        parser.advance_token().unwrap();
        let pattern: Vec<TokenType> = vec![
            TokenType::EndOfLine,    // \n
            TokenType::BracketLeft,  // [
            TokenType::BracketRight, // ]
            TokenType::EndOfLine,    // \n
        ];

        let l = 2;
        let r = parser.till_pattern(&pattern).unwrap();
        assert_eq!(l, r);

        let lexer = init_lexer("this is a test \n[]\n another line \n[]\n");
        let mut parser = Parser::new(lexer);

        parser.advance_n_token(4).unwrap();

        let l = 8;
        let r = parser.till_pattern(&pattern).unwrap();
        assert_eq!(l, r);
    }
}
