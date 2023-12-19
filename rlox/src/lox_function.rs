use std::{cell::RefCell, rc::Rc};

use crate::{
    environment::Environment, errors::RuntimeError, interpreter::Interpreter,
    lox_callable::LoxCallable, lox_instance::LoxInstance, stmt::Stmt, token::Token, value::Value,
};

#[derive(Clone)]
pub struct LoxFunction {
    name: Token,
    params: Vec<Token>,
    body: Vec<Option<Stmt>>,
    closure: Rc<RefCell<Environment>>,
    is_initializer: bool,
}

impl LoxFunction {
    pub fn new(
        name: &Token,
        params: &[Token],
        body: &[Option<Stmt>],
        closure: Rc<RefCell<Environment>>,
        is_initializer: bool,
    ) -> Self {
        LoxFunction {
            name: name.clone(),
            params: Vec::from(params),
            body: Vec::from(body),
            closure,
            is_initializer,
        }
    }

    pub fn bind(&self, instance: Rc<RefCell<LoxInstance>>) -> Self {
        let mut environment = Environment::with_enclosing(self.closure.clone());
        environment.define("this".into(), instance.into());
        Self::new(
            &self.name,
            &self.params,
            &self.body,
            Rc::new(RefCell::new(environment)),
            self.is_initializer,
        )
    }
}

impl LoxCallable for LoxFunction {
    fn arity(&self) -> usize {
        self.params.len()
    }

    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: &[Value],
    ) -> Result<Value, RuntimeError> {
        let mut environment = Environment::with_enclosing(self.closure.clone());
        for (i, param) in self.params.iter().enumerate() {
            environment.define(param.lexeme.clone(), arguments[i].clone())
        }
        let environment = Rc::new(RefCell::new(environment));
        match interpreter.execute_block(&self.body, environment) {
            Ok(_) => {
                if self.is_initializer {
                    Ok(self
                        .closure
                        .borrow()
                        .get_at(0, "this")
                        .expect("Could not find `this` bound to initializer scope"))
                } else {
                    Ok(Value::Nil)
                }
            }
            Err(RuntimeError {
                token: _,
                msg: _,
                return_value: Some(v),
            }) => {
                if self.is_initializer {
                    let value = self
                        .closure
                        .borrow()
                        .get_at(0, "this")
                        .expect("no keyword `this` found in initializer scope");
                    Ok(value)
                } else {
                    Ok(v)
                }
            }
            Err(e) => Err(e),
        }
    }

    fn string_repr(&self) -> String {
        format!("<fn {}>", self.name.lexeme)
    }
}

impl PartialEq for LoxFunction {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.params == other.params
            && self.body == other.body
            && self.closure.as_ptr() == other.closure.as_ptr()
    }
}
