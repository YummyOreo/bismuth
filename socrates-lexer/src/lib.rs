#![allow(unused)]
use socrates_md::MarkdownFile;
use std::ops::RangeInclusive;

pub struct Token {
    start: [usize; 2],
    end: [usize; 2],
}

pub enum TokenType {
    Text(Token),

    Asterisk(Token),
    Underscore(Token),

    Hash(Token),

    BracketLeft(Token),
    BracketRight(Token),
    ParenthesisLeft(Token),
    ParenthesisRight(Token),
    Exclamation(Token),

    GreaterThan(Token),

    Dash(Token),

    Tilda(Token),

    CurlybraceLeft(Token),
    CurlybraceRight(Token),

    Fontmatter(Token),
}

pub struct Lexer<'a> {
    file: &'a MarkdownFile,
    chars: Vec<char>,
    position: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(file: &'a MarkdownFile) -> Self {
        Lexer {
            file,
            chars: file.content.chars().collect::<Vec<char>>(),
            position: 0,
        }
    }

    pub fn move_to(&mut self, pos: usize) -> Option<&char> {
        if pos >= self.chars.len() {
            return None;
        }

        self.position = pos;
        return self.peek(0);
    }

    pub fn peek(&self, next: usize) -> Option<&char> {
        self.chars.get(self.position + next)
    }

    pub fn peek_till_diff(&self) -> std::ops::RangeInclusive<usize> {
        let mut chars = self.chars.split_at(self.position).1.iter();
        let len = chars.len();

        self.position
            ..=chars
                .position(|c| c != self.chars.get(self.position).unwrap())
                .unwrap_or(len)
                + self.position
                - 1
    }

    pub fn get_range(&self, range: RangeInclusive<usize>) -> Vec<char> {
        self.chars[range].to_vec()
    }
}

#[cfg(test)]
mod test {
    use super::*;
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
        let file = &setup("this is a test aaaaab");
        let mut lexer = Lexer::new(file);
        lexer.move_to(15);

        assert_eq!(lexer.peek_till_diff(), 15..=lexer.chars.len() - 2);

        let file = &setup("this is a test aaaaaa");
        let mut lexer = Lexer::new(file);
        lexer.move_to(15);

        assert_eq!(lexer.peek_till_diff(), 15..=lexer.chars.len() - 1);
    }

    #[test]
    fn test_range() {
        let file = &setup("this is a test aaaaaa");
        let mut lexer = Lexer::new(file);
        lexer.move_to(15);

        assert_eq!(
            lexer.get_range(lexer.peek_till_diff()),
            vec!['a', 'a', 'a', 'a', 'a', 'a']
        );

        let file = &setup("this is a test aaaaab");
        let mut lexer = Lexer::new(file);
        lexer.move_to(15);

        let num = lexer.peek_till_diff();

        assert_eq!(
            lexer.get_range(lexer.peek_till_diff()),
            vec!['a', 'a', 'a', 'a', 'a']
        );
    }
}
