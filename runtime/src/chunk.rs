use std::{ collections::HashMap, fmt::{ Display, Formatter }, mem::size_of };

use thiserror::Error;

use crate::{
    instructions::Instruction,
    stack::{ RegisterContents },
    bufreader::{ Address, BufReader },
};

pub type Result<T> = std::result::Result<T, BytecodeError>;

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum BytecodeError {
    #[error("Invalid instruction: {0:x}")] InvalidInstruction(u8),

    #[error("Chunk ends in the middle of an instruction")]
    UnexpectedEOF,
    #[error("Invalid jump address: {0:x}")] InvalidJumpAddress(Address),
}

pub struct Chunk {
    pub consts: HashMap<u16, RegisterContents>,
    pub buffer: Box<[u8]>,
}

impl Chunk {
    pub fn size(&self) -> usize {
        self.consts.len() * size_of::<RegisterContents>() + self.buffer.len()
    }
}

impl<'a> Display for Chunk {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{:-^30}", "Constants")?;

        for (index, value) in self.consts.iter() {
            writeln!(f, "{:x}: {:?>30}", index, value)?;
        }

        writeln!(f, "{:-^30}", "Instructions")?;

        let mut reader = BufReader::new(&self.buffer);
        while !reader.eof() {
            let addr = reader.current_address();
            let instruction = reader.read_instruction()?;
            let (name, args) = match instruction {
                Instruction::Const => { ("const", format!("{:x}", reader.read_index()?)) }
                Instruction::Return => { ("ret", String::default()) }
                Instruction::PushFrame => ("pushframe", String::default()),
                Instruction::StoreLocal => {
                    ("store_loc", format!(" >> [{}] ", reader.read_index()?))
                }
                Instruction::LoadLocal => {
                    ("load_loc", format!(" << [{}]", reader.read_index()?))
                }
                Instruction::JumpIfFalse => {
                    ("jump_if_false", format!("#{}", reader.read_address()?))
                }
                Instruction::Jump => { ("jump", format!("#{}", reader.read_address()?)) }

                other if other.is_binary() => {
                    let name = match other {
                        Instruction::Add => "add",
                        Instruction::Sub => "sub",
                        Instruction::Mul => "mul",
                        Instruction::Div => "div",
                        Instruction::Mod => "mod",
                        Instruction::BitAnd => "bitand",
                        Instruction::BitOr => "bitor",
                        Instruction::BitXor => "bitxor",
                        Instruction::Eq => "eq",
                        Instruction::And => "and",
                        _ => unreachable!(),
                    };
                    (name, String::default())
                }
                _ => unreachable!("Missing match arm for instruction {instruction:?}"),
            };
            writeln!(f, "#{addr} {name:<5} {args:>25}")?;
        }

        Ok(())
    }
}

impl From<BytecodeError> for std::fmt::Error {
    fn from(_: BytecodeError) -> Self {
        Self
    }
}

#[cfg(test)]
mod test {
    use crate::stack::RegisterContents;

    #[test]
    fn test_sizes() {
        use std::mem::size_of;
        assert_eq!(size_of::<super::Instruction>(), 6);
        assert_eq!(size_of::<super::Chunk>(), 40)
    }
}
