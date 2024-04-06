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

    Band,

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
    // Integer
    Int { base: Base },

    // Floating point
    Float { base: Base },

    // Strings
    Str {},
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

pub fn tokenize(input: &str) {
    #[allow(unused)]
    let tokenizer = Lexer::new(input);
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
