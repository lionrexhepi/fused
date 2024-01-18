pub mod expr;
mod block;
mod symbols;

use std::cell::Cell;

use crate::{ stack::{ Register, RegisterContents }, instructions::Instruction, chunk::Chunk };

use self::symbols::SymbolTable;

pub struct Codegen {
    bytes: Vec<u8>,
    constants: Vec<RegisterContents>,
    used_registers: Cell<Register>,
    symbols: SymbolTable
}

impl Codegen {
    pub fn new() -> Self {
        Self {
            bytes: Vec::new(),
            constants: Default::default(),
            used_registers: Cell::new(0),
            symbols: SymbolTable::new()
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

    pub fn access_symbol(&mut self, name: &str) -> Option<Register> {
        self.symbols.get(name).map(|(reg, _)| reg)
    }

    pub fn declare(&mut self, name: String, initial: Register, mutable: bool) {
        
        _ = self.symbols.declare(&name, initial, mutable);
        
    }

    pub fn enter_scope(&mut self) {
        //SAFETY: We immediately replace the uninitialized value with a valid one
        let old = std::mem::replace(&mut self.symbols, unsafe  {#[allow(invalid_value)] std::mem::MaybeUninit::zeroed().assume_init()});
        self.symbols = old.push();

    }

    pub fn leave_scope(&mut self) {
        //SAFETY: We immediately replace the uninitialized value with a valid one
        let old = std::mem::replace(&mut self.symbols, unsafe  {#[allow(invalid_value)] std::mem::MaybeUninit::zeroed().assume_init()});
         self.symbols = old.pop().expect("Attempted to leave scope when there was no scope to leave");
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
