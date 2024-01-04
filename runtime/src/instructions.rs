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
    Mul {
        left: Register,
        right: Register,
        dst: Register,
    } = Self::MUL,
    Div {
        left: Register,
        right: Register,
        dst: Register,
    } = Self::DIV,
    Mod {
        left: Register,
        right: Register,
        dst: Register,
    } = Self::MOD,
    BitAnd {
        left: Register,
        right: Register,
        dst: Register,
    } = Self::BITAND,
    BitOr {
        left: Register,
        right: Register,
        dst: Register,
    } = Self::BITOR,
    BitXor {
        left: Register,
        right: Register,
        dst: Register,
    } = Self::BITXOR,
    LeftShift {
        left: Register,
        right: Register,
        dst: Register,
    } = Self::LEFTSHIFT,
    RightShift {
        left: Register,
        right: Register,
        dst: Register,
    } = Self::RIGHTSHIFT,
    Or {
        left: Register,
        right: Register,
        dst: Register,
    } = Self::OR,
    And {
        left: Register,
        right: Register,
        dst: Register,
    } = Self::AND,
    Eq {
        left: Register,
        right: Register,
        dst: Register,
    } = Self::EQ,
}

impl Instruction {
    pub const RETURN: u8 = 0;
    pub const CONST: u8 = 1;
    pub const ADD: u8 = 2;
    pub const SUB: u8 = 3;
    pub const MUL: u8 = 4;
    pub const DIV: u8 = 5;
    pub const MOD: u8 = 6;
    pub const BITAND: u8 = 7;
    pub const BITOR: u8 = 8;
    pub const BITXOR: u8 = 9;
    pub const LEFTSHIFT: u8 = 10;
    pub const RIGHTSHIFT: u8 = 11;
    pub const OR: u8 = 12;
    pub const AND: u8 = 13;
    pub const EQ: u8 = 14;

    pub fn read_from_chunk(buffer: &[u8]) -> Result<(Self, u8)> {
        match buffer[0] {
            Self::RETURN => {
                if buffer.len() < 2 {
                    Err(RuntimeError::InvalidChunkEnd)
                } else {
                    Ok((Self::Return(buffer[1]), 2))
                }
            }
            Self::CONST => {
                if buffer.len() < 4 {
                    Err(RuntimeError::InvalidChunkEnd)
                } else {
                    let const_index = u16::from_le_bytes([buffer[1], buffer[2]]);
                    Ok((Self::Const(const_index, buffer[3]), 4))
                }
            }
            Self::ADD => {
                if buffer.len() < 4 {
                    Err(RuntimeError::InvalidChunkEnd)
                } else {
                    Ok((
                        Self::Add { left: buffer[1], right: buffer[2], dst: buffer[3] },
                        4, //1 byte we've already read + the 3 args
                    ))
                }
            }
            Self::SUB => {
                if buffer.len() < 4 {
                    Err(RuntimeError::InvalidChunkEnd)
                } else {
                    Ok((Self::Sub { left: buffer[1], right: buffer[2], dst: buffer[3] }, 4))
                }
            }
            Self::MUL => {
                if buffer.len() < 4 {
                    Err(RuntimeError::InvalidChunkEnd)
                } else {
                    Ok((Self::Mul { left: buffer[1], right: buffer[2], dst: buffer[3] }, 4))
                }
            }
            Self::DIV => {
                if buffer.len() < 4 {
                    Err(RuntimeError::InvalidChunkEnd)
                } else {
                    Ok((Self::Div { left: buffer[1], right: buffer[2], dst: buffer[3] }, 4))
                }
            }
            Self::MOD => {
                if buffer.len() < 4 {
                    Err(RuntimeError::InvalidChunkEnd)
                } else {
                    Ok((Self::Mod { left: buffer[1], right: buffer[2], dst: buffer[3] }, 4))
                }
            }
            Self::BITAND => {
                if buffer.len() < 4 {
                    Err(RuntimeError::InvalidChunkEnd)
                } else {
                    Ok((Self::BitAnd { left: buffer[1], right: buffer[2], dst: buffer[3] }, 4))
                }
            }
            Self::BITOR => {
                if buffer.len() < 4 {
                    Err(RuntimeError::InvalidChunkEnd)
                } else {
                    Ok((Self::BitOr { left: buffer[1], right: buffer[2], dst: buffer[3] }, 4))
                }
            }
            Self::BITXOR => {
                if buffer.len() < 4 {
                    Err(RuntimeError::InvalidChunkEnd)
                } else {
                    Ok((Self::BitXor { left: buffer[1], right: buffer[2], dst: buffer[3] }, 4))
                }
            }
            Self::LEFTSHIFT => {
                if buffer.len() < 4 {
                    Err(RuntimeError::InvalidChunkEnd)
                } else {
                    Ok((Self::LeftShift { left: buffer[1], right: buffer[2], dst: buffer[3] }, 4))
                }
            }
            Self::RIGHTSHIFT => {
                if buffer.len() < 4 {
                    Err(RuntimeError::InvalidChunkEnd)
                } else {
                    Ok((Self::RightShift { left: buffer[1], right: buffer[2], dst: buffer[3] }, 4))
                }
            }
            Self::OR => {
                if buffer.len() < 4 {
                    Err(RuntimeError::InvalidChunkEnd)
                } else {
                    Ok((Self::Or { left: buffer[1], right: buffer[2], dst: buffer[3] }, 4))
                }
            }
            Self::AND => {
                if buffer.len() < 4 {
                    Err(RuntimeError::InvalidChunkEnd)
                } else {
                    Ok((Self::And { left: buffer[1], right: buffer[2], dst: buffer[3] }, 4))
                }
            }
            Self::EQ => {
                if buffer.len() < 4 {
                    Err(RuntimeError::InvalidChunkEnd)
                } else {
                    Ok((Self::Eq { left: buffer[1], right: buffer[2], dst: buffer[3] }, 4))
                }
            }

            other => Err(RuntimeError::InvalidInstruction(other)),
        }
    }
}
