use crate::file::SourceCursor;

use super::{ Token, TokenType };

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TokenStream {
    inner: Vec<Token>,
    eof: Token,
}

impl TokenStream {
    pub fn from_vec(vec: Vec<Token>) -> Self {
        let last = vec.last().map_or(0, |token| token.span.start);
        let eof = Token {
            span: (last..last + 1).into(),
            content: TokenType::EOF,
        };

        Self {
            inner: vec,
            eof,
        }
    }

    pub fn from_string(string: impl ToString) -> Result<Self, super::TokenError> {
        let data = string.to_string();
        let mut cursor = SourceCursor::new(&data);

        let mut vec = Vec::new();
        let eof = loop {
            let token = Token::try_read(&mut cursor)?;

            if token.content == TokenType::EOF {
                break token;
            }

            vec.push(token);
        };

        Ok(Self { inner: vec, eof: eof })
    }

    pub fn push(&mut self, token: Token) {
        self.inner.push(token);
    }

    pub fn current(&self) -> &Token {
        self.inner.first().unwrap_or(&self.eof)
    }

    pub fn next(&self) -> &Token {
        self.inner.get(1).unwrap_or(&self.eof)
    }

    pub fn nth(&self, n: usize) -> &Token {
        self.inner.get(n).unwrap_or(&self.eof)
    }

    pub fn advance(&mut self) -> Token {
        if self.inner.is_empty() { self.eof.clone() } else { self.inner.remove(0) }
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
    use crate::tokens::{ TokenError, TokenType, stream::TokenStream };
    #[test]
    fn test_from_string() {
        let mut stream = TokenStream::from_string("test 1").unwrap();
        assert_eq!(stream.len(), 2); //Ident, Literal
        assert!(matches!(stream.advance().content, TokenType::Ident(_)));
        assert!(matches!(stream.advance().content, TokenType::Literal(_)));
        assert!(matches!(stream.advance().content, TokenType::EOF));
    }

    #[test]
    fn test_invalid_eof() {
        let stream = TokenStream::from_string("0x");
        assert_eq!(stream.err(), Some(TokenError::UnexpectedEof));
    }
}
