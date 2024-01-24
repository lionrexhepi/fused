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

    Load,
    LoadLocal,
    Store,
    StoreLocal,
}
macro_rules! match_bytes {
    
    ($byte:expr, $($instrs:ident),*) => {
        match $byte {
            $(byte if byte == (Self::$instrs as u8) => Ok(Self::$instrs)),*,
            _ => Err(BytecodeError::InvalidInstruction($byte)),
        }
    };
}

impl Instruction {
    pub fn read_binary_args(buffer: &[u8]) -> Result<(Register, Register, Register)> {
        if buffer.len() < 3 {
            Err(BytecodeError::UnexpectedEOF)?
        } else {
            Ok((buffer[0], buffer[1], buffer[2]))
        }
    }

    pub fn read_variable(buffer: &[u8]) -> Result<u16> {
        if buffer.len() < 2 {
            Err(BytecodeError::UnexpectedEOF)
        } else {
            Ok(u16::from_le_bytes([buffer[0], buffer[1]]))
        }
    }

    pub fn read_constant(buffer: &[u8]) -> Result<(u16, Register)> {
        if buffer.len() < 3 {
            Err(BytecodeError::UnexpectedEOF)
        } else {
            Ok((u16::from_le_bytes([buffer[0], buffer[1]]), buffer[2]))
        }
    }

    #[inline(always)]
    pub const fn is_binary(self) -> bool {
        (self as u8) <= (Instruction::Or as u8) && (self as u8) >= (Instruction::Add as u8)
    }

    pub const fn from_byte(byte: u8) -> Result<Self> {
        match_bytes!(
            byte,
            Return, // byte if byte == (Self::Return as u8) => Ok(Self::Return),
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
            Load,
            LoadLocal,
            Store,
            StoreLocal
        )
    }
}

