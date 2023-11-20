use std::{ fmt::Debug, ops::Range };

use crate::file::Cursor;

use self::{ literal::TokenLiteral, comment::TokenComment, group::TokenGroup };

mod literal;
mod ident;
pub mod spacing;
pub mod punct;
pub mod comment;
pub mod group;
pub mod stream;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl From<Range<usize>> for Span {
    fn from(range: Range<usize>) -> Self {
        Self { start: range.start, end: range.end }
    }
}

impl Span {
    pub fn len(&self) -> usize {
        self.end - self.start
    }
}

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
    pub fn try_read(cursor: &mut Cursor) -> Result<Self, TokenError> {
        cursor.skip_whitespaces();
        let start = cursor.pos();
        let content = if let Some(literal) = TokenLiteral::try_read(cursor)? {
            TokenType::Literal(literal)
        } else if let Some(ident) = ident::TokenIdent::try_read(cursor)? {
            TokenType::Ident(ident)
        } else if spacing::read_newline(cursor) {
            TokenType::Newline(spacing::count_spaces(cursor))
        } else if let Some(punct) = punct::TokenPunct::try_read(cursor)? {
            TokenType::Punct(punct)
        } else if let Some(comment) = comment::TokenComment::try_read(cursor)? {
            TokenType::Comment(comment)
        } else if let Some(group) = TokenGroup::try_read(cursor)? {
            TokenType::Group(group)
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
    Newline(usize),
    Comment(TokenComment),
    EOF,
}

#[derive(Debug, PartialEq, Eq)]
pub enum TokenError {
    InvalidChar(char),
    UnexpectedEof,
}

pub type TokenResult<T> = std::result::Result<Option<T>, TokenError>;

trait TokenContent: Clone + PartialEq + Eq + Debug {
    fn try_read(cursor: &mut Cursor) -> TokenResult<Self> where Self: Sized;
}

#[cfg(test)]
mod test {
    use crate::file::Cursor;

    #[test]
    fn test_span_start() {
        let mut cursor = Cursor::new("test");
        let token = super::Token::try_read(&mut cursor).unwrap();
        assert_eq!(token.span.start, 0);
    }

    #[test]
    fn test_span_end() {
        let mut cursor = Cursor::new("test");
        let token = super::Token::try_read(&mut cursor).unwrap();
        assert_eq!(token.span.end, 4);
    }

    #[test]
    fn test_nonzero_start() {
        let mut cursor = Cursor::new(" test");
        cursor.advance(); //Skip the whitespace
        let token = super::Token::try_read(&mut cursor).unwrap();
        assert_eq!(token.span.start, 1);
    }

    #[test]
    fn test_length() {
        let mut cursor = Cursor::new("test");
        let token = super::Token::try_read(&mut cursor).unwrap();
        assert_eq!(token.span.len(), 4);
    }
}
