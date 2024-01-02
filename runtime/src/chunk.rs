use std::{ fmt::{ Formatter, Display } };

use crate::{ RuntimeError, stack::{ RegisterContents, Stack }, instructions::{ Instruction } };

use super::Result;

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
        while ip < self.buffer.len() {
            match Instruction::read_from_chunk(&self.buffer[ip..]) {
                Ok((instruction, offset)) => {
                    write!(f, "0x{:<5x}", ip)?;
                    let (name, args) = match instruction {
                        Instruction::Return(val) => ("return", format!("<{val:x}>")),
                        Instruction::Const(from, to) => ("const", format!("[{from:x}] <{to:x}>")),
                        Instruction::Add { left, right, dst } =>
                            ("add", format!("<{left:x}> <{right:x}> <{dst:x}>")),
                        Instruction::Sub { left, right, dst } =>
                            ("sub", format!("<{left:x}> <{right:x}> <{dst:x}>")),
                    };
                    write!(f, "{: <10}", name)?;
                    writeln!(f, "{: <15}", args)?;
                    ip += offset as usize;
                }
                Err(RuntimeError::InvalidChunkEnd) => {
                    writeln!(f, "{:x}: <invalid chunk end>", ip)?;
                    break;
                }
                Err(RuntimeError::InvalidInstruction(byte)) => {
                    writeln!(f, "{:x}: <invalid instruction: {:x}>", ip, byte)?;
                    ip += 1;
                }
                Err(_) => unreachable!(),
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::io::stdin;

    use crate::stack::RegisterContents;

    #[test]
    fn test_sizes() {
        use std::mem::size_of;
        assert_eq!(size_of::<super::Instruction>(), 6);
        assert_eq!(size_of::<super::Chunk>(), 40)
    }

    #[test]
    fn display() {
        let mut buffer = [
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
