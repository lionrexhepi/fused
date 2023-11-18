use crate::file::Cursor;

use super::{ Token, TokenResult, TokenType };

#[derive(Default)]
pub struct TokenStream {
    inner: Vec<Token>,
}

impl TokenStream {
    pub fn from_string(string: String) -> Result<Self, super::TokenError> {
        let mut cursor = Cursor::new(&string);

        let mut vec = Vec::new();

        loop {
            let token = Token::try_read(&mut cursor)?;

            let eof = token.content == TokenType::EOF;

            vec.push(token);

            if eof {
                break;
            }
        }

        Ok(Self { inner: vec })
    }

    pub fn push(&mut self, token: Token) {
        self.inner.push(token);
    }

    pub fn current(&self) -> Option<&Token> {
        self.inner.first()
    }

    pub fn next(&self) -> Option<&Token> {
        self.inner.get(1)
    }

    pub fn advance(&mut self) -> Option<Token> {
        if self.inner.is_empty() { None } else { Some(self.inner.remove(0)) }
    }

    pub fn advance_to(&mut self, n: usize) -> Vec<Token> {
        self.inner.drain(0..n).collect()
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }
}

impl IntoIterator for TokenStream {
    type Item = Token;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

#[cfg(test)]
mod test {
    use crate::tokens::{ ident::TokenIdent, literal::LiteralNumber, Token, TokenError };

    #[test]
    fn test_from_string() {
        let mut stream = super::TokenStream::from_string("test 1".to_string()).unwrap();
        assert_eq!(stream.len(), 4); //Ident, Space, Literal, EOF
        assert!(matches!(stream.advance().unwrap().content, super::TokenType::Ident(_)));
        assert!(matches!(stream.advance().unwrap().content, super::TokenType::Space(_)));
        assert!(matches!(stream.advance().unwrap().content, super::TokenType::Literal(_)));
        assert!(matches!(stream.advance().unwrap().content, super::TokenType::EOF));
        assert!(stream.advance().is_none());
    }

    #[test]
    fn test_invalid_eof() {
        let stream = super::TokenStream::from_string("0x".to_string());
        assert_eq!(stream.err(), Some(TokenError::UnexpectedEof));
    }
}
