use core::fmt;
use std::hash::Hash;

use ordered_float::OrderedFloat;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Token {
    pub ttype: TokenType,
    pub lexeme: String,
    pub literal: TokenLiteral,
    pub line: usize,
    pub cursor: usize,
}

impl Token {
    pub fn new(
        ttype: TokenType,
        lexeme: &str,
        literal: &TokenLiteral,
        line: usize,
        cursor: usize,
    ) -> Self {
        Self {
            ttype,
            lexeme: lexeme.into(),
            literal: literal.clone(),
            line,
            cursor,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.ttype, self.lexeme, self.literal)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // one or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    String,
    Number,

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    // end of file.
    EOF,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TokenLiteral {
    Nil,
    String(String),
    Number(OrderedFloat<f64>),
    Bool(bool),
}

impl fmt::Display for TokenLiteral {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenLiteral::Nil => f.write_str("nil"),
            TokenLiteral::String(s) => f.write_str(s),
            TokenLiteral::Number(n) => write!(f, "{}", n),
            TokenLiteral::Bool(b) => write!(f, "{}", b),
        }
    }
}
