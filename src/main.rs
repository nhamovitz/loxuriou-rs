use chunk::{Chunk, OpCode};

mod chunk;
mod vm;

fn main() {
    let _args = std::env::args().collect::<Vec<_>>();

    let mut chunk = Chunk::default();

    chunk.write(OpCode::Constant(chunk::Value(2.)), 123);

    chunk.write(OpCode::Constant(chunk::Value(3.)), 123);

    chunk.write(OpCode::Add, 123);
    chunk.write(OpCode::Constant(chunk::Value(11.)), 123);
    chunk.write(OpCode::Divide, 123);

    chunk.write(OpCode::Negate, 123);
    chunk.write(OpCode::Return, 123);
    chunk.dissasemble("test chunk");

    println!("starting vm");
    vm::VM::interpret(chunk);
}
