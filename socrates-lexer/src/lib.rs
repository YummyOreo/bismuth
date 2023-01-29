#![allow(unused_doc_comments, dead_code)]
use regex::Regex;
use socrates_md::MarkdownFile;
use std::ops::RangeInclusive;

pub mod error;
pub mod token;
use crate::error::LexerError;

pub struct Lexer {
    pub file: MarkdownFile,
    chars: Vec<char>,
    position: usize,

    current_token: token::Token,
    pub tokens: Vec<token::Token>,
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

    pub fn get_lines(&self) -> Vec<(usize, String)> {
        let mut curr_line = String::new();
        let mut lines: Vec<(usize, String)> = Vec::new();
        let mut line_index = 0;
        for (i, c) in self.chars.iter().enumerate() {
            if c == &'\n' {
                curr_line.push('\n');
                lines.push((line_index, curr_line));
                curr_line = String::new();
                line_index = i + 1;
            } else {
                curr_line.push(*c);
            }
        }
        lines
    }

    fn current(&self) -> Result<&char, LexerError> {
        self.chars
            .get(self.position)
            .ok_or(LexerError::GetCharError)
    }

    fn new_token(&mut self, t: token::Token) {
        self.tokens.push(self.current_token.clone());
        self.current_token = t;
    }

    fn move_to(&mut self, pos: usize) -> Result<&char, LexerError> {
        if pos >= self.chars.len() {
            return Err(LexerError::MoveError(pos));
        }

        self.position = pos;
        return self.current();
    }

    fn next(&mut self) -> Result<&char, LexerError> {
        if self.position >= self.chars.len() {
            return Err(LexerError::MoveError(self.position + 1));
        }

        self.position += 1;
        return self.current();
    }

    fn next_line(&mut self) -> usize {
        let mut chars = self.chars.split_at(self.position).1.iter();
        let len = chars.len();
        chars.position(|c| c == &'\n').unwrap_or(len) + self.position
    }

    fn peek(&self, next: usize) -> Result<&char, LexerError> {
        self.peek_at(self.position + next)
    }

    fn peek_back(&self, next: usize) -> Result<&char, LexerError> {
        self.peek_at(self.position - next)
    }

    fn peek_at(&self, index: usize) -> Result<&char, LexerError> {
        self.chars.get(index).ok_or(LexerError::PeekError(index))
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
                .position(|c| c != self.peek_at(self.position).expect("Should be at a char"))
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

    fn make_token_at_pos(&self, kind: token::TokenType) -> Result<token::Token, LexerError> {
        Ok(token::Token::new(
            kind,
            vec![*self.current()?],
            self.position,
            self.position,
        ))
    }

    fn handle_hash(&self) -> Result<token::Token, LexerError> {
        let diff = self.peek_till_diff();
        let next = self.peek_at(diff.end() + 1).unwrap_or(&'a');

        if let ' ' = next {
            self.make_token_at_pos(token::TokenType::Hash)
        } else {
            self.make_token_at_pos(token::TokenType::Text)
        }
    }

    fn handle_exclamation(&self) -> Result<token::Token, LexerError> {
        let diff = self.peek_till_diff();
        let next = self.peek_at(diff.end() + 1).unwrap_or(&'a');

        if let ' ' = next {
            self.make_token_at_pos(token::TokenType::Text)
        } else {
            self.make_token_at_pos(token::TokenType::Exclamation)
        }
    }

    fn handle_greaterthan(&self) -> Result<token::Token, LexerError> {
        if self.current_token.kind == token::TokenType::EndOfLine {
            let peek = self.peek(1)?;
            if peek == &' ' {
                return self.make_token_at_pos(token::TokenType::GreaterThan);
            }
        }
        self.make_token_at_pos(token::TokenType::Text)
    }

    // Fontmatter

    fn get_fm_start(&self) -> Option<RangeInclusive<usize>> {
        /// Checks if the current position is the start of the fontmatter
        /// if it is, it will return the range of the fontmatter
        // gets till change in char
        let diff = self.peek_till_diff();
        let diff_end = *diff.end();

        if diff_end - self.position == 2 {
            return Some(diff);
        }
        None
    }

    fn append_fm_start_token(&mut self, start: usize, end: usize) -> Result<(), LexerError> {
        // makes the fontmatter start token
        let fontmatter_start_token = token::Token {
            start,
            end,
            kind: token::TokenType::FontmatterStart,
            text: vec!['-', '-', '-'],
        };

        // appends the token
        self.new_token(fontmatter_start_token);

        // moves past the token
        self.move_to(end + 1)?;

        Ok(())
    }

    fn append_fm_inside(&mut self, inside: RangeInclusive<usize>) -> Result<(), LexerError> {
        let start = *inside.start();
        let end = *inside.end();
        self.move_to(start)?;

        // let fm_txt = self.get_range(start..=end);

        loop {
            let next_line = self.next_line();

            if next_line > end {
                break;
            }

            let inside_text = self.get_range(self.position..=next_line - 1);
            if !inside_text.is_empty() {
                let t = token::Token {
                    start: self.position,
                    end: next_line - 1,
                    kind: token::TokenType::FontmatterInside,
                    text: inside_text,
                };
                self.new_token(t);
            }

            let eol = token::Token {
                start: next_line,
                end: next_line,
                kind: token::TokenType::EndOfLine,
                text: vec!['\n'],
            };
            self.new_token(eol);
            self.move_to(next_line + 1)?;
        }

        // moves to after the inside of the fontmatter
        self.move_to(end + 1)?;
        Ok(())
    }

    fn get_fm_end(&self) -> Option<RangeInclusive<usize>> {
        let fontmatter_end = self.peek_regex(Regex::new("---\n").expect("Should be valid regex"));
        if fontmatter_end.start() == fontmatter_end.end() {
            None
        } else {
            Some(*fontmatter_end.start()..=*fontmatter_end.end() - 1)
        }
    }

    fn handle_fontmatter(&mut self) -> Result<token::Token, LexerError> {
        // use self.peek_till_diff and self.peek_till

        // ie self.peek_till_diff to know when the --- end and to test if there are 3
        // and self.regex("^---$") to know when the end is there (and get how far away it is)

        // and self.get_range to get the stuff inbetween, from the end of the first ---, and the
        // start of the last ---
        if let Some(diff) = self.get_fm_start() {
            let diff_end = *diff.end();

            self.append_fm_start_token(self.position, diff_end)?;

            let fontmatter_end = self.get_fm_end().ok_or(LexerError::FontmatterError)?;

            self.append_fm_inside(diff_end + 2..=*fontmatter_end.start() - 1)?;

            // appending fm_end

            // Moves to the end of the fontmatter
            self.move_to(*fontmatter_end.end())?;

            return Ok(token::Token {
                start: *fontmatter_end.start(),
                end: *fontmatter_end.end(),
                kind: token::TokenType::FontmatterEnd,
                text: vec!['-', '-', '-'],
            });
        }
        self.make_token_at_pos(token::TokenType::Text)
    }

    fn handle_dash(&mut self) -> Result<token::Token, LexerError> {
        if self.current_token.kind == token::TokenType::StartOfFile {
            return self.handle_fontmatter();
        }

        let before_after = (self.peek_back(1)?, self.peek(1)?);

        match before_after {
            (&'\n', &'-') => {
                let diff = *self.peek_till_diff().end();
                if diff == self.position + 2 && self.peek_at(diff + 1)? == &'\n' {
                    let t = token::Token {
                        start: self.position,
                        end: diff,
                        kind: token::TokenType::Dash,
                        text: vec!['-', '-', '-'],
                    };
                    self.move_to(self.position + 2)?;
                    return Ok(t);
                }
            }
            (_, &' ') => {
                return self.make_token_at_pos(token::TokenType::Dash);
            }
            _ => {}
        }

        self.make_token_at_pos(token::TokenType::Text)
    }

    fn handle_whitespace(&mut self) -> Result<token::Token, LexerError> {
        match self.current_token.kind {
            token::TokenType::Whitespace | token::TokenType::EndOfLine => {
                self.make_token_at_pos(token::TokenType::Whitespace)
            }
            _ => self.make_token_at_pos(token::TokenType::Text),
        }
    }

    fn handle_number(&mut self) -> Result<token::Token, LexerError> {
        let arround = (self.peek_back(1)?, self.peek(1)?);

        match arround {
            (&'\n', _) => {
                let till_dot =
                    self.peek_regex(Regex::new(r"\d*\.").expect("Should be valid regex"));
                let end = *till_dot.end();
                let text = self.get_range(till_dot);
                let t = token::Token {
                    start: self.position,
                    end,
                    kind: token::TokenType::ListNumber,

                    text,
                };
                self.move_to(end)?;
                Ok(t)
            }
            (_, _) => self.make_token_at_pos(token::TokenType::Text),
        }
    }

    fn match_char(&mut self) -> Result<token::Token, LexerError> {
        let c = self.current()?;
        if c.is_numeric() {
            return self.handle_number();
        }
        match c {
            '\n' => self.make_token_at_pos(token::TokenType::EndOfLine),

            // could be text or whitespace
            ' ' => self.handle_whitespace(),

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

            '%' => self.make_token_at_pos(token::TokenType::Percent),

            _ => self.make_token_at_pos(token::TokenType::Text),
        }
    }

    fn read_char(&mut self) -> Result<(), error::LexerError> {
        // get the token (this will get edge cases, ie. `# ` is different from `#t` (one is text,
        // one is Hash))
        // if the token type is the same as the current token type
        // append it the the current token
        let token = self.match_char()?;

        // we don't want to combine EOL tokens
        if token.kind == token::TokenType::EndOfLine {
            self.new_token(token);
        } else if token.kind == self.current_token.kind {
            self.current_token.append(token.text);
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
            if self.next().is_err() {
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
        lexer.move_to(15).unwrap();

        assert_eq!(lexer.peek_till(&'c'), 15..=lexer.chars.len() - 2);

        let file = setup("this is a test aaaaaab");
        let mut lexer = Lexer::new(file);
        lexer.move_to(15).unwrap();

        assert_eq!(lexer.peek_till(&'b'), 15..=lexer.chars.len() - 2);
    }

    #[test]
    fn test_peek_till_diff() {
        let file = setup("this is a test aaaaab");
        let mut lexer = Lexer::new(file);
        lexer.move_to(15).unwrap();

        assert_eq!(lexer.peek_till_diff(), 15..=lexer.chars.len() - 2);

        let file = setup("this is a test aaaaaa");
        let mut lexer = Lexer::new(file);
        lexer.move_to(15).unwrap();

        assert_eq!(lexer.peek_till_diff(), 15..=lexer.chars.len() - 1);
    }

    #[test]
    fn test_range() {
        let file = setup("this is a test aaaaaa");
        let mut lexer = Lexer::new(file);
        lexer.move_to(15).unwrap();

        assert_eq!(
            lexer.get_range(lexer.peek_till_diff()),
            vec!['a', 'a', 'a', 'a', 'a', 'a']
        );

        let file = setup("this is a test aaaaab");
        let mut lexer = Lexer::new(file);
        lexer.move_to(15).unwrap();

        assert_eq!(
            lexer.get_range(lexer.peek_till_diff()),
            vec!['a', 'a', 'a', 'a', 'a']
        );
    }

    #[test]
    fn test_nexn_line() {
        let file = setup("this is a test aaaaaa \n this is a test");
        let mut lexer = Lexer::new(file);
        lexer.move_to(15).unwrap();

        let num = lexer.next_line();
        assert_eq!(22, num);
    }

    #[test]
    fn test_regex() {
        let file = setup("This is a test for my test, email@test.com");
        let mut lexer = Lexer::new(file);
        lexer.move_to(27).unwrap();

        let re = Regex::new("([a-zA-Z0-9]+)@([a-zA-Z]*).([a-z]+)").unwrap();
        assert_eq!(lexer.peek_regex(re), 28..=41);

        let file = setup("This is a test \n for myttest, ");
        let mut lexer = Lexer::new(file);
        lexer.move_to(21).unwrap();

        let re = Regex::new(r"(est,\s$)").unwrap();
        assert_eq!(lexer.peek_regex(re), 25..=29);
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

        let tokens = lexer.tokens.clone();
        if tokens[0].kind != token::TokenType::StartOfFile
            || tokens[tokens.len() - 1].kind != token::TokenType::EndOfFile
        {
            panic!("There should be a StartOfFile and EndOfFile token at the start and end");
        }

        let mut output = String::new();
        let mut token_ptr = 0;

        for (index, line) in lexer.get_lines() {
            let line = line.replace('\n', "↲\n");
            output += &line;

            while token_ptr < tokens.len() {
                let t = &tokens[token_ptr];
                token_ptr += 1;

                if t.kind == token::TokenType::StartOfFile || t.kind == token::TokenType::EndOfFile
                {
                    continue;
                }

                output += &" ".repeat(t.start - index);

                let mut repeat_num = t.end - t.start;
                repeat_num += 1;

                // cap at 100, because i don't want to use a lib just to get the terminal width
                if repeat_num > 100 {
                    repeat_num = 100;
                }
                output += &"^".repeat(repeat_num);

                output += &format!(" {t}");
                output += "\n";
                match t.kind {
                    token::TokenType::EndOfLine | token::TokenType::FontmatterStart => {
                        break;
                    }
                    _ => {}
                }
            }
        }

        output
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
    snapshot!(test_load_file_1, "./testdata/tests/test2.md");
    snapshot!(test_load_file_2, "./testdata/tests/test3.md");
    snapshot!(fontmatter_test, "./testdata/tests/test_fontmatter.md");
    snapshot!(fontmatter_test_1, "./testdata/tests/test_fontmatter_2.md");
}
