use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{errors::RuntimeError, token::Token, value::Value};

pub struct Environment {
    enclosing: Option<Rc<RefCell<Environment>>>,
    values: HashMap<String, Value>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            enclosing: None,
            values: HashMap::new(),
        }
    }

    pub fn with_enclosing(enclosing: Rc<RefCell<Environment>>) -> Self {
        Self {
            enclosing: Some(enclosing),
            values: HashMap::new(),
        }
    }

    pub fn get(&self, name: &Token) -> Result<Value, RuntimeError> {
        if self.values.contains_key(&name.lexeme) {
            return Ok(self.values[&name.lexeme].clone());
        }
        if self.enclosing.is_some() {
            return self.enclosing.as_ref().unwrap().borrow().get(name);
        }
        Err(RuntimeError::new(
            name.clone(),
            format!("Undefined variable '{}'.", name.lexeme),
        ))
    }

    pub fn assign(&mut self, name: &Token, value: Value) -> Result<(), RuntimeError> {
        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme.clone(), value);
            return Ok(());
        }

        if self.enclosing.is_some() {
            return self
                .enclosing
                .as_ref()
                .unwrap()
                .borrow_mut()
                .assign(name, value);
        }

        Err(RuntimeError::new(
            name.clone(),
            format!("Undefined variable '{}'.", name.lexeme),
        ))
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }
}
