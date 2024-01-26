use crate::{ chunk::BytecodeError, instructions::Instruction, stack::Register };

pub struct BufReader<'a> {
    ip: usize,
    buf: &'a [u8],
}

pub type Index = u16;

impl<'a> BufReader<'a> {
    pub fn new(buffer: &'a [u8]) -> Self {
        Self { ip: 0, buf: buffer }
    }

    fn next_byte(&mut self) -> Result<u8, BytecodeError> {
        if self.ip >= self.buf.len() {
            return Err(BytecodeError::UnexpectedEOF);
        }
        let byte = self.buf[self.ip];
        self.ip += 1;
        Ok(byte)
    }

    pub fn eof(&self) -> bool {
        self.ip >= self.buf.len()
    }

    pub fn read_instruction(&mut self) -> Result<Instruction, BytecodeError> {
        Instruction::from_byte(self.next_byte()?)
    }

    pub fn read_register(&mut self) -> Result<Register, BytecodeError> {
        Ok(Register::new(self.next_byte()?))
    }

    pub fn read_index(&mut self) -> Result<Index, BytecodeError> {
        Ok(Index::from_le_bytes([self.next_byte()?, self.next_byte()?]))
    }
}
