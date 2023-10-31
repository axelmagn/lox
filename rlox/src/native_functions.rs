use std::time::{SystemTime, UNIX_EPOCH};

use ordered_float::OrderedFloat;

use crate::{
    errors::RuntimeError, interpreter::Interpreter, lox_callable::LoxCallable, value::Value,
};

pub const CLOCK_FN: Clock = Clock;

pub struct Clock;

impl LoxCallable for Clock {
    fn arity(&self) -> usize {
        0
    }

    fn call(
        &self,
        _interpreter: &mut Interpreter,
        _arguments: &[Value],
    ) -> Result<Value, RuntimeError> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();
        Ok(Value::Number(OrderedFloat::from(now)))
    }

    fn string_repr(&self) -> String {
        "<native fn>".into()
    }
}
