use rlox2::{
    chunk::{Chunk, OpCode},
    debug::disassemble_chunk,
    value::Value,
};

fn main() {
    let mut chunk = Chunk::new();

    let x1 = chunk.add_constant(Value::new_number(2.5)) as u8;
    let x2 = chunk.add_constant(Value::new_number(4.6)) as u8;

    chunk.write_op(OpCode::Constant, 123);
    chunk.write_u8(x1, 123);
    chunk.write_op(OpCode::Constant, 123);
    chunk.write_u8(x2, 123);
    chunk.write_op(OpCode::Add, 123);

    disassemble_chunk(&chunk, "main");
}
