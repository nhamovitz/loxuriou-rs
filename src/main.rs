#![allow(dead_code)]

use std::{ops::Index, usize};

#[derive(Debug)]
enum OpCode {
    Return,
    Constant(Value),
}

#[derive(Debug)]
struct Value(f64);

impl Value {
    fn print(&self) {
        println!("{:?}", self.0);
    }
}

/// A sequence of bytecode.
#[derive(Default)]
struct Chunk {
    code: Vec<OpCode>,
    constants: Vec<Value>,
    lines: Vec<usize>,
}

impl Chunk {
    fn add_constant(&mut self, value: Value) -> usize {
        self.constants.push(value);
        self.constants.len() - 1
    }

    fn write(&mut self, value: OpCode, line: usize) {
        self.code.push(value);
        self.lines.push(line);
    }

    fn dissasemble(&self, name: impl AsRef<str>) {
        println!("== {:?} ==", name.as_ref());

        let mut offset = 0;
        while offset < self.code.len() {
            offset = self.dissasemble_instruction(offset);
        }
    }

    fn dissasemble_instruction(&self, offset: usize) -> usize {
        print!("{:0>4} ", offset);

        if offset > 0 && self.lines[offset] == self.lines[offset - 1] {
            print!("   | ");
        } else {
            print!("{:4} ", self.lines[offset]);
        }

        let instruction = &self.code[offset];
        match instruction {
            OpCode::Return => Self::simple_instruction(&OpCode::Return, offset),
            c @ OpCode::Constant(_) => self.constant_instruction(c, offset),

            _ => {
                println!("Unknown opcode {:?}", instruction);
                offset + 1
            }
        }
    }
    fn simple_instruction(op_code: &OpCode, offset: usize) -> usize {
        println!("{:?}", op_code);
        offset + 1
    }

    fn constant_instruction(&self, op_code: &OpCode, offset: usize) -> usize {
        println!("{:?}", op_code); //   printf("%-16s %4d '", name, constant_index);
                                   // constant is inline to we have no index to print
                                   // debug impl includes opcode and the constant

        offset + 1
    }
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();

    let mut chunk = Chunk::default();

    chunk.write(OpCode::Constant(Value(1.2)), 123);

    chunk.write(OpCode::Return, 123);
    chunk.dissasemble("test chunk");
}
