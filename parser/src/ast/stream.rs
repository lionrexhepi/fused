use crate::tokens::{ stream::TokenStream, Token, TokenType };

use super::{ Parse, ParseResult };

#[derive(Clone)]
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

    pub fn cursor(&self) -> TokenCursor {
        TokenCursor::new(&self.tokens)
    }

    pub fn parse_with<T>(&mut self, parser: impl Parser<T>) -> ParseResult<T> {
        let mut cursor = TokenCursor::new(&self.tokens);
        let result = parser.parse(&mut cursor);
        if result.is_ok() {
            self.tokens.advance_to(cursor.moved);
        }
        result
    }

    pub fn skip_newlines(&mut self) {
        while let TokenType::Newline(_) = self.tokens.current().content {
            self.tokens.advance();
        }
    }

    pub fn current(&self) -> &Token {
        self.tokens.current()
    }
}

pub trait Parser<T> {
    fn parse(self, stream: &mut TokenCursor) -> ParseResult<T>;
}

impl<T, F> Parser<T> for F where F: FnOnce(&mut TokenCursor) -> ParseResult<T> {
    fn parse(self, stream: &mut TokenCursor) -> ParseResult<T> {
        self(stream)
    }
}

#[derive(Clone)]
pub struct TokenCursor<'a> {
    tokenstream: &'a TokenStream,
    moved: usize,
}

impl<'a> TokenCursor<'a> {
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
