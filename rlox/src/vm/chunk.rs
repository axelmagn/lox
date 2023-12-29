use super::value::Value;

#[repr(u8)]
pub enum OpCode {
    Constant,
    Return,
}

impl OpCode {
    pub fn as_byte(&self) -> u8 {
        self as u8
    }
}

impl TryFrom<u8> for OpCode {
}

#[derive(Clone, Debug, Default)]
pub struct Chunk {
    pub code: Vec<u8>,
    pub lines: Vec<usize>,
    pub constants: Vec<Value>,
}

impl Chunk {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn write(&mut self, byte: u8, line: usize) {
        self.code.push(byte);
        self.lines.push(line);
    }

    pub fn add_constant(&mut self, value: Value) {
        self.constants.push(value);
    }
}