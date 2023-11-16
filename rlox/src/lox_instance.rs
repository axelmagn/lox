use std::{cell::RefCell, collections::HashMap, fmt::Display, rc::Rc};

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

    pub fn get(instance: Rc<RefCell<LoxInstance>>, name: &Token) -> Result<Value, RuntimeError> {
        if let Some(v) = instance.borrow().fields.get(&name.lexeme) {
            return Ok(v.clone());
        }

        if let Some(method) = instance.borrow().class.find_method(&name.lexeme) {
            return Ok(method.bind(instance.clone()).into());
        }

        Err(RuntimeError::new(
            name.clone(),
            format!("Undefined property '{}'", name.lexeme),
        ))
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
