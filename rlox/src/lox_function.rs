use std::{cell::RefCell, rc::Rc};

use crate::{
    environment::Environment, errors::RuntimeError, interpreter::Interpreter,
    lox_callable::LoxCallable, stmt::Stmt, token::Token, value::Value,
};

#[derive(Clone)]
pub struct LoxFunction {
    name: Token,
    params: Vec<Token>,
    body: Vec<Option<Stmt>>,
    closure: Rc<RefCell<Environment>>,
}

impl LoxFunction {
    pub fn new(
        name: &Token,
        params: &[Token],
        body: &[Option<Stmt>],
        closure: Rc<RefCell<Environment>>,
    ) -> Self {
        LoxFunction {
            name: name.clone(),
            params: Vec::from(params),
            body: Vec::from(body),
            closure,
        }
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
            Ok(_) => Ok(Value::Nil),
            Err(RuntimeError {
                token: _,
                msg: _,
                return_value: Some(v),
            }) => Ok(v),
            Err(e) => Err(e),
        }
    }

    fn string_repr(&self) -> String {
        format!("<fn {}>", self.name.lexeme)
    }
}
