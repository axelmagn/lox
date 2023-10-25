use crate::{errors::RuntimeError, interpreter::Interpreter, value::Value};

pub trait LoxCallable {
    fn arity(&self) -> usize;
    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: &[Value],
    ) -> Result<Value, RuntimeError>;
    fn string_repr(&self) -> String;
}
