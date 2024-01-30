use crate::{ chunk::BytecodeError, codegen::scope::SymbolId, instructions::Instruction };

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

    pub fn read_symbol(&mut self) -> Result<SymbolId, BytecodeError> {
        Ok(
            SymbolId::from_le_bytes([
                self.next_byte()?,
                self.next_byte()?,
                self.next_byte()?,
                self.next_byte()?,
            ])
        )
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

#[cfg(test)]
mod test {
    use crate::instructions::Instruction;

    #[test]
    fn test_read_instruction() {
        let buffer = &[Instruction::Add as u8, Instruction::Sub as u8];
        let mut reader = super::BufReader::new(buffer);
        assert_eq!(reader.read_instruction().unwrap(), Instruction::Add);
        assert_eq!(reader.read_instruction().unwrap(), Instruction::Sub);
        assert!(reader.eof());
    }

    #[test]
    fn test_read_index() {
        let buffer = &[0x01, 0x00, 0x02, 0x00];
        let mut reader = super::BufReader::new(buffer);
        assert_eq!(reader.read_index().unwrap(), 1);
        assert_eq!(reader.read_index().unwrap(), 2);
        assert!(reader.eof());
    }

    #[test]
    fn test_read_symbol() {
        let buffer = &[0x01, 0x00, 0x00, 0x00];
        let mut reader = super::BufReader::new(buffer);
        assert_eq!(reader.read_symbol().unwrap(), 1);
        assert!(reader.eof());
    }

    #[test]
    fn test_read_address() {
        let buffer = &[0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
        let mut reader = super::BufReader::new(buffer);
        assert_eq!(reader.read_address().unwrap(), 1);
        assert!(reader.eof());
    }

    #[test]
    fn test_jump_to() {
        let buffer = &[Instruction::Const as u8, 0x00, 0x00, Instruction::Return as u8];
        let mut reader = super::BufReader::new(buffer);
        assert_eq!(reader.read_instruction().unwrap(), Instruction::Const);
        assert_eq!(reader.jump_to(3).unwrap(), 1);
        assert_eq!(reader.read_instruction().unwrap(), Instruction::Return);
        assert!(reader.eof());
    }
}
