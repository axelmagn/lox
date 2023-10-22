use crate::token::Token;

#[derive(Debug, Clone)]
pub struct RuntimeError {
    pub token: Token,
    pub msg: String,
}

impl RuntimeError {
    pub fn new(token: Token, msg: String) -> Self {
        Self { token, msg }
    }
}
