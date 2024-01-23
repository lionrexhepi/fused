use std::fmt::{ Formatter, Display };

use thiserror::Error;

use crate::{ stack::{ RegisterContents, Register }, instructions::Instruction };

pub type Result<T> = std::result::Result<T, BytecodeError>;

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum BytecodeError {
    #[error("Invalid instruction: {0:x}")] InvalidInstruction(u8),
    #[error("The register {0:x} exceeds the size of the current stack frame")] RegisterNotFound(
        Register,
    ),
    #[error("Chunk ends in the middle of an instruction")]
    UnexpectedEOF,
}

pub struct Chunk {
    pub consts: Vec<RegisterContents>,
    pub buffer: Box<[u8]>,
}

impl<'a> Display for Chunk {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{:-^30}", "Constants")?;

        for (index, value) in self.consts.iter().enumerate() {
            writeln!(f, "{:x}: {:?>30}", index, value)?;
        }

        writeln!(f, "{:-^30}", "Instructions")?;

        let mut ip = 0;
        println!("Raw: {:?}", self.buffer);
        while ip < self.buffer.len() {
            let instruction = Instruction::from_byte(self.buffer[ip])?;
            ip += 1;
            let (name, args) = match instruction {
                Instruction::Const => {
                    let (address, dest) = Instruction::read_constant(&self.buffer[ip..])?;
                    ip += 3;
                    ("const", format!("{:x} <{}>", address, dest))
                }
                Instruction::Return => {
                    ip += 2;
                    ("ret", format!("<{}> -> <{}>", self.buffer[ip - 2], self.buffer[ip - 1]))
                },
                Instruction::PushFrame => {
                    

                    ("pushframe", String::new())
                }
                    
                other if other.is_binary() => {
                    let (left, right, dest) = Instruction::read_binary_args(&self.buffer[ip..])?;
                    ip += 3;
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
                    (name, format!("<{}> <{}> <{}>", left, right, dest))
                }
                _ => unreachable!("Missing match arm for instruction {instruction:?}"),
            };
            writeln!(f, "{name:<5} {args:>25}")?;
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

    #[test]
    fn display() {
        let buffer = [
            1,
            0,
            0,
            0, //const [0] <0>
            1,
            0,
            1,
            1, //const [1] <1>
            2,
            0,
            1,
            2, //add <0> <1> <2>
            0,
            2, //return <2>
        ];
        let chunk = super::Chunk {
            buffer: Box::new(buffer),
            consts: vec![RegisterContents::Int(19), RegisterContents::Float(34f64)],
        };
        println!("{}", chunk);
    }
}
