use crate::tokens::{ stream::TokenStream, Token };

use super::{ keywords::Keyword, Parse, ParseResult, ParseError };

pub struct ParseStream {
    tokens: TokenStream,
}

pub struct UnexpectedToken(String, Token);

impl ParseStream {
    pub fn is_keyword<K: Keyword>(&self) -> bool {
        self.tokens.current().map_or(false, |token| K::from_token(token).is_some())
    }

    pub fn keyword<K: Keyword>(&mut self) -> ParseResult<K> {
        let token = self.tokens
            .advance()
            .expect("A ParseStream shouldn't be read after an EOF has been detected!");
        K::from_token(&token).ok_or_else(||
            ParseError::UnexpectedToken(K::name().to_string(), token)
        )
    }

    pub fn advance(&mut self) -> Option<Token> {
        self.tokens.advance()
    }
}

#[cfg(test)]
mod test {
    use crate::{
        ast::{ keywords::{ If, Else }, stream::ParseStream },
        tokens::{ stream::TokenStream, Span },
    };

    #[test]
    fn test_is_keyword() {
        let mut stream = ParseStream {
            tokens: TokenStream::from_string("if else".to_string()).unwrap(),
        };
        assert!(stream.is_keyword::<If>());
        stream.advance();
        assert!(!stream.is_keyword::<If>());
    }

    #[test]
    fn test_keyword() {
        let mut stream = ParseStream {
            tokens: TokenStream::from_string("if else else".to_string()).unwrap(),
        };
        assert!(stream.keyword::<If>().is_ok());
        assert!(stream.keyword::<If>().is_err());
        assert_eq!(stream.keyword::<Else>(), Ok(Else((8..12).into())));
    }
}
