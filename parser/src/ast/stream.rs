use crate::tokens::{ stream::TokenStream, Token };

use super::{ keywords::Keyword, Parse, ParseResult };

pub struct ParseStream {
    tokens: TokenStream,
}

pub struct UnexpectedToken(String, Token);

impl ParseStream {
    pub fn is_keyword<K: Keyword>(&self) -> bool {
        self.tokens.current().map_or(false, |token| K::parse(token).is_ok())
    }

    pub fn keyword<K: Keyword>(&mut self) -> ParseResult<K> {
        let token = self.tokens
            .advance()
            .expect("A ParseStream shouldn't be read after an EOF has been detected!");
        K::parse(&token)
    }
}
