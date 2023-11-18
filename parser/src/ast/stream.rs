use crate::tokens::{ stream::TokenStream, Token };

use super::{ keywords::Keyword, Expr };

pub struct ParseStream {
    tokens: TokenStream,
}

pub struct UnexpectedToken(String, Token);

type ParseResult<T> = Result<T, UnexpectedToken>;

impl ParseStream {
    pub fn is_keyword<K: Keyword>(&self) -> bool {
        self.tokens.current().map_or(false, |token| K::from_token(token).is_some())
    }

    pub fn keyword<K: Keyword>(&mut self) -> ParseResult<K> {
        let token = self.tokens
            .advance()
            .expect("A ParseStream shouldn't be read after an EOF has been detected!");
        K::from_token(&token).ok_or_else(|| UnexpectedToken(K::name().to_string(), token))
    }
}
