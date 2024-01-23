use crate::{ stack::Register, chunk::{ Result, BytecodeError } };

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Instruction {
    Return,
    Const,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    BitAnd,
    BitOr,
    BitXor,
    LeftShift,
    RightShift,
    Eq,
    And,
    Or,
    PushFrame,
    PopFrame,
    Load,
    LoadLocal,
    Store,
    StoreLocal,
}

impl Instruction {
    pub fn read_binary_args(buffer: &[u8]) -> Result<(Register, Register, Register)> {
        if buffer.len() < 3 {
            Err(BytecodeError::UnexpectedEOF)?
        } else {
            Ok((buffer[0], buffer[1], buffer[2]))
        }
    }

    pub fn read_constant(buffer: &[u8]) -> Result<(u16, Register)> {
        if buffer.len() < 3 {
            Err(BytecodeError::UnexpectedEOF)?
        } else {
            Ok((u16::from_le_bytes([buffer[0], buffer[1]]), buffer[2]))
        }
    }

    #[inline(always)]
    pub const fn is_binary(self) -> bool {
        (self as u8) <= (Instruction::Or as u8) && (self as u8) >= (Instruction::Add as u8)
    }

    pub const fn from_byte(byte: u8) -> Result<Self> {
        match byte {
            0 => Ok(Instruction::Return),
            1 => Ok(Instruction::Const),
            2 => Ok(Instruction::Add),
            3 => Ok(Instruction::Sub),
            4 => Ok(Instruction::Mul),
            5 => Ok(Instruction::Div),
            6 => Ok(Instruction::Mod),
            7 => Ok(Instruction::BitAnd),
            8 => Ok(Instruction::BitOr),
            9 => Ok(Instruction::BitXor),
            10 => Ok(Instruction::LeftShift),
            11 => Ok(Instruction::RightShift),
            12 => Ok(Instruction::Eq),
            13 => Ok(Instruction::And),
            14 => Ok(Instruction::Or),
            15 => Ok(Instruction::PushFrame),
            16 => Ok(Instruction::PopFrame),
            17 => Ok(Instruction::Load),
            18 => Ok(Instruction::LoadLocal),
            19 => Ok(Instruction::Store),
            20 => Ok(Instruction::StoreLocal),

            _ => Err(BytecodeError::InvalidInstruction(byte)),
        }
    }
}
