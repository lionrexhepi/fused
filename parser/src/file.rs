use std::{io::{BufReader, Read, BufRead}, collections::VecDeque, ascii::AsciiExt};

pub struct SourceFile<S: Read> {
    pub name: String,
    content: BufReader<S>,
    buffer: VecDeque<char>,
    pos: usize,
    buffer_pos: usize
}

pub enum SourceFileError {
    IoError(std::io::Error),
    EOF
}

impl From<std::io::Error> for SourceFileError {
    fn from(err: std::io::Error) -> Self {
        Self::IoError(err)
    }
}

impl<S: Read> SourceFile<S> {
    pub fn new(name: String, content: BufReader<S>) -> Self {
        Self {
            name,
            content,
            buffer: VecDeque::with_capacity(255),
            pos: 0,
            buffer_pos: 0
        }
    }

    fn fill_buffer(&mut self) -> Result<(), SourceFileError> {

        let mut buf = vec![0; 255 - self.buffer.len()];
        let len = self.content.read( &mut buf)?;

        self.buffer.extend(buf.into_iter().take(len).map(|c| c as char));

        Ok(())
    }



    pub fn next(&mut self) -> Result<(char, usize), SourceFileError> {
        self.fill_buffer()?;

        match self.buffer.pop_front() {
            Some(c) => {
                self.pos += 1;
                Ok((c, self.pos))
            },
            None => Err(SourceFileError::EOF)
        }

        
    }

    pub fn until(&mut self, condition: impl Fn(char) -> bool) -> Result<(String, usize), SourceFileError> {
        let mut buf = String::new();

        loop {
            let (c, pos) = match self.next() {
                Ok((c, pos)) => (c, pos),
                Err(SourceFileError::EOF) => break Ok((buf, self.pos)),
                Err(SourceFileError::IoError(err)) => return Err(SourceFileError::IoError(err))
            };

            if condition(c) {
                break Ok((buf, pos));
            }

            buf.push(c);
        }
    }
}