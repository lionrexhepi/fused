use std::str::Chars;

pub const EOF: char = '\0';

pub struct Cursor<'a> {
    content: Chars<'a>,

    absolute: usize,
}

impl<'a> Cursor<'a> {
    pub fn new(data: &'a str) -> Self {
        Self {
            content: data.chars(),

            absolute: 0,
        }
    }

    pub fn eof(&self) -> bool {
        self.content.as_str().is_empty()
    }

    pub fn pos(&self) -> usize {
        self.absolute
    }

    pub fn current(&self) -> char {
        self.content.clone().nth(0).unwrap_or(EOF)
    }

    pub fn next(&self) -> char {
        self.content.clone().nth(1).unwrap_or(EOF)
    }

    pub fn nth(&self, n: usize) -> char {
        self.content.clone().nth(n).unwrap_or(EOF)
    }

    pub fn skip_whitespaces(&mut self) {
        while matches!(self.current(), ' ' | '\t') {
            self.advance();
        }
    }

    pub fn advance(&mut self) -> char {
        self.absolute += 1;

        self.content.next().unwrap_or(EOF)
    }
}
