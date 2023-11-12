use std::str::Chars;

pub const EOF: char = '\0';

pub struct Cursor<'a> {
    content: Chars<'a>,
    current: char,
    consumed: usize,
    absolute: usize,
}

impl<'a> Cursor<'a> {
    pub fn new(data: &'a str) -> Self {
        Self {
            content: data.chars(),
            current: EOF,
            consumed: 0,
            absolute: 0,
        }
    }

    pub fn begin_token(&mut self) {
        self.consumed = 0;
    }

    pub fn eof(&self) -> bool {
        self.content.as_str().is_empty()
    }

    pub fn relative_pos(&self) -> usize {
        self.consumed
    }

    pub fn current(&self) -> char {
        self.current
    }

    pub fn next(&self) -> char {
        self.content.clone().next().unwrap_or(EOF)
    }

    pub fn nth(&self, n: usize) -> char {
        self.content.clone().nth(n).unwrap_or(EOF)
    }

    pub fn advance(&mut self) -> Option<char> {
        let char = self.content.next()?;

        self.current = char;
        self.consumed += 1;
        self.absolute += 1;

        Some(char)
    }
}
