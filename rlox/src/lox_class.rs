use std::{collections::HashMap, fmt::Display};

use crate::{lox_callable::LoxCallable, lox_function::LoxFunction, lox_instance::LoxInstance};

#[derive(Clone, PartialEq)]
pub struct LoxClass {
    name: String,
    methods: HashMap<String, LoxFunction>,
}

impl LoxClass {
    pub fn new(name: &str, methods: HashMap<String, LoxFunction>) -> Self {
        Self {
            name: name.into(),
            methods,
        }
    }

    pub fn find_method(&self, name: &str) -> Option<&LoxFunction> {
        self.methods.get(name)
    }
}

impl Display for LoxClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.name.fmt(f)
    }
}

impl LoxCallable for LoxClass {
    fn arity(&self) -> usize {
        0
    }

    fn call(
        &self,
        _interpreter: &mut crate::interpreter::Interpreter,
        _arguments: &[crate::value::Value],
    ) -> Result<crate::value::Value, crate::errors::RuntimeError> {
        let instance = LoxInstance::new(self).into();
        Ok(instance)
    }

    fn string_repr(&self) -> String {
        todo!()
    }
}
