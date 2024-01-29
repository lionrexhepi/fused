pub mod expr;
mod block;
mod scope;

use std::{
    cell::{ Cell, RefCell },
    io::Write,
    mem::size_of,
    ops::{ RangeFrom, RangeFull },
    rc::Rc,
};

use thiserror::Error;

use crate::{ bufreader::Index, chunk::Chunk, instructions::Instruction, stack::RegisterContents };

use self::scope::SymbolTable;

#[derive(Error, Debug)]
pub enum CodegenError {
    #[error("Undefined symbol: {0}")] UndefinedSymbol(String),
}

type CodegenResult = std::result::Result<(), CodegenError>;
type JumpMark = usize;

#[derive(Clone)]
pub struct Codegen {
    bytes: Vec<u8>,
    constants: Rc<RefCell<Vec<RegisterContents>>>,
    scope: Rc<SymbolTable>,
}

impl Codegen {
    pub fn new() -> Self {
        Self {
            bytes: Vec::new(),
            constants: Default::default(),
            scope: Default::default(),
        }
    }

    fn create_const(&mut self, value: RegisterContents) -> u16 {
        self.constants.borrow_mut().push(value);
        (self.constants.borrow().len() as u16) - 1
    }

    /// - Emits simple instructions
    pub fn emit_simple(&mut self, instruction: Instruction) -> CodegenResult {
        self.bytes.push(instruction as u8);
        Ok(())
    }

    pub fn emit_const(&mut self, value: RegisterContents) -> CodegenResult {
        let id = self.create_const(value);
        self.bytes.push(Instruction::Const as u8);
        self.bytes.extend(id.to_le_bytes());
        Ok(())
    }

    /// - Pushes a new lexical scope
    /// - Instructs the VM to push a new frame
    /// - Executes the provided function to generate the code to be executed in the new scope
    /// - Pops the frame, instructing the VM to copy the result into the parent scope via a return instruction
    /// - Returns the register containing the result
    pub fn new_scope(&mut self, gen: impl FnOnce(&mut Self) -> CodegenResult) -> CodegenResult {
        self.bytes.push(Instruction::PushFrame as u8);
        self.scope.push();
        let mut child = Self {
            constants: self.constants.clone(), //TODO: inefficient
            bytes: Vec::new(),
            scope: self.scope.clone(),
        };

        gen(&mut child)?;
        self.scope.pop();
        let Chunk { buffer, .. } = child.chunk();
        self.bytes.append(&mut buffer.into_vec());
        Ok(())
    }

    /// Declares a new symbol in the current scope
    /// -> This is a purely lexical method and does not emit any instructions
    pub fn declare(&mut self, name: String, _mutable: bool) -> scope::SymbolId {
        self.scope.declare(name)
    }

    /// - Loads the value of the symbol into a new register
    /// - Returns the register
    /// - If the symbol is not defined, returns an error
    pub fn emit_load(&mut self, name: &str) -> CodegenResult {
        if let Some((depth, symbol)) = self.scope.get(name) {
            if depth == 0 {
                self.bytes.push(Instruction::LoadLocal as u8);
            } else {
                self.bytes.extend(&[Instruction::Load as u8, depth]);
            }

            self.bytes.extend(symbol.to_le_bytes());
            Ok(())
        } else {
            Err(CodegenError::UndefinedSymbol(name.to_string()))
        }
    }

    /// - Stores the value of the provided register into the symbol
    /// - Returns an error if the symbol is not defined
    /// - Returns the register
    pub fn emit_store(&mut self, name: &str) -> CodegenResult {
        if let Some((depth, symbol)) = self.scope.get(name) {
            if depth == 0 {
                self.bytes.push(Instruction::StoreLocal as u8);
            } else {
                self.bytes.extend(&[Instruction::Store as u8, depth]);
            }

            self.bytes.extend(symbol.to_le_bytes());

            Ok(())
        } else {
            Err(CodegenError::UndefinedSymbol(name.to_string()))
        }
    }

    pub fn emit_cond_jump(&mut self) -> JumpMark {
        self.bytes.push(Instruction::JumpIfFalse as u8);
        let mark = self.bytes.len();
        self.bytes.extend((0usize).to_le_bytes());
        mark
    }

    pub fn emit_uncond_jump(&mut self) -> JumpMark {
        self.bytes.push(Instruction::Jump as u8);
        let mark = self.bytes.len();
        self.bytes.extend((0usize).to_le_bytes());
        mark
    }

    pub fn patch_jump(&mut self, to: JumpMark) {
        let bytes = self.bytes.len().to_le_bytes();
        println!("before: {:?}, len {}", self.bytes, self.bytes.len());
        self.bytes.splice(to..to + size_of::<JumpMark>(), bytes);
        println!("after: {:?}, {}", self.bytes, self.bytes.len());
    }

    pub fn chunk(self) -> Chunk {
        Chunk {
            consts: self.constants.borrow().clone(),
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
        assert_eq!(codegen.constants.take(), vec![RegisterContents::Int(0)]);
    }
}
