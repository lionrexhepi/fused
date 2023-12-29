use std::{ fmt::Formatter, path::Display };

type Register = u8;
use crate::RuntimeError;

use super::Result;

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum Instruction {
    Return(Register),
    Add {
        left: Register,
        right: Register,
        dst: Register,
    },
    Sub {
        left: Register,
        right: Register,
        dst: Register,
    },
}

impl Instruction {
    pub fn read_from_chunk(chunk: Chunk) -> Result<(Self, u8)> {
        match chunk[0] {
            0 => {
                if chunk.len() <= 1 {
                    Err(RuntimeError::InvalidChunkEnd)
                } else {
                    Ok((Self::Return(chunk[1]), 2))
                }
            }
            1 => {
                if chunk.len() < 4 {
                    Err(RuntimeError::InvalidChunkEnd)
                } else {
                    Ok((
                        Self::Add { left: chunk[1], right: chunk[2], dst: chunk[3] },
                        4, //1 byte we've already read + the 3 args
                    ))
                }
            }
            2 => {
                if chunk.len() < 4 {
                    Err(RuntimeError::InvalidChunkEnd)
                } else {
                    Ok((Self::Sub { left: chunk[1], right: chunk[2], dst: chunk[3] }, 4))
                }
            }

            other => Err(RuntimeError::InvalidInstruction(other)),
        }
    }
}

pub type Chunk<'a> = &'a [u8];

#[cfg(test)]
mod test {
    use std::io::stdin;

    #[test]
    fn test_sizes() {
        use std::mem::size_of;
        assert_eq!(size_of::<super::Register>(), 1);
        assert_eq!(size_of::<super::Instruction>(), 4);
    }
}
