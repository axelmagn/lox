use std::{collections::HashMap, fmt::Display};

use crate::{
    errors::RuntimeError, interpreter::Interpreter, lox_callable::LoxCallable,
    lox_function::LoxFunction, lox_instance::LoxInstance, value::Value,
};

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
        if let Some(initializer) = self.find_method("init") {
            initializer.arity()
        } else {
            0
        }
    }

    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: &[Value],
    ) -> Result<Value, RuntimeError> {
        let instance: Value = LoxInstance::new(self).into();
        let initializer = self.find_method("init");
        match (&initializer, &instance) {
            (Some(initializer), Value::LoxInstance(instance)) => {
                initializer
                    .bind(instance.clone())
                    .call(interpreter, arguments)?;
            }
            _ => {}
        }
        Ok(instance)
    }

    fn string_repr(&self) -> String {
        todo!()
    }
}
