use crate::{token::Token, value::Value};

#[derive(Clone)]
pub struct RuntimeError {
    pub token: Token,
    pub msg: String,
    pub return_value: Option<Value>,
}

impl RuntimeError {
    pub fn new(token: Token, msg: String) -> Self {
        Self {
            token,
            msg,
            return_value: None,
        }
    }

    pub fn new_return(token: Token, msg: String, return_value: Option<Value>) -> Self {
        Self {
            token,
            msg,
            return_value,
        }
    }
}
