pub mod error;
mod lexer;

pub use lexer::Lexer;

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub len: u32,
}

impl Token {
    pub fn new(kind: TokenKind, len: u32) -> Self {
        Self { kind, len }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TokenKind {
    Comment,

    WhiteSpace,

    Ident,

    Literal {
        kind: LiteralKind,
        start: i32,
    },

    Semi,

    Comma,

    Dot,

    // =>
    FatArrow,

    // return
    FnReturn,

    ///  (
    OpenParen,

    /// )
    CloseParen,

    /// [
    OpenBracket,

    /// ]
    CloseBracket,

    /// {
    OpenBrace, // {

    /// }
    CloseBrace,

    /// @
    At,

    Question,

    Colon,

    Eq,

    Bang,

    Lt,

    Gt,

    Minus,

    And,

    Or,

    Plus,

    Star,

    Slash,

    Caret,

    Percent,

    Unknown,

    Eof,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum LiteralKind {
    // 1, 3, 4, 0b100, 0o10, 0xff
    Int {
        base: Base,
        empty: bool,
    },

    // 1.2 .2, 0.3
    Float {
        base: Base,
        empty_exponent: bool,
    },

    // "Hello" , 'world'
    Str {
        terminated: bool,
    },

    /// `Hello {name}`
    FormatStr {
        terminated: bool,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Base {
    /// Literal starts with "0b".
    Binary = 2,
    /// Literal starts with "0o".
    Octal = 8,
    /// Literal doesn't contain a prefix.
    Decimal = 10,
    /// Literal starts with "0x".
    Hexadecimal = 16,
}

pub fn tokenize(input: &str) -> impl Iterator<Item = Token> + '_ {
    #[allow(unused)]
    let mut tokenizer = Lexer::new(input);
    std::iter::from_fn(move || {
        let token = tokenizer.consume();
        if token.kind != TokenKind::Eof {
            Some(token)
        } else {
            None
        }
    })
}

pub fn is_whitespace(c: char) -> bool {
    matches!(
        c,
        '\u{0009}'   // \t
      | '\u{000A}' // \n
      | '\u{000B}' // vertical tab
      | '\u{000C}' // form feed
      | '\u{000D}' // \r
      | '\u{0020}' // space

      // NEXT LINE from latin1
      | '\u{0085}'

      | '\u{200E}' // LEFT-TO-RIGHT MARK
      | '\u{200F}' // RIGHT-TO-LEFT MARK

      // Dedicated whitespace characters from Unicode
      | '\u{2028}' // LINE SEPARATOR
      | '\u{2029}' // PARAGRAPH SEPARATOR
    )
}

pub fn is_ident_start(c: char) -> bool {
    c == '_' || unicode_xid::UnicodeXID::is_xid_start(c)
}

pub fn can_continue_ident(c: char) -> bool {
    unicode_xid::UnicodeXID::is_xid_continue(c)
}

use LiteralKind::*;
use TokenKind::*;
impl Lexer<'_> {
    pub fn consume(&mut self) -> Token {
        let cur_char = match self.eat() {
            Some(c) => c,
            None => return Token::new(TokenKind::Eof, 0),
        };

        let token_kind = match cur_char {
            '#' => self.comment(),
            c if is_whitespace(c) => self.whitespace(),

            '=' => match self.peek_next() {
                '>' => {
                    self.eat();
                    FatArrow
                }
                _ => TokenKind::Eq,
            },

            // consume string literals
            c @ '"' | c @ '\'' | c @ '`' => self.string_literal(c),

            c if is_ident_start(c) => self.identifier_or_unknown(),

            c @ '0'..='9' => {
                let kind = self.number(c);
                // todo add start
                Literal { kind, start: 0 }
            }

            '+' => Plus,
            '-' => Minus,
            '*' => Star,
            '/' => Slash,
            '%' => Percent,
            '^' => Caret,
            '.' => Dot,
            '!' => Bang,

            '&' => And,
            '|' => Or,

            '@' => At,

            '(' => OpenParen,
            ')' => CloseParen,
            '[' => OpenBracket,
            ']' => CloseBracket,
            '{' => OpenBrace,
            '}' => CloseBrace,

            '?' => Question,
            ':' => Colon,

            ';' => Semi,
            ',' => Comma,
            '<' => TokenKind::Lt,
            '>' => TokenKind::Gt,

            _ => Unknown,
        };

        Token {
            kind: token_kind,
            len: 0,
        }
    }

    fn number(&mut self, first_digit: char) -> LiteralKind {
        debug_assert!(
            first_digit.is_digit(10),
            "the first digit should be a valid base 10 digit"
        );
        let mut base = Base::Decimal;
        if first_digit == '0' {
            match self.peek_next() {
                'b' => {
                    base = Base::Binary;
                    self.eat();
                    if !self.consume_decimal_digits() {
                        return Int { base, empty: true };
                    }
                }
                'o' => {
                    base = Base::Octal;
                    self.eat();
                    if !self.consume_decimal_digits() {
                        return Int { base, empty: true };
                    }
                }
                'x' => {
                    base = Base::Hexadecimal;

                    self.eat();
                    if !self.consume_hex_digits() {
                        return Int { base, empty: true };
                    }
                }

                '0'..='9' | '_' => {
                    self.consume_decimal_digits();
                }

                '.' | 'e' | 'E' => {}

                _ => return Int { base, empty: false },
            }
        } else {
            self.consume_decimal_digits();
        } // End of integer

        // Check for float or exponent
        match self.peek_next() {
            '.' if !is_ident_start(self.peek_next_2()) => {
                self.eat();

                let mut empty_exponent = true;

                if self.peek_next().is_digit(10) {
                    self.consume_decimal_digits();

                    match self.peek_next() {
                        'e' | 'E' => {
                            self.eat();
                            empty_exponent = !self.consume_float_exponent();
                        }
                        _ => (),
                    }
                }
                Float {
                    base,
                    empty_exponent,
                }
            }
            'e' | 'E' => {
                self.eat();
                let empty_exponent = !self.consume_float_exponent();
                Float {
                    base,
                    empty_exponent,
                }
            }
            _ => Int { base, empty: false },
        }
    }

    fn consume_float_exponent(&mut self) -> bool {
        if self.peek_next() == '-' || self.peek_next() == '+' {
            self.eat();
        }

        self.consume_decimal_digits()
    }

    fn consume_decimal_digits(&mut self) -> bool {
        let mut is_empty = true;
        loop {
            match self.peek_next() {
                '_' => {
                    // consume separator
                    self.eat();
                }
                '0'..='9' => {
                    is_empty = false;
                    self.eat();
                }
                _ => break,
            }
        }
        !is_empty
    }

    fn consume_hex_digits(&mut self) -> bool {
        let mut is_empty = true;
        loop {
            match self.peek_next() {
                '_' => {
                    self.eat();
                }
                '0'..='9' | 'a'..='f' | 'A'..='F' => {
                    is_empty = false;
                    self.eat();
                }
                _ => break,
            }
        }
        !is_empty
    }

    fn string_literal(&mut self, str_type: char) -> TokenKind {
        let is_format_str = str_type == '`';
        let terminated = self.consume_str_literal(str_type);
        let kind = if is_format_str {
            FormatStr { terminated }
        } else {
            Str { terminated }
        };

        // TODO: implement start
        Literal { kind, start: 0 }
    }

    fn consume_str_literal(&mut self, str_type: char) -> bool {
        debug_assert!(
            matches!(str_type, '\'' | '"' | '`'),
            "Expected one of [\", ', `]"
        );

        while let Some(c) = self.eat() {
            match c {
                _ if c == str_type => return true,
                '\\' if self.peek_next() == '\\' || self.peek_next() == str_type => {
                    self.eat();
                }
                _ => (),
            };
        }
        false
    }

    fn identifier_or_unknown(&mut self) -> TokenKind {
        self.eat_while(can_continue_ident);

        Ident
    }

    fn whitespace(&mut self) -> TokenKind {
        self.eat_while(is_whitespace);
        TokenKind::WhiteSpace
    }

    fn comment(&mut self) -> TokenKind {
        self.eat_while(|c| c != '\n');
        self.eat(); // eat the \n
        TokenKind::Comment
    }
}
