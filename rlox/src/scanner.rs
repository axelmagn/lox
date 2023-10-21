use lazy_static::lazy_static;
use std::collections::HashMap;

use crate::{
    lox::Lox,
    token::{Token, TokenLiteral, TokenType},
};

lazy_static! {
    static ref KEYWORDS: HashMap<String, TokenType> = {
        use crate::token::TokenType::*;
        let mut keywords = HashMap::new();
        keywords.insert("and".into(), And);
        keywords.insert("class".into(), Class);
        keywords.insert("else".into(), Else);
        keywords.insert("false".into(), False);
        keywords.insert("for".into(), For);
        keywords.insert("if".into(), If);
        keywords.insert("nil".into(), Nil);
        keywords.insert("or".into(), Or);
        keywords.insert("print".into(), Print);
        keywords.insert("return".into(), Return);
        keywords.insert("super".into(), Super);
        keywords.insert("this".into(), This);
        keywords.insert("true".into(), True);
        keywords.insert("var".into(), Var);
        keywords.insert("while".into(), While);
        keywords
    };
}

pub struct Scanner {
    source: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.chars().collect(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        let token = Token::new(TokenType::EOF, "", &TokenLiteral::None, self.line);
        self.tokens.push(token);
        self.tokens.clone()
    }

    fn scan_token(&mut self) {
        use TokenType::*;
        let c = self.advance();
        match c {
            '(' => self.add_token(LeftParen),
            ')' => self.add_token(RightParen),
            '{' => self.add_token(LeftBrace),
            '}' => self.add_token(RightBrace),
            ',' => self.add_token(Comma),
            '.' => self.add_token(Dot),
            '-' => self.add_token(Minus),
            '+' => self.add_token(Plus),
            ';' => self.add_token(Semicolon),
            '*' => self.add_token(Star),
            '!' => {
                let token = if self.try_match('=') { BangEqual } else { Bang };
                self.add_token(token);
            }
            '=' => {
                let token = if self.try_match('=') {
                    EqualEqual
                } else {
                    Equal
                };
                self.add_token(token);
            }
            '<' => {
                let token = if self.try_match('=') { LessEqual } else { Less };
                self.add_token(token);
            }
            '>' => {
                let token = if self.try_match('=') {
                    GreaterEqual
                } else {
                    Greater
                };
                self.add_token(token);
            }
            '/' => {
                if self.try_match('/') {
                    // a comment goes until the end of the line
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else if self.try_match('*') {
                    while !self.is_at_end() && self.peek() != '*' && self.peek_next() != '/' {
                        self.advance();
                    }
                    // consume closing `*/`
                    self.advance();
                    self.advance();
                } else {
                    self.add_token(Slash);
                }
            }
            // ignore whitespace
            ' ' | '\r' | '\t' => {}
            '\n' => self.line += 1,
            '"' => self.scan_string(),
            _ => {
                if c.is_digit(10) {
                    self.scan_number();
                } else if c.is_alphabetic() {
                    self.scan_identifier();
                } else {
                    Lox::error(self.line, &format!("Unexpected character: {}", c));
                }
            }
        };
    }

    fn scan_string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            Lox::error(self.line, "Unterminated string.");
            return;
        }

        // the closing ".
        self.advance();

        let value: String = self.source[self.start + 1..self.current - 1]
            .iter()
            .collect();
        self.add_token_literal(TokenType::String, &TokenLiteral::String(value));
    }

    fn scan_number(&mut self) {
        while self.peek().is_digit(10) {
            self.advance();
        }

        // look for a fractional part.
        if self.peek() == '.' && self.peek_next().is_digit(10) {
            // consume the '.'
            self.advance();
        }

        while self.peek().is_digit(10) {
            self.advance();
        }

        let value_str: String = self.source[self.start..self.current].iter().collect();
        let value: f64 = value_str.parse::<f64>().unwrap();
        self.add_token_literal(TokenType::Number, &TokenLiteral::Number(value))
    }

    fn scan_identifier(&mut self) {
        while self.peek().is_alphanumeric() {
            self.advance();
        }

        let text: String = self.source[self.start..self.current].iter().collect();
        let ttype = *KEYWORDS.get(&text).unwrap_or(&TokenType::Identifier);
        self.add_token(ttype);
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        let out = self.source[self.current];
        self.current += 1;
        out
    }

    fn add_token(&mut self, ttype: TokenType) {
        self.add_token_literal(ttype, &TokenLiteral::None);
    }

    fn add_token_literal(&mut self, ttype: TokenType, literal: &TokenLiteral) {
        let text: String = self.source[self.start..self.current].iter().collect();
        self.tokens
            .push(Token::new(ttype, &text, literal, self.line));
    }

    fn try_match(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.source[self.current] != expected {
            return false;
        }

        self.current += 1;
        return true;
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source[self.current]
        }
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            '\0'
        } else {
            self.source[self.current + 1]
        }
    }
}
