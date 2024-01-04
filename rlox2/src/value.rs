use std::fmt::Display;

use derive_new::new;

use crate::object::Obj;

#[derive(Clone, Debug, PartialEq, new)]
pub enum Value {
    Boolean(bool),
    Nil,
    Number(f64),
    Obj(Box<Obj>),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Boolean(v) => v.fmt(f),
            Value::Number(v) => v.fmt(f),
            Value::Obj(v) => v.fmt(f),
            Value::Nil => "nil".fmt(f),
        }
    }
}
