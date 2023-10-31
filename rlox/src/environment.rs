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

    pub fn with_enclosing(enclosing: Rc<RefCell<Self>>) -> Self {
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

        if let Some(enclosing) = &self.enclosing {
            return (**enclosing).borrow_mut().assign(name, value);
        }

        Err(RuntimeError::new(
            name.clone(),
            format!("Undefined variable '{}'.", name.lexeme),
        ))
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }

    pub fn get_at(&self, distance: usize, name: &str) -> Option<Value> {
        if distance == 0 {
            return self.values.get(name).cloned();
        }

        self.enclosing
            .as_ref()
            .unwrap()
            .borrow()
            .get_at(distance - 1, name)

        // let mut environment = self.enclosing.clone().unwrap();
        // for _ in 1..distance {
        //     let enclosing = environment.borrow().enclosing.clone().unwrap();
        //     environment = enclosing;
        // }

        // let value = environment.borrow().values.get(name).cloned();
        // value
    }

    pub fn assign_at(&mut self, distance: usize, name: &Token, value: &Value) {
        if distance == 0 {
            self.values.insert(name.lexeme.clone(), value.clone());
            return;
        }

        self.enclosing
            .as_mut()
            .unwrap()
            .borrow_mut()
            .assign_at(distance - 1, name, value);

        // let mut environment = self.enclosing.clone().unwrap();
        // for _ in 1..distance {
        //     let enclosing = environment.borrow().enclosing.clone().unwrap();
        //     environment = enclosing;
        // }

        // Rc::get_mut(&mut environment)
        //     .unwrap()
        //     .get_mut()
        //     .values
        //     .insert(name.lexeme.clone(), value.clone());
    }
}
