use std::{ ascii::AsciiExt, collections::VecDeque, io::{ BufRead, BufReader, Read }, str::Chars };

use crate::location::SourceLocation;

pub type SourceChar = Option<char>;

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

    pub fn second_next(&self) -> char {
        self.content.clone().nth(1).unwrap_or(EOF)
    }

    pub fn advance(&mut self) -> Option<char> {
        let char = self.content.next()?;

        self.current = char;

        Some(char)
    }
}
