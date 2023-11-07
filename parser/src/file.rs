use std::{ ascii::AsciiExt, collections::VecDeque, io::{ BufRead, BufReader, Read } };

use crate::location::SourceLocation;

pub type Result<T> = std::result::Result<T, SourceFileError>;

pub struct SourceFile<'a> {
    pub name: String,
    content: BufReader<Box<dyn Read + 'a>>,
    buffer: VecDeque<char>,
    pos: SourceLocation,
    buffer_pos: usize,
}

pub enum SourceFileError {
    IoError(std::io::Error),
    EOF,
}

impl From<std::io::Error> for SourceFileError {
    fn from(err: std::io::Error) -> Self {
        Self::IoError(err)
    }
}

impl<'a> SourceFile<'a> {
    pub fn new(name: String, content: impl Read + 'a) -> Self {
        Self {
            name,
            content: BufReader::new(Box::new(content)),
            buffer: VecDeque::with_capacity(255),
            pos: SourceLocation::default(),
            buffer_pos: 0,
        }
    }

    fn fill_buffer(&mut self) -> Result<()> {
        let mut buf = vec![0; 255 - self.buffer.len()];
        let len = self.content.read(&mut buf)?;

        self.buffer.extend(
            buf
                .into_iter()
                .take(len)
                .map(|c| c as char)
        );

        Ok(())
    }

    pub fn peek(&self) -> Result<(char, SourceLocation)> {
        match self.buffer.front() {
            Some(c) => Ok((*c, self.pos)),
            None => Err(SourceFileError::EOF),
        }
    }

    pub fn next(&mut self) -> Result<(char, SourceLocation)> {
        let (c, pos) = self.peek()?;

        self.buffer.pop_front();

        if self.buffer.is_empty() {
            self.fill_buffer()?;
        }

        if c == '\n' {
            self.pos.newline();
        } else {
            self.pos.advance();
        }

        Ok((c, pos))
    }

    pub fn until(&mut self, condition: impl Fn(char) -> bool) -> Result<(String, SourceLocation)> {
        let mut buf = String::new();

        loop {
            let (c, pos) = match self.peek() {
                Ok((c, pos)) => (c, pos),
                Err(SourceFileError::EOF) => {
                    break Ok((buf, self.pos));
                }
                Err(SourceFileError::IoError(err)) => {
                    return Err(SourceFileError::IoError(err));
                }
            };

            if condition(c) {
                break Ok((buf, pos));
            }

            buf.push(c);
        }
    }

    pub fn next_seq(&mut self, len: usize) -> Result<(String, SourceLocation)> {
        let mut buf = String::new();

        for _ in 0..len {
            let (c, pos) = self.next()?;

            buf.push(c);
        }

        Ok((buf, self.pos))
    }

    pub fn next_if(
        &mut self,
        condition: impl Fn(char) -> bool
    ) -> Result<Option<(char, SourceLocation)>> {
        let (c, pos) = self.peek()?;

        if condition(c) {
            self.next()?;
            Ok(Some((c, pos)))
        } else {
            Ok(None)
        }
    }

    pub fn next_is(&mut self, ch: char) -> Result<bool> {
        let (c, pos) = self.peek()?;

        if c == ch {
            self.next()?;
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
