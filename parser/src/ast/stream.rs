use std::result;

use crate::tokens::{ stream::TokenStream, Token, TokenType };

use super::{ keywords::Keyword, Parse, ParseResult, ParseError };

pub struct ParseStream {
    tokens: TokenStream,
}

pub struct UnexpectedToken(String, Token);

impl ParseStream {
    pub fn new(tokens: TokenStream) -> Self {
        Self {
            tokens,
        }
    }
    pub fn parse<T: Parse>(&mut self) -> ParseResult<T> {
        T::parse(self)
    }

    pub fn parse_with<T>(&mut self, parser: impl Parser<T>) -> ParseResult<T> {
        let mut cursor = Cursor::new(&self.tokens);
        let result = parser.parse(&mut cursor);
        self.tokens.advance_to(cursor.moved);
        result
    }

    pub fn take<T>(&mut self) -> Token {
        self.tokens.advance()
    }
}

pub trait Parser<T> {
    fn parse(self, stream: &mut Cursor) -> ParseResult<T>;
}

impl<T, F> Parser<T> for F where F: FnOnce(&mut Cursor) -> ParseResult<T> {
    fn parse(self, stream: &mut Cursor) -> ParseResult<T> {
        self(stream)
    }
}

#[derive(Clone)]
pub struct Cursor<'a> {
    tokenstream: &'a TokenStream,
    moved: usize,
}

impl<'a> Cursor<'a> {
    fn new(tokenstream: &'a TokenStream) -> Self {
        Self {
            tokenstream,
            moved: 0,
        }
    }

    pub fn current(&self) -> &Token {
        self.tokenstream.nth(self.moved)
    }

    pub fn next(&self) -> &Token {
        self.tokenstream.nth(self.moved + 1)
    }

    pub fn nth(&self, n: usize) -> &Token {
        self.tokenstream.nth(self.moved + n)
    }

    pub fn advance(&mut self) -> bool {
        self.moved += 1;
        self.current().content != TokenType::EOF
    }

    pub fn split<T>(&mut self, parser: impl Parser<T>) -> ParseResult<T> {
        let mut cursor = self.clone();
        let result = parser.parse(&mut cursor);
        if result.is_ok() {
            self.moved = cursor.moved;
        }

        result
    }

    pub fn reset(&mut self) {
        self.moved = 0;
    }
}
