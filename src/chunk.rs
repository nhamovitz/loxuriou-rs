#[derive(Debug)]
pub(crate) enum OpCode {
    Return,
    Constant(Value),
    Negate,
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug, Clone)]
pub(crate) struct Value(pub f64);

impl Value {
    pub(crate) fn print(&self) {
        println!("{:?}", self.0);
    }
}

/// A sequence of bytecode.
#[derive(Default)]
pub(crate) struct Chunk {
    pub(crate) code: Vec<OpCode>,
    pub(crate) _constants: Vec<Value>,
    pub(crate) lines: Vec<usize>,
}

impl Chunk {
    pub(crate) fn _add_constant(&mut self, value: Value) -> usize {
        self._constants.push(value);
        self._constants.len() - 1
    }

    pub(crate) fn write(&mut self, value: OpCode, line: usize) {
        self.code.push(value);
        self.lines.push(line);
    }

    pub(crate) fn dissasemble(&self, name: impl AsRef<str>) {
        println!("== {:?} ==", name.as_ref());

        let mut offset = 0;
        while offset < self.code.len() {
            offset = self.dissasemble_instruction(offset);
        }
    }

    pub(crate) fn dissasemble_instruction(&self, offset: usize) -> usize {
        print!("{:0>4} ", offset);

        if offset > 0 && self.lines[offset] == self.lines[offset - 1] {
            print!("   | ");
        } else {
            print!("{:4} ", self.lines[offset]);
        }

        let instruction = &self.code[offset];
        match instruction {
            oc @ (OpCode::Return
            | OpCode::Negate
            | OpCode::Add
            | OpCode::Subtract
            | OpCode::Multiply
            | OpCode::Divide) => Self::simple_instruction(oc, offset),
            c @ OpCode::Constant(_) => self.constant_instruction(c, offset),

            _ => {
                println!("Unknown opcode {:?}", instruction);
                offset + 1
            }
        }
    }
    pub(crate) fn simple_instruction(op_code: &OpCode, offset: usize) -> usize {
        println!("{:?}", op_code);
        offset + 1
    }

    pub(crate) fn constant_instruction(&self, op_code: &OpCode, offset: usize) -> usize {
        println!("{:?}", op_code); //   printf("%-16s %4d '", name, constant_index);
                                   // constant is inline to we have no index to print
                                   // debug impl includes opcode and the constant

        offset + 1
    }
}
