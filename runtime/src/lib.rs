use chunk::{BytecodeError, Chunk};
use instructions::Instruction;
use stack::{RegisterContents, Stack};
use thiserror::Error;

mod chunk;
pub mod codegen;
pub mod constants;
mod instructions;
pub mod stack;

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
        let mut ip = 0;
        let return_value = loop {
            if ip == chunk.buffer.len() {
                break RegisterContents::None;
            }
            let instruction = Instruction::from_byte(chunk.buffer[ip])?;
            ip += 1;
            match instruction {
                Instruction::Return => {
                    if chunk.buffer.len() < ip + 2 {
                        Err(RuntimeError::InvalidBytecode(BytecodeError::UnexpectedEOF))?;
                    }

                    let value = self.stack.get(chunk.buffer[ip])?;
                    self.stack.pop_frame();
                    if self.stack.is_root() {
                        break value;
                    } else {
                        self.stack.set(chunk.buffer[1], value)?;
                        ip += 1;
                    }
                }
                Instruction::Const => {
                    let (address, dest) = Instruction::read_constant(&chunk.buffer[ip..])?;
                    let const_val = chunk
                        .consts
                        .get(address as usize)
                        .ok_or(RuntimeError::InvalidConstant(address))?;
                    self.stack.set(dest, *const_val)?;
                    ip += 3;
                }
                Instruction::PushFrame => {
                    self.stack.push_frame();
                }

                Instruction::StoreLocal => {
                    let var_id = Instruction::read_variable(&chunk.buffer[ip..])?;
                    let value = chunk.buffer[2];
                    self.stack.store(var_id, self.stack.get(value)?);
                    ip += 3;
                }

                Instruction::LoadLocal => {
                    let var_id = Instruction::read_variable(&chunk.buffer[ip..])?;
                    let dest = chunk.buffer[ip + 2];
                    let value = self.stack.load(var_id);
                    self.stack.set(dest, value)?;
                    ip += 3;
                }

                other if other.is_binary() => {
                    let (left, right, dest) = Instruction::read_binary_args(&chunk.buffer[ip..])?;
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
                    ip += 3;
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
