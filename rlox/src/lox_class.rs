use std::fmt::Display;

use crate::{lox_callable::LoxCallable, lox_instance::LoxInstance};

#[derive(Clone, Debug, Hash, PartialEq)]
pub struct LoxClass {
    name: String,
}

impl LoxClass {
    pub fn new(name: &str) -> Self {
        Self { name: name.into() }
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
