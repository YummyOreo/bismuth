use bismuth_lexer::{
    token::{Token, TokenType},
    Lexer,
};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub mod custom;
pub mod error;
mod frontmatter;
pub mod tree;
use crate::{
    error::ParseError,
    frontmatter::FrontMatter,
    tree::{Ast, Element, Kind},
};

#[derive(Default, Debug, Clone)]
pub struct Metadata {
    pub absolute_path: PathBuf,
    pub frontmatter: FrontMatter,
}

impl Metadata {
    pub fn new(path: &Path) -> Self {
        Metadata {
            absolute_path: path.to_path_buf(),
            frontmatter: FrontMatter::new(path),
        }
    }
}

#[derive(Clone)]
struct State {
    pub new_line: bool,
    pub inside: Vec<u32>,
    pub indent_level: i32,
}

impl Default for State {
    fn default() -> Self {
        Self {
            new_line: true,
            inside: vec![],
            indent_level: 0,
        }
    }
}

type ParseReturn = Result<(), ParseError>;

#[derive(Clone)]
pub struct Parser {
    pub lexer: Lexer,

    index: usize,

    pub metadata: Metadata,

    current_element: Option<Element>,

    state: State,
    pub has_custom: bool,

    pub ast: Ast,
}

// Utils

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        let metadata = Metadata::new(&lexer.path);
        Parser {
            lexer,

            index: 0,

            metadata,

            current_element: None,

            ast: Default::default(),

            state: Default::default(),
            has_custom: false,
        }
    }

    pub fn new_test(path: &str, content: &str) -> Self {
        let mut lexer = Lexer::new_test(PathBuf::from(path), content.to_string());
        lexer.run_lexer().unwrap();
        Parser::new(lexer)
    }

    fn current_token(&self) -> Result<&Token, ParseError> {
        self.lexer
            .tokens
            .get(self.index)
            .ok_or(ParseError::GetToken(self.index))
    }

    fn current_token_chars(&self) -> Result<&Vec<char>, ParseError> {
        Ok(&self.current_token()?.text)
    }

    fn token_len(&self, token: &Token) -> usize {
        (token.end - token.start) + 1
    }

    fn current_token_len(&self) -> Result<usize, ParseError> {
        Ok(self.token_len(self.current_token()?))
    }

    fn current_element(&self) -> Option<&Element> {
        self.current_element.as_ref()
    }

    fn append_element(&mut self, elm: Element) {
        if self.current_element().is_some() {
            let curr_elm = {
                let mut elm = self.current_element.as_mut().expect("Should be there");

                for compair_id in &self.state.inside {
                    if let Some(p) = elm.elements.iter().position(|e| e.get_id() == *compair_id) {
                        elm = elm.elements.get_mut(p).expect("should be there");
                    }
                }
                elm
            };

            if elm.kind != Kind::EndOfLine {
                self.state.new_line = false;
                curr_elm.append_element(elm);
                return;
            } else {
                self.ast
                    .elements
                    .push(self.current_element.clone().unwrap());
                self.ast.elements.push(elm);
                self.current_element = None;

                self.reset_state();
                return;
            }
        } else if elm.kind == Kind::EndOfLine {
            self.ast.elements.push(elm);
            return;
        }
        self.state.new_line = false;
        self.set_current(elm);
    }

    fn set_current(&mut self, elm: Element) {
        self.current_element = Some(elm);
    }

    fn advance_token(&mut self) -> Result<&Token, ParseError> {
        self.advance_n_token(1)
    }

    fn back_token(&mut self) -> Result<&Token, ParseError> {
        self.back_n_token(1)
    }

    fn advance_n_token(&mut self, n: usize) -> Result<&Token, ParseError> {
        if self.index + n >= self.lexer.tokens.len() {
            return Err(ParseError::Move(self.index + n));
        }
        self.index += n;

        self.current_token()
    }

    fn back_n_token(&mut self, n: usize) -> Result<&Token, ParseError> {
        if self.index.checked_sub(n).is_none() {
            return Err(ParseError::Move(self.index + n));
        }
        self.index = self.index.checked_sub(n).expect("Should work");

        self.current_token()
    }

    fn peek_at(&self, n: usize) -> Result<&Token, ParseError> {
        self.lexer.tokens.get(n).ok_or(ParseError::Peek(n))
    }

    /// This is relitive
    fn peek(&self, n: usize) -> Result<&Token, ParseError> {
        self.peek_at(self.index + n)
    }

    /// This is relitive
    fn peek_till(&self, n: usize) -> Result<Vec<Token>, ParseError> {
        if n >= self.lexer.tokens.len() {
            return Err(ParseError::Peek(n));
        }

        let tokens_after = self.lexer.tokens.split_at(self.index).1;
        Ok(tokens_after.split_at(n).0.to_vec())
    }

    /// This is relitive
    fn peek_till_kind(&self, kind: &TokenType) -> Result<Vec<Token>, ParseError> {
        let tokens_after = self.lexer.tokens.split_at(self.index).1;
        let end = tokens_after
            .iter()
            .position(|t| &t.kind == kind)
            .ok_or(ParseError::Peek(0))?;

        Ok(tokens_after.split_at(end).0.to_vec())
    }

    /// This is relitive
    /// Returs with error if \n occurs before the kind
    /// Returns w/ error if it could not find the token
    /// Returns w/ the tokens till it if it could find the token
    fn peek_till_kind_eol(&self, kind: &TokenType) -> Result<Vec<Token>, ParseError> {
        let tokens_after = self.lexer.tokens.split_at(self.index).1;

        let mut end = None;
        for (index, token) in tokens_after.iter().enumerate() {
            if token.kind == TokenType::EndOfLine {
                return Err(ParseError::Peek(0));
            } else if &token.kind == kind {
                end = Some(index);
                break;
            }
        }
        match end {
            Some(end) => Ok(tokens_after.split_at(end).0.to_vec()),
            None => Err(ParseError::Peek(0)),
        }
    }

    /// This is relitive
    fn peek_till_pattern(&self, kinds: &[TokenType]) -> Result<usize, ParseError> {
        let tokens_after = self.lexer.tokens.split_at(self.index).1;

        let mut pat_index = 0;
        let mut repeate_len = 0;

        // we have to do this complecated stuff because one token may have 2 or more of its kind
        // So for each token, we will get the len, then check if there is the same ammount of kinds
        // in the list of kinds (`kinds`). If it is then we will add the len to pat_index (skipping
        // the checked ones) and add len - 1 to repeate_len to be subtraced because they were
        // condinced.
        // Then to get the return value we will subtract the index of the tokens to the (pat index
        // - repeate_len), then add it all w/ our current index
        for (i, token) in tokens_after.iter().enumerate() {
            if pat_index >= kinds.len() {
                return Ok((i - (pat_index - repeate_len)) + self.index);
            }
            let kind = &kinds[pat_index];

            if kind == &token.kind {
                // handle text edge case
                if kind == &TokenType::Text {
                    pat_index += 1;
                    continue;
                }

                let len = self.token_len(token);
                let kinds_after = kinds.split_at(pat_index).1;
                let diff = kinds_after
                    .iter()
                    .position(|k| k != kind)
                    .unwrap_or(kinds_after.len());
                if diff == len {
                    pat_index += len;
                    repeate_len += len - 1;
                    continue;
                }
            }
            pat_index = 0;
        }

        Err(ParseError::CouldNotFindPattern)
    }

    fn make_text(&self) -> Result<Element, ParseError> {
        if self.state.new_line {
            let mut elm = Element::new(Kind::Paragraph);
            let mut elm_txt = Element::new(Kind::Text);

            elm_txt.text = Some(self.current_token_chars()?.iter().collect::<String>());

            elm.append_element(elm_txt);
            Ok(elm)
        } else {
            let mut elm = Element::new(Kind::Text);
            elm.text = Some(self.current_token_chars()?.iter().collect::<String>());
            Ok(elm)
        }
    }

    fn reset_state(&mut self) {
        self.state = State::default();
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
            self.append_element(self.make_text()?);
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
        let mut elm = Element::new(Kind::Header);

        let num = self.current_token_len()?;
        elm.add_attr("level", &num);

        self.append_element(elm);
        Ok(())
    }

    fn handle_dash(&mut self) -> ParseReturn {
        let diff = self.current_token_len()?;

        if self.state.new_line {
            if diff == 1 {
                let mut elm = Element::new(Kind::ListItem);
                elm.add_attr("level", &self.state.indent_level);
                self.append_element(elm);

                return Ok(());
            } else if diff == 3 && self.peek(1)?.kind == TokenType::EndOfLine {
                self.append_element(Element::new(Kind::HorizontalRule));
                return Ok(());
            }
        }

        self.append_element(self.make_text()?);
        Ok(())
    }

    fn make_custom(&mut self) -> Result<Element, ParseError> {
        // advance past %{{
        self.advance_n_token(2)?;

        // get where the \n}}\n is indicating the end
        let pattern = vec![
            TokenType::EndOfLine,
            TokenType::CurlybraceRight,
            TokenType::CurlybraceRight,
            TokenType::EndOfLine,
        ];
        let pattern_or = vec![
            TokenType::EndOfLine,
            TokenType::CurlybraceRight,
            TokenType::CurlybraceRight,
            TokenType::EndOfFile,
        ];

        let end = match self.peek_till_pattern(&pattern) {
            Ok(e) => e,
            Err(_) => self.peek_till_pattern(&pattern_or)?,
        };

        let inside_tokens = self.peek_till(end - self.index)?;
        let inside_str = inside_tokens[0..inside_tokens.len()]
            .iter()
            .map(|t| t.text.iter().collect::<String>())
            .collect::<String>();

        // advance past inside till last \n
        self.advance_n_token(inside_tokens.len() + 1)?;

        // makes the custom element
        let c =
            custom::CustomElm::from_string(&inside_str).map_err(ParseError::CustomElementError)?;
        Ok(Element::new(Kind::CustomElement(c)))
    }

    fn handle_precent(&mut self) -> ParseReturn {
        let peek = self.peek(1)?;

        let is_newline = self.state.new_line;
        let is_curlybrace = peek.kind == TokenType::CurlybraceLeft;
        let is_2_len = self.token_len(peek) == 2;

        if is_newline && is_curlybrace && is_2_len {
            let elm = self.make_custom()?;
            self.append_element(elm);

            self.has_custom = true;
            return Ok(());
        }
        self.append_element(self.make_text()?);
        Ok(())
    }

    fn handle_backtick(&mut self) -> ParseReturn {
        // se handle text container
        let len = self.current_token_len()?;
        if len == 1 {
            return self.handle_container_text(TokenType::Backtick);
        } else if len == 3 {
            self.advance_token()?;
            let inside = self.peek_till_kind(&TokenType::Backtick)?;

            let (lang, code) = inside.split_at(
                inside
                    .iter()
                    .position(|t| t.kind == TokenType::EndOfLine)
                    .ok_or(error::ParseError::Peek(0))?,
            );

            // get code and lang
            let lang = lang
                .iter()
                .map(|e| e.text.iter().collect::<String>())
                .collect::<String>();
            let code = code
                .iter()
                .map(|t| t.text.iter().collect::<String>())
                .collect::<String>();

            // make the blockcode element
            let mut elm = Element::new(Kind::BlockCode);
            elm.add_attr("lang", &lang);

            // append the text inside the blockcode
            elm.text = Some(code);

            self.advance_n_token(inside.len())?;
            self.append_element(elm);
        } else {
            self.append_element(self.make_text()?);
        }
        Ok(())
    }

    fn handle_container_text(&mut self, kind: TokenType) -> ParseReturn {
        let len = self.current_token_len()?;
        let elm_kind = match (kind, len) {
            (TokenType::DollarSign, 1) => Kind::InlineLaTeX,
            (TokenType::DollarSign, 2) => Kind::BlockLaTeX,
            (TokenType::Backtick, 1) => Kind::InlineCode,
            _ => Kind::Text,
        };

        if elm_kind == Kind::Text {
            self.append_element(self.make_text()?);
            return Ok(());
        }

        let inside = if len != 3 {
            self.advance_token()?;
            let inside = match self.peek_till_kind_eol(&kind) {
                Ok(t) => t,
                Err(_) => {
                    self.append_element(self.make_text()?);
                    return Ok(());
                }
            };
            self.back_token()?;
            inside
        } else {
            self.advance_token()?;
            let pattern = vec![kind].repeat(len);
            let pat_start = self.peek_till_pattern(&pattern)?;
            self.peek_till(pat_start - self.index)?
        };

        let text = inside
            .iter()
            .map(|t| t.text.iter().collect::<String>())
            .collect::<String>();

        let mut elm = Element::new(elm_kind);

        elm.text = Some(text);

        self.advance_n_token(inside.len() + 1)?;
        self.append_element(elm);

        Ok(())
    }

    fn handle_container(&mut self, kind: TokenType) -> ParseReturn {
        // to handle *** or **test*test*** use patterns
        // oooo, this may be hard bc how would you test for `*` against `***`?
        let elm_kind = match (kind, self.current_token_len()?) {
            (TokenType::Asterisk, 1) | (TokenType::Underscore, 1) => Kind::Italic,
            (TokenType::Asterisk, 2) | (TokenType::Underscore, 2) => Kind::Bold,
            (TokenType::Asterisk, 3) | (TokenType::Underscore, 3) => Kind::BoldItalic,
            _ => Kind::Text,
        };

        if elm_kind == Kind::Text {
            self.append_element(self.make_text()?);
            return Ok(());
        }
        self.advance_token()?;
        let inside = self.peek_till_kind_eol(&kind);
        self.back_token()?;

        let inside = match inside {
            Ok(t) => t,
            Err(_) => {
                self.append_element(self.make_text()?);
                return Ok(());
            }
        };

        let elm = Element::new(elm_kind);
        let elm_id = elm.get_id();
        self.append_element(elm);
        self.state.inside.push(elm_id);

        self.advance_token()?;

        let mut last_index = self.index;
        let mut last_diff = 0;

        for token in &inside {
            if last_diff > 0 {
                last_diff -= 1;
                continue;
            }

            self.parse_token(token)?;

            last_diff = (self.index - last_index) + 1;
            last_index = self.index;

            self.advance_n_token(1)?;
        }
        self.state.inside.pop();
        Ok(())
    }

    // should only appear at start of line, so should be handled after eol
    fn handle_num(&mut self) -> ParseReturn {
        let mut elm = Element::new(Kind::OrderedListElement);
        let num = self
            .current_token_chars()?
            .iter()
            .filter(|c| c.is_numeric())
            .collect::<String>();

        elm.add_attr("num", &num);
        elm.add_attr("level", &self.state.indent_level);

        self.append_element(elm);
        Ok(())
    }

    // Somewhat same as *
    fn handle_bracket(&mut self) -> ParseReturn {
        match self.get_url() {
            Ok((text, url)) => {
                let mut elm = Element::new(Kind::Link);
                elm.text = Some(text);
                elm.add_attr("link", &url);
                self.append_element(elm);
            }
            Err(ParseError::Peek(_)) => {
                self.append_element(self.make_text()?);
            }
            Err(e) => {
                return Err(e);
            }
        }
        Ok(())
    }

    // just bracket with diff type and checks
    fn handle_exclamation(&mut self) -> ParseReturn {
        let peek = self.peek(1);
        match peek {
            Ok(t) => {
                if t.kind != TokenType::BracketLeft {
                    self.append_element(self.make_text()?);
                    return Ok(());
                }
            }
            Err(_) => {
                self.append_element(self.make_text()?);
                return Ok(());
            }
        }

        self.advance_token()?;
        match self.get_url() {
            Ok((text, url)) => {
                let mut elm = Element::new(Kind::FilePrev);
                elm.text = Some(text);
                elm.add_attr("link", &url);
                self.append_element(elm);
            }
            Err(ParseError::Peek(_)) => {
                self.append_element(self.make_text()?);
            }
            Err(e) => {
                return Err(e);
            }
        }
        Ok(())
    }

    fn get_url(&mut self) -> Result<(String, String), ParseError> {
        self.advance_token()?;

        let text = self.peek_till_kind_eol(&TokenType::BracketRight)?;
        let text_s = text
            .iter()
            .map(|t| t.text.iter().collect::<String>())
            .collect::<String>();

        self.advance_n_token(text.len() + 2)?;

        let link = self.peek_till_kind(&TokenType::ParenthesisRight)?;
        let link_s = link
            .iter()
            .map(|t| t.text.iter().collect::<String>())
            .collect::<String>();

        self.advance_n_token(link.len())?;

        Ok((text_s, link_s))
    }

    fn handle_frontmatter(&mut self) -> ParseReturn {
        let mut inside = self.peek_till_kind(&TokenType::FrontmatterEnd).unwrap();
        inside.remove(0);
        inside.pop();
        let s = inside
            .iter()
            .map(|t| t.text.iter().collect::<String>())
            .collect::<String>();

        self.advance_n_token(inside.len() + 1)?;
        self.metadata
            .frontmatter
            .update_from_str(&s)
            .map_err(ParseError::FrontMatterError)?;
        Ok(())
    }

    fn parse_token(&mut self, token: &Token) -> ParseReturn {
        match token.kind {
            TokenType::Text | TokenType::CurlybraceLeft | TokenType::CurlybraceRight => {
                self.append_element(self.make_text()?)
            }

            TokenType::EndOfLine => self.append_element(Element::new(Kind::EndOfLine)),
            TokenType::Tab => self.handle_tab()?,
            TokenType::Whitespace => self.handle_whitespace()?,

            TokenType::Hash => self.handle_hash()?,
            TokenType::Percent => self.handle_precent()?,
            TokenType::Dash => self.handle_dash()?,
            TokenType::ListNumber => self.handle_num()?,

            TokenType::GreaterThan => self.append_element(Element::new(Kind::Blockquote)),

            TokenType::Asterisk => self.handle_container(TokenType::Asterisk)?,
            TokenType::Backtick => self.handle_backtick()?,
            TokenType::DollarSign => self.handle_container_text(TokenType::DollarSign)?,
            TokenType::Underscore => self.handle_container(TokenType::Underscore)?,

            TokenType::BracketLeft => self.handle_bracket()?,
            TokenType::Exclamation => self.handle_exclamation()?,

            TokenType::FrontmatterStart => self.handle_frontmatter()?,
            _ => {}
        };
        Ok(())
    }

    pub fn parse(&mut self) -> Result<(), ParseError> {
        while {
            match self.advance_token() {
                Ok(t) => Ok(t),
                Err(e) => match e {
                    ParseError::Move(_) => Err(e),
                    _ => {
                        return Err(e);
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
        // append the last element
        if let Some(elm) = self.current_element.clone() {
            self.ast.elements.push(elm);
        }
        self.current_element = None;
        Ok(())
    }
}

fn sort_hm(map: &HashMap<String, String>) -> String {
    let mut keys_values = map.iter().collect::<Vec<(&String, &String)>>();
    keys_values.sort();
    format!("{keys_values:?}")
}

fn format_kind(kind: &Kind) -> String {
    match kind {
        Kind::CustomElement(c) => {
            format!(
                "Custom{{Name: {}, Body: {:?}, Values: {}, Template: {:?}}}",
                c.name,
                c.body,
                sort_hm(&c.values),
                c.template
            )
        }
        _ => format!("{kind:?}"),
    }
}

fn render_element(element: &Element, level: usize) -> String {
    let t = "    ";
    let ts = t.repeat(level);
    let mut s = format!(
        "Element{{\n{ts}{t}Kind: {:#?},\n{ts}{t}Text: {:?},\n{ts}{t}Attrs: {},\n{ts}{t}Elements: [",
        format_kind(&element.kind),
        element.text,
        sort_hm(&element.attrs)
    );
    for elm in &element.elements {
        let inside_s = format!("\n{ts}{ts}{t}{},", render_element(elm, level + 1));
        s.push_str(&inside_s);
    }
    s.push_str(&format!("\n{ts}{t}])\n{ts}}}"));
    s
}

impl core::fmt::Debug for Parser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ast = &self.ast;
        let mut s = String::new();
        s.push_str(&format!("{:#?}\n", self.metadata.frontmatter));
        for element in &ast.elements {
            s.push_str(&render_element(&element.clone(), 0));
            s.push('\n');
        }

        write!(f, "{}", s)
    }
}

#[cfg(test)]
mod test_utils {
    use super::*;
    use std::path::PathBuf;

    fn init_lexer(content: &str) -> Lexer {
        let content = content.to_string();
        let path = PathBuf::from("/test/test.md");
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
            &Token::new(TokenType::EndOfLine, vec!['\n'], 17, 17),
            parser.advance_token().unwrap()
        );
        assert_eq!(
            &Token::new(TokenType::EndOfFile, Vec::new(), 18, 18),
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
        let lexer = init_lexer("this is a test \n[]\n, [[]]");
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
        let r = parser.peek_till_pattern(&pattern).unwrap();
        assert_eq!(l, r);

        let pattern_2: Vec<TokenType> = vec![
            TokenType::Text,
            TokenType::BracketLeft,
            TokenType::BracketLeft,
            TokenType::BracketRight,
            TokenType::BracketRight,
            TokenType::EndOfLine,
        ];
        let l = 6;
        let r = parser.peek_till_pattern(&pattern_2).unwrap();
        assert_eq!(l, r);

        let lexer = init_lexer("\ntest\n}}\ntest");
        let parser = Parser::new(lexer);

        let pattern_2: Vec<TokenType> = vec![
            TokenType::EndOfLine,
            TokenType::CurlybraceRight,
            TokenType::CurlybraceRight,
            TokenType::EndOfLine,
        ];

        let l = 3;
        let r = parser.peek_till_pattern(&pattern_2).unwrap();
        assert_eq!(l, r);

        let lexer = init_lexer("this is a test \n[]\n another line \n[]\n");
        let mut parser = Parser::new(lexer);

        parser.advance_n_token(4).unwrap();

        let l = 8;
        let r = parser.peek_till_pattern(&pattern).unwrap();
        assert_eq!(l, r);
    }
}

#[cfg(test)]
mod test {
    // TODO: improve how this is handled, make it so you do not have to always init a test
    // function, also make 2 types of tests: from string | from file. Denote this in the name by
    // adding it to the start of the name along with test
    // (ie test_str_$name or test_file_$name)
    use super::*;
    use std::fs;
    use std::path::PathBuf;

    fn init_lexer(content: &str) -> Lexer {
        let content = content.to_string();
        let path = PathBuf::from("/test/test.md");
        let mut l = Lexer::new_test(path, content);
        l.run_lexer().unwrap();
        l
    }

    fn init_lexer_path(path: &str) -> Lexer {
        let path = PathBuf::from(path);
        let content = fs::read_to_string(&path).unwrap();
        let mut l = Lexer::new_test(path, content);
        l.run_lexer().unwrap();
        l
    }

    fn snapshot_str(content: &str) -> String {
        let lexer = init_lexer(content);
        let mut parser = Parser::new(lexer);
        parser.parse().unwrap();
        format!("{parser:#?}")
    }

    macro_rules! snapshot_str {
        ($name:tt, $content:tt) => {
            #[test]
            fn $name() {
                let mut settings = insta::Settings::clone_current();
                settings.set_snapshot_path("../testdata/output/");
                settings.bind(|| {
                    insta::assert_snapshot!(snapshot_str($content));
                });
            }
        };
    }

    fn snapshot_path(path: &str) -> String {
        let lexer = init_lexer_path(path);
        let mut parser = Parser::new(lexer);
        parser.parse().unwrap();
        format!("{parser:#?}")
    }

    macro_rules! snapshot_path {
        ($name:tt, $path:tt) => {
            #[test]
            fn $name() {
                let mut settings = insta::Settings::clone_current();
                settings.set_snapshot_path("../testdata/output/");
                settings.bind(|| {
                    insta::assert_snapshot!(snapshot_path($path));
                });
            }
        };
    }

    snapshot_str!(test, "test \n test *__test__* none");
    snapshot_str!(
        test_1,
        "## header\n> blockquote\n- list\n\t- item\n        - level _**two??**_"
    );
    snapshot_str!(test_2, "1. list item\n\t2. hmm");
    snapshot_str!(
        test_3,
        "[test](link.url)\n![prev of a file](example.com)\ntest ![other txt](./*test*)"
    );
    snapshot_str!(
        test_custom,
        "%{{\nname: test\nother: key\n---\nbody\ntest\n}}\n---\ntest"
    );
    snapshot_str!(
        test_custom_1,
        "%{{\nname: test\nother: key\n---\nbody\ntest\n}}"
    );
    snapshot_str!(
        test_fm,
        "---\ntitle: test title\npath: /test/path/\nvalues:\n    - key: value\n---"
    );
    snapshot_str!(test_latex, "test $e = mc^2$ \n $$e = mc^3$$");
    snapshot_str!(test_linebreak, "test\n\n---\n");
    snapshot_str!(test_inline, "\ntest `test`\n");

    snapshot_path!(test_load, "./testdata/tests/test.md");
    snapshot_path!(test_load_1, "./testdata/tests/test1.md");
}
