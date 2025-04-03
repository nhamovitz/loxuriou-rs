use crate::chunk::{self, Chunk, OpCode, Value};

pub(crate) struct VM {
    chunk: Chunk,
    ip: usize,
    stack: Vec<Value>,
}

pub(crate) enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}

macro_rules! binary_op {
    ($self:expr, $op:tt) => { {
        let b = $self.stack.pop().unwrap().0;
        let a = $self.stack.pop().unwrap().0;
        let res = a $op b;
        $self.stack.push(chunk::Value(res)); }
    };
} // pickup: this could be a function. there's function versions of all the arithmetic ops

impl VM {
    pub(crate) fn interpret(chunk: Chunk) -> InterpretResult {
        let mut vm = VM {
            chunk,
            ip: 0,
            stack: Vec::new(),
        };

        vm.run()
    }

    fn run(&mut self) -> InterpretResult {
        for (i, instruction) in self.chunk.code.iter().enumerate() {
            println!("{:?}", self.stack);
            self.chunk.dissasemble_instruction(i); // bit horrible

            match instruction {
                OpCode::Return => {
                    if let Some(v) = self.stack.pop() {
                        v.print();
                    }
                    return InterpretResult::Ok;
                }
                OpCode::Constant(v) => self.stack.push(v.clone()),
                OpCode::Negate => {
                    let v = -self.stack.pop().unwrap().0;
                    self.stack.push(Value(v));
                }
                OpCode::Add => binary_op!(self, +),
                OpCode::Subtract => binary_op!(self, -),
                OpCode::Multiply => binary_op!(self, *),
                OpCode::Divide => binary_op!(self, /),
            }
        }

        InterpretResult::Ok
    }
}
