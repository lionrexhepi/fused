use crate::{ chunk::BytecodeError, instructions::Instruction };

pub struct BufReader<'a> {
    ip: Address,
    buf: &'a [u8],
}

pub type Index = u16;
pub type Address = usize;

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
        let byte = self.next_byte()?;
        let res = Instruction::from_byte(byte);
        res
    }

    pub fn read_index(&mut self) -> Result<Index, BytecodeError> {
        Ok(Index::from_le_bytes([self.next_byte()?, self.next_byte()?]))
    }

    pub fn read_address(&mut self) -> Result<Address, BytecodeError> {
        return Ok(
            Address::from_le_bytes([
                self.next_byte()?,
                self.next_byte()?,
                self.next_byte()?,
                self.next_byte()?,
                self.next_byte()?,
                self.next_byte()?,
                self.next_byte()?,
                self.next_byte()?,
            ])
        );
    }

    pub fn current_address(&self) -> Address {
        self.ip
    }

    ///Jumps to the provided address and returns the previous address.
    pub fn jump_to(&mut self, address: Address) -> Result<Address, BytecodeError> {
        if address >= self.buf.len() {
            return Err(BytecodeError::InvalidJumpAddress(address));
        }
        Ok(std::mem::replace(&mut self.ip, address))
    }
}
