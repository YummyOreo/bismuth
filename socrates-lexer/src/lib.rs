#![allow(unused)]
use socrates_md::MarkdownFile;
use std::ops::Range;

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

    pub fn peek_till_diff(&self) -> usize {
        let mut chars = self.chars.split_at(self.position).1.iter();
        println!("{:#?}", chars);
        let len = chars.len();

        chars
            .position(|c| c != self.chars.get(self.position).unwrap())
            .unwrap_or(len)
            + self.position
    }

    pub fn get_range(&self, range: Range<usize>) -> Vec<char> {
        self.chars[range].to_vec()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_peek_till() {
        let file = MarkdownFile {
            content: "this is a test: aaaaab".to_string(),
            path: PathBuf::new(),
        };

        let mut lexer = Lexer::new(&file);
        lexer.move_to(16);

        assert_eq!(lexer.peek_till_diff(), lexer.chars.len() - 1);

        let file = MarkdownFile {
            content: "this is a test: aaaaaa".to_string(),
            path: PathBuf::new(),
        };

        let mut lexer = Lexer::new(&file);
        lexer.move_to(16);

        assert_eq!(lexer.peek_till_diff(), lexer.chars.len());
    }

    #[test]
    fn test_range() {
        let file = MarkdownFile {
            content: "this is a test: aaaaaa".to_string(),
            path: PathBuf::new(),
        };

        let mut lexer = Lexer::new(&file);
        lexer.move_to(16);

        assert_eq!(
            lexer.get_range(16..lexer.peek_till_diff()),
            vec!['a', 'a', 'a', 'a', 'a', 'a']
        );

        let file = MarkdownFile {
            content: "this is a test: aaaaab".to_string(),
            path: PathBuf::new(),
        };

        let mut lexer = Lexer::new(&file);
        lexer.move_to(16);

        assert_eq!(
            lexer.get_range(16..lexer.peek_till_diff()),
            vec!['a', 'a', 'a', 'a', 'a']
        );
    }
}
