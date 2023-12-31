use crate::chunk::{Chunk, OpCode};

pub fn disassemble_chunk(chunk: &Chunk, name: &str) {
    println!("== {} ==", name);

    let mut offset = 0;
    while offset < chunk.len() {
        offset = disassemble_instruction(chunk, offset)
    }
}

pub fn disassemble_instruction(chunk: &Chunk, offset: usize) -> usize {
    print!("{:04} ", offset);
    if offset > 0 && chunk.lines[offset] == chunk.lines[offset - 1] {
        print!("   | ");
    } else {
        print!("{:4} ", chunk.lines[offset])
    }

    let instruction: OpCode = match num::FromPrimitive::from_u8(chunk.code[offset]) {
        Some(v) => v,
        None => {
            println!("Unknown opcode: {}", chunk.code[offset]);
            return offset + 1;
        }
    };

    match instruction {
        OpCode::Constant => constant_instruction("OP_CONSTANT", chunk, offset),
        OpCode::Nil => simple_instruction("OP_NIL", offset),
        OpCode::True => simple_instruction("OP_TRUE", offset),
        OpCode::False => simple_instruction("OP_FALSE", offset),
        OpCode::Equal => simple_instruction("OP_EQUAL", offset),
        OpCode::Greater => simple_instruction("OP_GREATER", offset),
        OpCode::Less => simple_instruction("OP_LESS", offset),
        OpCode::Add => simple_instruction("OP_ADD", offset),
        OpCode::Subtract => simple_instruction("OP_SUBTRACT", offset),
        OpCode::Multiply => simple_instruction("OP_MULTIPLY", offset),
        OpCode::Divide => simple_instruction("OP_DIVIDE", offset),
        OpCode::Not => simple_instruction("OP_NOT", offset),
        OpCode::Negate => simple_instruction("OP_NEGATE", offset),
        OpCode::Return => simple_instruction("OP_RETURN", offset),
    }
}

fn simple_instruction(name: &str, offset: usize) -> usize {
    println!("{}", name);
    offset + 1
}

fn constant_instruction(name: &str, chunk: &Chunk, offset: usize) -> usize {
    let constant = chunk.code[offset + 1] as usize;
    println!(
        "{:-16} {:4} '{}'",
        name, constant, chunk.constants[constant]
    );
    offset + 2
}
