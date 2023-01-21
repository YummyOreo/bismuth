#![allow(dead_code, clippy::should_implement_trait)]
use regex::Regex;
use socrates_md::MarkdownFile;
use std::ops::RangeInclusive;

pub mod error;
pub mod token;

pub struct Lexer {
    file: MarkdownFile,
    chars: Vec<char>,
    position: usize,

    current_token: token::Token,
    tokens: Vec<token::Token>,
}

impl Lexer {
    pub fn new(file: MarkdownFile) -> Self {
        let mut content = file.content.chars().collect::<Vec<char>>();
        content.retain(|c| c != &'\r');
        Lexer {
            file,
            chars: content,
            position: 0,

            current_token: token::Token::new(token::TokenType::StartOfFile, vec![], 0, 0),
            tokens: vec![],
        }
    }

    fn current(&self) -> Option<&char> {
        self.chars.get(self.position)
    }

    fn new_token(&mut self, t: token::Token) {
        self.tokens.push(self.current_token.clone());
        self.current_token = t;
    }

    fn move_to(&mut self, pos: usize) -> Option<&char> {
        if pos >= self.chars.len() {
            return None;
        }

        self.position = pos;
        return self.current();
    }

    fn next(&mut self) -> Option<&char> {
        if self.position >= self.chars.len() {
            return None;
        }

        self.position += 1;
        return self.current();
    }

    fn next_line(&mut self) -> usize {
        let mut chars = self.chars.split_at(self.position).1.iter();
        let len = chars.len();
        chars.position(|c| c == &'\n').unwrap_or(len) + self.position
    }

    fn peek(&self, next: usize) -> Option<&char> {
        self.peek_at(self.position + next)
    }

    fn peek_at(&self, index: usize) -> Option<&char> {
        self.chars.get(index)
    }

    fn peek_till(&self, c: &char) -> std::ops::RangeInclusive<usize> {
        let mut chars = self.chars.split_at(self.position).1.iter();
        let len = chars.len();

        self.position..=chars.position(|s| s == c).unwrap_or(len) + self.position - 1
    }

    fn peek_till_diff(&self) -> std::ops::RangeInclusive<usize> {
        let mut chars = self.chars.split_at(self.position).1.iter();
        let len = chars.len();

        self.position
            ..=chars
                .position(|c| c != self.peek_at(self.position).unwrap())
                .unwrap_or(len)
                + self.position
                - 1
    }

    fn peek_regex(&self, re: Regex) -> std::ops::RangeInclusive<usize> {
        //! something that from the current position
        //! will match regex to the chars going forward
        //! (get thi first match)
        let s = String::from_iter(self.chars.split_at(self.position).1);

        match re.find(&s) {
            Some(m) => m.start() + self.position..=m.end() - 1 + self.position,
            None => self.position..=self.position,
        }
    }

    fn get_range(&self, range: RangeInclusive<usize>) -> Vec<char> {
        self.chars[range].to_vec()
    }

    fn make_token_at_pos(&self, kind: token::TokenType) -> token::Token {
        token::Token::new(
            kind,
            vec![*self.current().unwrap()],
            self.position,
            self.position,
        )
    }

    fn handle_hash(&self) -> token::Token {
        let diff = self.peek_till_diff();
        let next = self.peek_at(diff.end() + 1).unwrap_or(&'a');

        if let ' ' = next {
            self.make_token_at_pos(token::TokenType::Hash)
        } else {
            self.make_token_at_pos(token::TokenType::Text)
        }
    }

    fn handle_exclamation(&self) -> token::Token {
        let diff = self.peek_till_diff();
        let next = self.peek_at(diff.end() + 1).unwrap_or(&'a');

        if let ' ' = next {
            self.make_token_at_pos(token::TokenType::Text)
        } else {
            self.make_token_at_pos(token::TokenType::Exclamation)
        }
    }

    fn handle_greaterthan(&self) -> token::Token {
        let peeked = (self.peek(1).unwrap_or(&'a'), self.peek(2).unwrap_or(&'a'));

        match peeked {
            (_, ' ') => self.make_token_at_pos(token::TokenType::Text),
            (' ', _) => self.make_token_at_pos(token::TokenType::GreaterThan),
            (_, _) => self.make_token_at_pos(token::TokenType::Text),
        }
    }

    fn handle_fontmatter(&self) -> token::Token {
        // use self.peek_till_diff and self.peek_till
        unimplemented!()
    }

    fn handle_dash(&self) -> token::Token {
        // this will include fontmatter thing
        if self.current_token.kind == token::TokenType::StartOfFile {
            return self.handle_fontmatter();
        }
        unimplemented!()
    }

    fn match_char(&self) -> token::Token {
        let c = self.current().unwrap();
        match c {
            '\n' => self.make_token_at_pos(token::TokenType::EndOfLine),

            '$' => self.make_token_at_pos(token::TokenType::DollarSign),

            '\t' => self.make_token_at_pos(token::TokenType::Tab),

            '*' => self.make_token_at_pos(token::TokenType::Asterisk),
            '_' => self.make_token_at_pos(token::TokenType::Underscore),

            '#' => self.handle_hash(),

            '[' => self.make_token_at_pos(token::TokenType::BracketLeft),
            ']' => self.make_token_at_pos(token::TokenType::BracketRight),
            '(' => self.make_token_at_pos(token::TokenType::ParenthesisLeft),
            ')' => self.make_token_at_pos(token::TokenType::ParenthesisRight),

            '!' => self.handle_exclamation(),

            '>' => self.handle_greaterthan(),

            '-' => self.handle_dash(),

            '`' => self.make_token_at_pos(token::TokenType::Backtick),

            '{' => self.make_token_at_pos(token::TokenType::CurlybraceLeft),
            '}' => self.make_token_at_pos(token::TokenType::CurlybraceRight),

            _ => self.make_token_at_pos(token::TokenType::Text),
        }
    }

    fn read_char(&mut self) -> Result<(), error::LexerError> {
        // get the token (this will get edge cases, ie. `# ` is different from `#t` (one is text,
        // one is Hash))
        // if the token type is the same as the current token type
        // append it the the current token
        let token = self.match_char();
        if token.kind == self.current_token.kind {
            self.current_token.append(token.text.clone());
        } else {
            self.new_token(token);
        }
        Ok(())
    }

    fn read_until_eof(&mut self) -> Result<(), error::LexerError> {
        while self.current_token.kind != token::TokenType::EndOfFile {
            self.read_char()?;

            // checks if we are at the end of the file.
            // If we are, then set our current token to EndOfFile
            if self.next().is_none() {
                self.new_token(token::Token::new(
                    token::TokenType::EndOfFile,
                    vec![],
                    self.chars.len(),
                    self.chars.len(),
                ));
            }
        }

        // pushes the last token
        // because, when making a new token, it pushes the last token
        self.tokens.push(self.current_token.clone());

        Ok(())
    }

    pub fn run_lexer(&mut self) -> Result<(), error::LexerError> {
        self.read_until_eof()
    }
}

// Tests: Utils

#[cfg(test)]
mod test_utils {
    use super::*;
    use regex::Regex;
    use std::path::PathBuf;

    fn setup(content: &str) -> MarkdownFile {
        let content = content.to_string();
        MarkdownFile {
            content,
            path: PathBuf::new(),
        }
    }

    #[test]
    fn test_peek_till() {
        let file = setup("this is a test aaaaabc");
        let mut lexer = Lexer::new(file);
        lexer.move_to(15);

        assert_eq!(lexer.peek_till(&'c'), 15..=lexer.chars.len() - 2);

        let file = setup("this is a test aaaaaab");
        let mut lexer = Lexer::new(file);
        lexer.move_to(15);

        assert_eq!(lexer.peek_till(&'b'), 15..=lexer.chars.len() - 2);
    }

    #[test]
    fn test_peek_till_diff() {
        let file = setup("this is a test aaaaab");
        let mut lexer = Lexer::new(file);
        lexer.move_to(15);

        assert_eq!(lexer.peek_till_diff(), 15..=lexer.chars.len() - 2);

        let file = setup("this is a test aaaaaa");
        let mut lexer = Lexer::new(file);
        lexer.move_to(15);

        assert_eq!(lexer.peek_till_diff(), 15..=lexer.chars.len() - 1);
    }

    #[test]
    fn test_range() {
        let file = setup("this is a test aaaaaa");
        let mut lexer = Lexer::new(file);
        lexer.move_to(15);

        assert_eq!(
            lexer.get_range(lexer.peek_till_diff()),
            vec!['a', 'a', 'a', 'a', 'a', 'a']
        );

        let file = setup("this is a test aaaaab");
        let mut lexer = Lexer::new(file);
        lexer.move_to(15);

        assert_eq!(
            lexer.get_range(lexer.peek_till_diff()),
            vec!['a', 'a', 'a', 'a', 'a']
        );
    }

    #[test]
    fn test_nexn_line() {
        let file = setup("this is a test aaaaaa \n this is a test");
        let mut lexer = Lexer::new(file);
        lexer.move_to(15);

        let num = lexer.next_line();
        assert_eq!(22, num);
    }

    #[test]
    fn test_regex() {
        let file = setup("This is a test for my test, email@test.com");
        let mut lexer = Lexer::new(file);
        lexer.move_to(27);

        let re = Regex::new("([a-zA-Z0-9]+)@([a-zA-Z]*).([a-z]+)").unwrap();
        assert_eq!(lexer.peek_regex(re), 28..=41);

        let file = setup("This is a test for myttest, email@test.com");
        let mut lexer = Lexer::new(file);
        lexer.move_to(21);

        let re = Regex::new(r"(est,\s)").unwrap();
        assert_eq!(lexer.peek_regex(re), 23..=27);
    }
}

// Tests: Snapshots

#[cfg(test)]
mod test {
    use super::*;
    use crate::MarkdownFile;
    use std::path::PathBuf;

    fn snapshot(path: &str) -> String {
        let path = PathBuf::from(path);
        let file = MarkdownFile::load_file(&path, &path).unwrap();

        let mut lexer = Lexer::new(file);
        lexer.run_lexer().unwrap();

        // let output = String::new();
        // for token in lexer.tokens {
        //     let text = token.text.iter().map(|c| c.to_owned()).collect::<String>();
        // }
        // output
        format!("{:#?}", lexer.tokens)
    }

    macro_rules! snapshot {
        ($name:tt, $path:tt) => {
            #[test]
            fn $name() {
                let mut settings = insta::Settings::clone_current();
                settings.set_snapshot_path("../testdata/output/");
                settings.bind(|| {
                    insta::assert_snapshot!(snapshot($path));
                });
            }
        };
    }

    snapshot!(test_load_file, "./testdata/tests/test1.md");
}
