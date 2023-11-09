use std::{collections::HashMap, fmt::Display};

use crate::{errors::RuntimeError, lox_class::LoxClass, token::Token, value::Value};

#[derive(Clone, PartialEq)]
pub struct LoxInstance {
    class: LoxClass,
    fields: HashMap<String, Value>,
}

impl LoxInstance {
    pub fn new(class: &LoxClass) -> Self {
        Self {
            class: class.clone(),
            fields: HashMap::new(),
        }
    }

    pub fn get(&self, name: &Token) -> Result<Value, RuntimeError> {
        match self.fields.get(&name.lexeme).cloned() {
            Some(v) => Ok(v),
            None => Err(RuntimeError::new(
                name.clone(),
                format!("Undefined property '{}'", name.lexeme),
            )),
        }
    }

    pub fn set(&mut self, name: &Token, value: &Value) {
        self.fields.insert((&name.lexeme).into(), value.clone());
    }
}

impl Display for LoxInstance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} instance", self.class)
    }
}
