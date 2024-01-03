pub mod expr;
mod block;

use std::cell::Cell;

use crate::{ stack::{ Register, RegisterContents }, instructions::Instruction, chunk::Chunk };

pub struct Codegen {
    bytes: Vec<u8>,
    constants: Vec<RegisterContents>,
    used_registers: Cell<Register>,
}

impl Codegen {
    pub fn new() -> Self {
        Self {
            bytes: Vec::new(),
            constants: Default::default(),
            used_registers: Cell::new(0),
        }
    }

    fn next_free_register(&mut self) -> Register {
        let register = self.used_registers.get();
        self.used_registers.set(register + 1);
        register
    }

    fn create_const(&mut self, value: RegisterContents) -> u16 {
        self.constants.push(value);
        (self.constants.len() as u16) - 1
    }

    pub fn emit_return(&mut self, value: Register) {
        self.bytes.extend([Instruction::RETURN, value])
    }

    pub fn emit_const(&mut self, value: RegisterContents) -> Register {
        let dest = self.next_free_register();
        let index = self.create_const(value);
        self.bytes.push(Instruction::CONST);
        self.bytes.extend(index.to_le_bytes());
        self.bytes.push(dest);
        dest
    }

    pub fn emit_add(&mut self, left: Register, right: Register) -> Register {
        let dest = self.next_free_register();
        self.bytes.extend([Instruction::ADD, left, right, dest]);
        dest
    }

    pub fn emit_sub(&mut self, left: Register, right: Register) -> Register {
        let dest = self.next_free_register();
        self.bytes.extend([Instruction::SUB, left, right, dest]);
        dest
    }

    pub fn chunk<'a>(self) -> Chunk {
        Chunk {
            consts: self.constants,
            buffer: self.bytes.into_boxed_slice(),
        }
    }
}

pub trait ToBytecode {
    fn to_bytecode(&self, codegen: &mut Codegen) -> Register;
}

#[cfg(test)]
mod test {
    use crate::{ instructions::Instruction, stack::RegisterContents };

    use super::Codegen;

    #[test]
    fn test_create_const() {
        let mut codegen = Codegen::new();

        let index = codegen.create_const(RegisterContents::Int(0));
        assert_eq!(index, 0);
        assert_eq!(codegen.constants, vec![RegisterContents::Int(0)]);
    }

    #[test]
    fn test_return() {
        let mut codegen = Codegen::new();

        codegen.emit_return(0);
        let chunk = codegen.chunk();
        assert_eq!(Instruction::read_from_chunk(&chunk.buffer), Ok((Instruction::Return(0), 2)));
    }

    #[test]
    fn test_const() {
        let mut codegen = Codegen::new();

        codegen.emit_const(RegisterContents::Int(1));
        let chunk = codegen.chunk();
        assert_eq!(chunk.consts.first(), Some(&RegisterContents::Int(1)));
        assert_eq!(Instruction::read_from_chunk(&chunk.buffer), Ok((Instruction::Const(0, 0), 4)));
    }

    #[test]
    fn test_add() {
        let mut codegen = Codegen::new();

        codegen.emit_add(1, 2); // Since we didn't emit a const, 0 is free and will be selected as the next free register
        let chunk = codegen.chunk();
        assert_eq!(
            Instruction::read_from_chunk(&chunk.buffer),
            Ok((Instruction::Add { left: 1, right: 2, dst: 0 }, 4))
        );
    }

    #[test]
    fn test_sub() {
        let mut codegen = Codegen::new();

        codegen.emit_sub(1, 2); // Since we didn't emit a const, 0 is free and will be selected as the next free register
        let chunk = codegen.chunk();
        assert_eq!(
            Instruction::read_from_chunk(&chunk.buffer),
            Ok((Instruction::Sub { left: 1, right: 2, dst: 0 }, 4))
        );
    }
}
