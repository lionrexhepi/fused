pub mod expr;
mod block;
mod scope;

use std::{ cell::Cell, rc::Rc };

use thiserror::Error;

use crate::{ stack::{ Register, RegisterContents }, instructions::Instruction, chunk::Chunk };

use self::scope::SymbolTable;

#[derive(Error, Debug)]
pub enum CodegenError {
    #[error("Undefined symbol: {0}")] UndefinedSymbol(String),
}

type CodegenResult = std::result::Result<Register, CodegenError>;

pub struct Codegen {
    bytes: Vec<u8>,
    constants: Vec<RegisterContents>,
    used_registers: Cell<Register>,
    scope: Rc<SymbolTable>,
}

impl Codegen {
    pub fn new() -> Self {
        Self {
            bytes: Vec::new(),
            constants: Default::default(),
            used_registers: Cell::new(0),
            scope: Default::default(),
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

    /// - Emits the provided instruction with "left" and "right" as its arguments
    /// - Stores the result in a new register
    /// - Returns the register
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

    ///Emits an instruction to return the value in the provided register
    pub fn emit_return(&mut self, value: Register) {
        self.bytes.extend([Instruction::Return as u8, value])
    }

    /// - Creates a new constant
    /// - Loads the constant into a free register
    /// - Returns the register
    pub fn emit_const(&mut self, value: RegisterContents) -> Register {
        let dest = self.next_free_register();
        let index = self.create_const(value);
        self.bytes.push(Instruction::Const as u8);
        self.bytes.extend(index.to_le_bytes());
        self.bytes.push(dest);
        dest
    }

    /// - Pushes a new lexical scope
    /// - Instructs the VM to push a new frame
    /// - Executes the provided function to generate the code to be executed in the new scope
    /// - Takes the return value of the provided function, that being a register containing the "result" of the code
    /// - Pops the frame, instructing the VM to copy the result into the parent scope
    /// - Returns the register containing the result
    pub fn new_scope(&mut self, gen: impl FnOnce(&mut Self) -> CodegenResult) -> CodegenResult {
        self.bytes.push(Instruction::PushFrame as u8);
        self.scope.push();
        let mut child = Self {
            used_registers: Cell::new(0),
            constants: Vec::new(),
            bytes: Vec::new(),
            scope: self.scope.clone(),
        };

        let result = gen(&mut child)?;
        self.scope.pop();
        let Chunk { consts, buffer } = child.chunk();

        self.bytes.extend(buffer.into_iter());
        self.constants.extend(consts);

        let preserve = self.next_free_register();

        self.bytes.extend(&[Instruction::PopFrame as u8, result, preserve]);
        Ok(preserve)
    }

    /// Declares a new symbol in the current scope
    /// -> This is a purely lexical method and does not emit any instructions
    pub fn declare(&mut self, name: String, _mutable: bool) {
        self.scope.declare(name);
    }

    /// - Loads the value of the symbol into a new register
    /// - Returns the register
    /// - If the symbol is not defined, returns an error
    pub fn emit_load(&mut self, name: &str) -> CodegenResult {
        if let Some((depth, symbol)) = self.scope.get(name) {
            let dest = self.next_free_register();
            if depth == 0 {
                self.bytes.push(Instruction::LoadLocal as u8);
            } else {
                self.bytes.extend(&[Instruction::Load as u8, depth]);
            }

            self.bytes.extend(symbol.to_le_bytes());

            Ok(dest)
        } else {
            Err(CodegenError::UndefinedSymbol(name.to_string()))
        }
    }

    /// - Stores the value of the provided register into the symbol
    /// - Returns an error if the symbol is not defined
    /// - Returns the register
    pub fn emit_store(&mut self, name: &str, value: Register) -> CodegenResult {
        if let Some((depth, symbol)) = self.scope.get(name) {
            if depth == 0 {
                self.bytes.push(Instruction::StoreLocal as u8);
            } else {
                self.bytes.extend(&[Instruction::Store as u8, depth]);
            }

            self.bytes.extend(symbol.to_le_bytes());
            self.bytes.push(value);

            Ok(0)
        } else {
            Err(CodegenError::UndefinedSymbol(name.to_string()))
        }
    }

    pub fn chunk(self) -> Chunk {
        Chunk {
            consts: self.constants,
            buffer: self.bytes.into_boxed_slice(),
        }
    }
}

pub trait ToBytecode {
    fn to_bytecode(&self, codegen: &mut Codegen) -> CodegenResult;
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
