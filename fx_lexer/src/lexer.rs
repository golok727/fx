use std::str::Chars;

pub(crate) const EOF_CHAR: char = '\0';

#[derive(Debug)]

pub struct Lexer<'a> {
    remaining_tokens: usize,
    chars: Chars<'a>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            remaining_tokens: input.len(),
            chars: input.chars(),
        }
    }

    pub fn as_str(&self) -> &'a str {
        self.chars.as_str()
    }

    pub fn peek_next(&self) -> char {
        self.chars.clone().next().unwrap_or(EOF_CHAR)
    }

    pub fn peek_next_2(&self) -> char {
        let mut iter = self.chars.clone();
        iter.next();
        iter.next().unwrap_or(EOF_CHAR)
    }

    pub fn peek_next_3(&self) -> char {
        let mut iter = self.chars.clone();
        iter.next();
        iter.next();
        iter.next().unwrap_or(EOF_CHAR)
    }

    pub(crate) fn is_eof(&self) -> bool {
        self.chars.as_str().is_empty()
    }

    pub(crate) fn eat(&mut self) -> Option<char> {
        let c = self.chars.next()?;
        Some(c)
    }

    pub(crate) fn get_cur_range(&self) -> u32 {
        (self.remaining_tokens - self.chars.as_str().len()) as u32
    }

    pub(crate) fn reset_range(&mut self) {
        self.remaining_tokens = self.chars.as_str().len();
    }

    pub(crate) fn eat_while(&mut self, mut cb: impl FnMut(char) -> bool) {
        while cb(self.peek_next()) && !self.is_eof() {
            self.eat();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT: &str = "abcd";

    #[test]
    pub fn first_second_third() {
        let lex = Lexer::new(&INPUT);
        assert_eq!(lex.peek_next(), 'a');
        assert_eq!(lex.peek_next_2(), 'b');
        assert_eq!(lex.peek_next_3(), 'c');

        assert_eq!(lex.as_str(), INPUT);
    }

    #[test]
    pub fn eat() {
        let mut lex = Lexer::new(&INPUT);
        assert_eq!(lex.eat(), Some('a'));
        assert_eq!(lex.as_str(), "bcd");

        assert_eq!(lex.eat(), Some('b'));
        assert_eq!(lex.eat(), Some('c'));
        assert_eq!(lex.eat(), Some('d'));
        assert_eq!(lex.eat(), None);
        assert_eq!(lex.as_str(), "");
    }

    #[test]
    pub fn eat_while() {
        use crate::is_whitespace;
        let input = "      a     b     c";
        let mut res = String::from("");
        let mut lex = Lexer::new(&input);
        lex.eat_while(is_whitespace);
        res.push_str(&lex.eat().unwrap_or_default().to_string());

        lex.eat_while(is_whitespace);
        res.push_str(&lex.eat().unwrap_or_default().to_string());

        lex.eat_while(is_whitespace);
        res.push_str(&lex.eat().unwrap_or_default().to_string());

        assert_eq!(res, "abc");
    }
}
