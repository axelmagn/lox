use crate::{chunk::Chunk, object::Obj, value::Value};

#[derive(Clone, Debug)]
pub struct VM {
    chunk: Box<Chunk>,
    ip: usize,
    stack: Vec<Value>,
    objects: Vec<Obj>,
}

impl VM {
    pub fn new() -> Self {
        Self {
            chunk: Box::new(Chunk::new()),
            ip: 0,
            stack: Vec::new(),
            objects: Vec::new(),
        }
    }

    pub fn interpret(&mut self, source: &str) -> InterpretResult {
        todo!();
    }

    pub fn push(&mut self, value: Value) {
        self.stack.push(value);
    }

    pub fn pop(&mut self) -> Value {
        self.stack.pop().expect("pop called on empty stack")
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}
