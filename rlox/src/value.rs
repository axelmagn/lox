use crate::token::TokenLiteral;

/// Value of an evaluated expression
#[derive(Debug, Clone)]
pub enum Value {
    Nil,
    String(String),
    Number(f64),
    Bool(bool),
}

impl From<TokenLiteral> for Value {
    fn from(literal: TokenLiteral) -> Self {
        match literal {
            TokenLiteral::Nil => Self::Nil,
            TokenLiteral::String(v) => Self::String(v),
            TokenLiteral::Number(v) => Self::Number(v),
            TokenLiteral::Bool(v) => Self::Bool(v),
        }
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Self::Number(value)
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Nil, Self::Nil) => true,
            (Self::Nil, _) => false,
            (Self::String(l0), Self::String(r0)) => l0 == r0,
            (Self::Number(l0), Self::Number(r0)) => l0 == r0,
            (Self::Bool(l0), Self::Bool(r0)) => l0 == r0,
            _ => false,
        }
    }
}