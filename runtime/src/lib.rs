use bufreader::BufReader;
use chunk::{BytecodeError, Chunk};
use instructions::Instruction;
use stack::{RegisterContents, Stack};
use thiserror::Error;

mod chunk;
pub mod codegen;
pub mod constants;
mod instructions;
pub mod stack;
mod bufreader;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum RuntimeError {
    #[error("Failed to allocate memory")]
    AllocationFailure,
    #[error("Error while reading bytecode: {0}\n\n This error is likely unrecoverable.")]
    InvalidBytecode(#[from] BytecodeError),
    #[error("Attempted operation with null value")]
    NullAccess,
    #[error("Bad stack frame {0:x}")]
    BadStackFrame(u16),
    #[error("Operation {0} unsupported for types {1} and {2}")]
    InvalidOperation(&'static str, &'static str, &'static str),
    #[error("Chunk does not contain a constant at index {0:x}")]
    InvalidConstant(u16),
    #[error("Attempted to access undefined variable {0}")]
    UndefinedSymbol(String),
    #[error("Attempted to mutate immutable variable {0}")]
    ImmutableSymbol(String),
}

pub(crate) type Result<T> = std::result::Result<T, RuntimeError>;

pub struct Thread {
    pub stack: Stack,
}

impl Thread {
    pub fn run_chunk(&mut self, chunk: Chunk) -> Result<RegisterContents> {
        let value;
        {
            value = self.run_guarded(chunk)?;
        }
        Ok(value)
    }

    fn run_guarded(&mut self, chunk: Chunk) -> Result<RegisterContents> {
        let mut reader = BufReader::new(&chunk.buffer);
        let return_value = loop {
            
            if reader.eof() {
                break RegisterContents::None;
            }
            let instruction = reader.read_instruction()?;
            match instruction {
                Instruction::Return => {

                    let value = self.stack.get(reader.read_register()?)?;
                    self.stack.pop_frame();
                    if self.stack.is_root() {
                        break value;
                    } else {
                        self.stack.set(reader.read_register()?, value)?;
                        
                    }
                }
                Instruction::Const => {
                    let address = reader.read_index()?;
                    let const_val = chunk
                        .consts
                        .get(address as usize)
                        .ok_or(RuntimeError::InvalidConstant(address))?;
                    self.stack.set(reader.read_register()?, *const_val)?;
                }
                Instruction::PushFrame => {
                    self.stack.push_frame();
                }

                Instruction::StoreLocal => {

                    self.stack.store(reader.read_index()?, self.stack.get(reader.read_register()?)?);
                }

                Instruction::LoadLocal => {
                    let value = self.stack.load(reader.read_index()?);
                    self.stack.set(reader.read_register()?, value)?;
                }

                other if other.is_binary() => {
                    let (left, right, dest) = (
                        reader.read_register()?,
                        reader.read_register()?,
                        reader.read_register()?

                    );
                    let operator = match other {
                        Instruction::Add => RegisterContents::try_add,
                        Instruction::Sub => RegisterContents::try_sub,
                        Instruction::Mul => RegisterContents::try_mul,
                        Instruction::Div => RegisterContents::try_div,
                        Instruction::Mod => RegisterContents::try_mod,
                        Instruction::BitAnd => RegisterContents::try_bitand,
                        Instruction::BitOr => RegisterContents::try_bitor,
                        Instruction::BitXor => RegisterContents::try_bitxor,
                        Instruction::LeftShift => RegisterContents::try_leftshift,
                        Instruction::RightShift => RegisterContents::try_rightshift,
                        Instruction::Eq => RegisterContents::try_eq,
                        Instruction::And => RegisterContents::try_and,
                        Instruction::Or => RegisterContents::try_or,
                        _ => unreachable!(),
                    };

                    let left = self.stack.get(left)?;
                    let right = self.stack.get(right)?;
                    let result = operator(&left, &right)?;
                    self.stack.set(dest, result)?;
                }
                _ => unreachable!("Missing match arm for instruction {:?}", instruction),
            }
        };

        Ok(return_value)
    }
}

#[cfg(test)]
mod test {
    use crate::stack::RegisterContents;

    #[test]
    fn test_display() {
        let values = [
            RegisterContents::Int(5),
            RegisterContents::Float(3.445),
            RegisterContents::Bool(true),
            RegisterContents::Char('o'),
            RegisterContents::None,
        ];

        for value in values {
            println!("{value}\n");
        }
    }
}
