use std::fmt::{ Debug, Display };

use thiserror::Error;

use crate::{ file::SourceCursor, Span };

use self::{
    comment::TokenComment,
    group::TokenGroup,
    literal::TokenLiteral,
    spacing::read_newline,
};

pub mod literal;
pub mod ident;
pub mod spacing;
pub mod punct;
pub mod comment;
pub mod group;
pub mod stream;

#[macro_export]
macro_rules! reject_eof {
    ($cursor:ident) => {
        if $cursor.eof() {
            return Err($crate::tokens::TokenError::UnexpectedEof);
        }
    };
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub content: TokenType,
    pub span: Span,
}

impl Token {
    pub fn try_read(cursor: &mut SourceCursor) -> Result<Self, TokenError> {
        cursor.skip_whitespaces();
        let start = cursor.pos();
        let content = if let Some(literal) = TokenLiteral::try_read(cursor)? {
            TokenType::Literal(literal)
        } else if let Some(ident) = ident::TokenIdent::try_read(cursor)? {
            TokenType::Ident(ident)
        } else if let Some(punct) = punct::TokenPunct::try_read(cursor)? {
            TokenType::Punct(punct)
        } else if let Some(comment) = comment::TokenComment::try_read(cursor)? {
            TokenType::Comment(comment)
        } else if let Some(group) = TokenGroup::try_read(cursor)? {
            TokenType::Group(group)
        } else if read_newline(cursor) {
            TokenType::Newline
        } else if cursor.eof() {
            TokenType::EOF
        } else {
            cursor.advance();
            return Err(TokenError::InvalidChar(cursor.current()));
        };

        Ok(Self { content, span: Span { start, end: cursor.pos() } })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenType {
    Literal(TokenLiteral),
    Ident(ident::TokenIdent),
    Group(TokenGroup),
    Punct(punct::TokenPunct),
    Comment(TokenComment),
    Newline,
    EOF,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenType::Ident(_) => write!(f, "<identifier>"),
            TokenType::Literal(_) => write!(f, "<literal>"),
            TokenType::Group(_) => write!(f, "<group>"),
            TokenType::Punct(_) => write!(f, "<punct>"),
            TokenType::Comment(_) => write!(f, "<comment>"),
            TokenType::Newline => write!(f, "<newline>"),
            TokenType::EOF => write!(f, "EOF"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Error)]
pub enum TokenError {
    #[error("Invalid character: {0}")] InvalidChar(char),
    #[error("Unexpected end of file")]
    UnexpectedEof,
}

pub type TokenResult<T> = std::result::Result<Option<T>, TokenError>;

trait TokenContent: Clone + PartialEq + Eq + Debug {
    fn try_read(cursor: &mut SourceCursor) -> TokenResult<Self> where Self: Sized;
}

#[cfg(test)]
mod test {
    use crate::file::SourceCursor;

    use super::Token;

    #[test]
    fn test_span_start() {
        let mut cursor = SourceCursor::new("test");
        let token = Token::try_read(&mut cursor).unwrap();
        assert_eq!(token.span.start, 0);
    }

    #[test]
    fn test_span_end() {
        let mut cursor = SourceCursor::new("test");
        let token = Token::try_read(&mut cursor).unwrap();
        assert_eq!(token.span.end, 4);
    }

    #[test]
    fn test_nonzero_start() {
        let mut cursor = SourceCursor::new(" test");
        cursor.advance(); //Skip the whitespace
        let token = Token::try_read(&mut cursor).unwrap();
        assert_eq!(token.span.start, 1);
    }

    #[test]
    fn test_length() {
        let mut cursor = SourceCursor::new("test");
        let token = Token::try_read(&mut cursor).unwrap();
        assert_eq!(token.span.len(), 4);
    }
}
