pub mod expr;
mod block;
pub mod scope;

use std::{ cell::RefCell, collections::HashMap, mem::size_of, rc::Rc };

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
    constants: Rc<RefCell<HashMap<RegisterContents, Index>>>,
    scope: Rc<SymbolTable>,
    breaks: Vec<Vec<JumpMark>>,
}

impl Codegen {
    pub fn new() -> Self {
        Self {
            bytes: Vec::new(),
            constants: Default::default(),
            scope: Rc::new(SymbolTable::new()),
            breaks: vec![],
        }
    }

    fn create_const(&mut self, value: RegisterContents) -> Index {
        let mut consts = self.constants.borrow_mut();
        let count = consts.len();
        *consts.entry(value).or_insert(count as Index)
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
        //self.bytes.push(Instruction::PushFrame as u8);
        self.scope.push();

        gen(self)?;

        self.scope.pop();
        //self.bytes.push(Instruction::PopFrame as u8);
        Ok(())
    }

    pub fn enter_loop(&mut self) {
        self.breaks.push(Vec::new())
    }

    pub fn emit_break(&mut self) -> CodegenResult {
        if !self.breaks.is_empty() {
            let mark = self.emit_uncond_jump();
            self.breaks.last_mut().unwrap().push(mark);
            Ok(())
        } else {
            Err(CodegenError::UndefinedSymbol("break".to_string()))
        }
    }

    pub fn exit_loop(&mut self) {
        let breaks = self.breaks.pop().expect("Attempted to exit loop without entering one");
        for mark in breaks {
            self.patch_jump(mark);
        }
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
        if let Some(symbol) = self.scope.get(name) {
            self.bytes.push(Instruction::Load as u8);

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
        if let Some(symbol) = self.scope.get(name) {
            self.bytes.push(Instruction::Store as u8);

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
        self.bytes.splice(to..to + size_of::<JumpMark>(), bytes);
    }

    pub fn get_jump_mark(&self) -> JumpMark {
        self.bytes.len()
    }

    pub fn emit_jump_back(&mut self, to: JumpMark) {
        self.bytes.push(Instruction::Jump as u8);
        self.bytes.extend(to.to_le_bytes());
    }

    pub fn chunk(self) -> Chunk {
        Chunk {
            consts: self.constants
                .borrow()
                .iter()
                .map(|(k, v)| (*v, *k))
                .collect(),
            buffer: self.bytes.into_boxed_slice(),
            var_count: self.scope.total_vars(),
        }
    }
}

pub trait ToBytecode {
    fn to_bytecode(&self, codegen: &mut Codegen) -> CodegenResult;
}

#[cfg(test)]
mod test {
    use crate::stack::RegisterContents;

    #[test]
    fn test_new_codegen() {
        let codegen = super::Codegen::new();
        assert_eq!(codegen.bytes.len(), 0);
    }

    #[test]
    fn test_emit_simple() {
        let mut codegen = super::Codegen::new();
        codegen.emit_simple(super::Instruction::Return).unwrap();
        let chunk = codegen.chunk();
        let mut reader = crate::bufreader::BufReader::new(&chunk.buffer);
        assert_eq!(reader.read_instruction().unwrap(), super::Instruction::Return);
    }

    #[test]
    fn test_emit_const() {
        let mut codegen = super::Codegen::new();
        codegen.emit_const(RegisterContents::Int(5)).unwrap();
        codegen.emit_const(RegisterContents::Int(3)).unwrap();

        let chunk = codegen.chunk();

        assert_eq!(chunk.consts.len(), 2);
        assert_eq!(chunk.consts[&0], RegisterContents::Int(5));
        assert_eq!(chunk.consts[&1], RegisterContents::Int(3));

        let mut reader = crate::bufreader::BufReader::new(&chunk.buffer);
        assert_eq!(reader.read_instruction().unwrap(), super::Instruction::Const);
        assert_eq!(reader.read_index().unwrap(), 0);
        assert_eq!(reader.read_instruction().unwrap(), super::Instruction::Const);
        assert_eq!(reader.read_index().unwrap(), 1);
    }

    #[test]
    fn test_emit_load() {
        let mut codegen = super::Codegen::new();
        codegen.declare("test".to_string(), false);
        codegen.emit_load("test").unwrap();

        let chunk = codegen.chunk();
        assert_eq!(chunk.consts.len(), 0);
        assert_eq!(chunk.var_count, 1);

        let mut reader = crate::bufreader::BufReader::new(&chunk.buffer);
        assert_eq!(reader.read_instruction().unwrap(), super::Instruction::Load);
        assert_eq!(reader.read_symbol().unwrap(), 0);
    }

    #[test]
    fn test_emit_store() {
        let mut codegen = super::Codegen::new();
        codegen.declare("test".to_string(), false);
        codegen.emit_store("test").unwrap();

        let chunk = codegen.chunk();
        assert_eq!(chunk.consts.len(), 0);
        assert_eq!(chunk.var_count, 1);

        let mut reader = crate::bufreader::BufReader::new(&chunk.buffer);
        assert_eq!(reader.read_instruction().unwrap(), super::Instruction::Store);
        assert_eq!(reader.read_symbol().unwrap(), 0);
    }

    #[test]
    fn test_emit_jump() {
        let mut codegen = super::Codegen::new();
        let mark = codegen.emit_uncond_jump();
        codegen.patch_jump(mark);
        codegen.emit_simple(super::Instruction::Return).unwrap();

        let chunk = codegen.chunk();
        assert_eq!(chunk.consts.len(), 0);
        assert_eq!(chunk.var_count, 0);

        let mut reader = crate::bufreader::BufReader::new(&chunk.buffer);
        assert_eq!(reader.read_instruction().unwrap(), super::Instruction::Jump);
        let address = reader.read_address().unwrap();
        assert_eq!(address, 9);
        assert_eq!(reader.read_instruction().unwrap(), super::Instruction::Return);
    }

    #[test]
    fn test_emit_cond_jump() {
        let mut codegen = super::Codegen::new();
        let mark = codegen.emit_cond_jump();
        codegen.patch_jump(mark);
        codegen.emit_simple(super::Instruction::Return).unwrap();

        let chunk = codegen.chunk();
        assert_eq!(chunk.consts.len(), 0);
        assert_eq!(chunk.var_count, 0);

        let mut reader = crate::bufreader::BufReader::new(&chunk.buffer);
        assert_eq!(reader.read_instruction().unwrap(), super::Instruction::JumpIfFalse);
        let address = reader.read_address().unwrap();
        assert_eq!(address, 9);
        assert_eq!(reader.read_instruction().unwrap(), super::Instruction::Return);
    }

    #[test]
    fn test_emit_jump_back() {
        let mut codegen = super::Codegen::new();
        let mark = codegen.get_jump_mark();
        codegen.emit_const(RegisterContents::Bool(true)).unwrap();
        codegen.emit_jump_back(mark);
        codegen.emit_simple(super::Instruction::Return).unwrap();

        let chunk = codegen.chunk();
        assert_eq!(chunk.consts.len(), 1);
        assert_eq!(chunk.var_count, 0);

        let mut reader = crate::bufreader::BufReader::new(&chunk.buffer);
        assert_eq!(reader.read_instruction().unwrap(), super::Instruction::Const);
        assert_eq!(reader.read_index().unwrap(), 0);
        assert_eq!(reader.read_instruction().unwrap(), super::Instruction::Jump);
        let address = reader.read_address().unwrap();
        assert_eq!(address, 0);
        reader.jump_to(address).unwrap();
        assert_eq!(reader.read_instruction().unwrap(), super::Instruction::Const);
    }

    #[test]
    fn test_new_scope() {
        let mut codegen = super::Codegen::new();
        codegen.declare("x".to_string(), true);

        codegen
            .new_scope(|codegen| {
                codegen.emit_const(RegisterContents::Int(5))?;
                codegen.emit_store("x")?;
                codegen.declare("y".to_string(), true);
                codegen.emit_const(RegisterContents::Int(3))?;
                codegen.emit_store("y")?;
                Ok(())
            })
            .unwrap();
        codegen.emit_simple(super::Instruction::Return).unwrap();

        let chunk = codegen.chunk();
        assert_eq!(chunk.consts.len(), 2);
        assert_eq!(chunk.var_count, 2);

        let mut reader = crate::bufreader::BufReader::new(&chunk.buffer);
        assert_eq!(reader.read_instruction().unwrap(), super::Instruction::Const);
        assert_eq!(reader.read_index().unwrap(), 0);
        assert_eq!(reader.read_instruction().unwrap(), super::Instruction::Store);
        assert_eq!(reader.read_symbol().unwrap(), 0);
        assert_eq!(reader.read_instruction().unwrap(), super::Instruction::Const);
        assert_eq!(reader.read_index().unwrap(), 1);
        assert_eq!(reader.read_instruction().unwrap(), super::Instruction::Store);
        assert_eq!(reader.read_symbol().unwrap(), 1);

        assert_eq!(reader.read_instruction().unwrap(), super::Instruction::Return);
    }

    #[test]
    fn test_emit_break() {
        let mut codegen = super::Codegen::new();
        codegen.enter_loop();
        codegen.emit_break().unwrap();
        codegen.exit_loop();
        codegen.emit_simple(super::Instruction::Return).unwrap();

        let chunk = codegen.chunk();
        assert_eq!(chunk.consts.len(), 0);
        assert_eq!(chunk.var_count, 0);

        let mut reader = crate::bufreader::BufReader::new(&chunk.buffer);
        assert_eq!(reader.read_instruction().unwrap(), super::Instruction::Jump);
        let address = reader.read_address().unwrap();
        assert_eq!(address, 9);
        assert_eq!(reader.read_instruction().unwrap(), super::Instruction::Return);
    }

    #[test]
    fn test_nested_scope() {
        let mut codegen = super::Codegen::new();
        codegen.declare("x".to_string(), true);

        codegen
            .new_scope(|codegen| {
                codegen.emit_const(RegisterContents::Int(5))?;
                codegen.emit_store("x")?;
                codegen.declare("y".to_string(), true);
                codegen.emit_const(RegisterContents::Int(3))?;
                codegen.emit_store("y")?;
                codegen.new_scope(|codegen| {
                    codegen.declare("x".to_string(), true);
                    codegen.emit_const(RegisterContents::Int(7))?;
                    codegen.emit_store("x")?;
                    Ok(())
                })?;
                Ok(())
            })
            .unwrap();

        codegen.emit_simple(super::Instruction::Return).unwrap();

        let chunk = codegen.chunk();
        assert_eq!(chunk.consts.len(), 3);
        assert_eq!(chunk.var_count, 3);

        let mut reader = crate::bufreader::BufReader::new(&chunk.buffer);
        assert_eq!(reader.read_instruction().unwrap(), super::Instruction::Const);
        assert_eq!(reader.read_index().unwrap(), 0);
        assert_eq!(reader.read_instruction().unwrap(), super::Instruction::Store);
        assert_eq!(reader.read_symbol().unwrap(), 0);
        assert_eq!(reader.read_instruction().unwrap(), super::Instruction::Const);
        assert_eq!(reader.read_index().unwrap(), 1);
        assert_eq!(reader.read_instruction().unwrap(), super::Instruction::Store);
        assert_eq!(reader.read_symbol().unwrap(), 1);
        assert_eq!(reader.read_instruction().unwrap(), super::Instruction::Const);
        assert_eq!(reader.read_index().unwrap(), 2);
        assert_eq!(reader.read_instruction().unwrap(), super::Instruction::Store);
        assert_eq!(reader.read_symbol().unwrap(), 2);
    }
}
