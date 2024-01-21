pub mod expr;
mod block;
mod symbols;
mod scope;

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

    pub fn emit_binary(
        &mut self,
        left: Register,
        right: Register,
        instruction: Instruction
    ) -> Register {
        let dst = self.next_free_register();
        self.bytes.extend([instruction as u8, left, right, dst]);
        dst
    }

    pub fn emit_return(&mut self, value: Register) {
        self.bytes.extend([Instruction::Return as u8, value])
    }

    pub fn emit_const(&mut self, value: RegisterContents) -> Register {
        let dest = self.next_free_register();
        let index = self.create_const(value);
        self.bytes.push(Instruction::Const as u8);
        self.bytes.extend(index.to_le_bytes());
        self.bytes.push(dest);
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
    use crate::stack::RegisterContents;

    use super::Codegen;

    #[test]
    fn test_create_const() {
        let mut codegen = Codegen::new();

        let index = codegen.create_const(RegisterContents::Int(0));
        assert_eq!(index, 0);
        assert_eq!(codegen.constants, vec![RegisterContents::Int(0)]);
    }
}
