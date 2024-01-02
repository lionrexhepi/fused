use crate::{ RuntimeError, Result, stack::Register };

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Instruction {
    Return(Register) = Self::RETURN,
    Const(u16, Register) = Self::CONST,
    Add {
        left: Register,
        right: Register,
        dst: Register,
    } = Self::ADD,
    Sub {
        left: Register,
        right: Register,
        dst: Register,
    } = Self::SUB,
}

impl Instruction {
    pub const RETURN: u8 = 0;
    pub const CONST: u8 = 1;
    pub const ADD: u8 = 2;
    pub const SUB: u8 = 3;

    pub fn read_from_chunk(buffer: &[u8]) -> Result<(Self, u8)> {
        match buffer[0] {
            0 => {
                if buffer.len() < 2 {
                    Err(RuntimeError::InvalidChunkEnd)
                } else {
                    Ok((Self::Return(buffer[1]), 2))
                }
            }
            1 => {
                if buffer.len() < 4 {
                    Err(RuntimeError::InvalidChunkEnd)
                } else {
                    let const_index = u16::from_le_bytes([buffer[1], buffer[2]]);
                    Ok((Self::Const(const_index, buffer[3]), 4))
                }
            }
            2 => {
                if buffer.len() < 4 {
                    Err(RuntimeError::InvalidChunkEnd)
                } else {
                    Ok((
                        Self::Add { left: buffer[1], right: buffer[2], dst: buffer[3] },
                        4, //1 byte we've already read + the 3 args
                    ))
                }
            }
            3 => {
                if buffer.len() < 4 {
                    Err(RuntimeError::InvalidChunkEnd)
                } else {
                    Ok((Self::Sub { left: buffer[1], right: buffer[2], dst: buffer[3] }, 4))
                }
            }

            other => Err(RuntimeError::InvalidInstruction(other)),
        }
    }
}
