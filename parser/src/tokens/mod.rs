

use crate::file::Cursor;

use self::{literal::TokenLiteral, comment::TokenComment, group::TokenGroup};

mod literal;
mod ident;
pub mod spacing;
pub mod punct;
pub mod comment;
pub mod group;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub fn len(&self) -> usize {
        self.end - self.start
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub content: TokenType,
    pub span: Span,
}

impl Token {
    pub fn try_read(cursor: &mut Cursor ) -> Option<Self> {

        let start = cursor.pos();
        let content = if let Some(literal) = TokenLiteral::try_read(cursor) {
            Some(TokenType::Literal(literal))
        } else if let Some(ident) = ident::TokenIdent::try_read(cursor) {
            Some(TokenType::Ident(ident))
        } else if spacing::read_newline(cursor)  {
            Some(TokenType::Newline)
        } else if let Some(punct) = punct::TokenPunct::try_read(cursor) {
            Some(TokenType::Punct(punct))
        } else if let Some(comment) = comment::TokenComment::try_read(cursor) {
            Some(TokenType::Comment(comment))
        } else if let Some(group) = TokenGroup::try_read(cursor){
            Some(TokenType::Group(group))
        }
        else if cursor.eof() {
            Some(TokenType::EOF)
        } else {
            let spaces = spacing::count_spaces(cursor);
            if spaces > 0 {
                Some(TokenType::Space(spaces))
            } else {
                cursor.advance();
                None
            }
        };

        content.map(|content| Self { content, span: Span { start, end: cursor.pos() } })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenType {
    Literal(TokenLiteral),
    Ident(ident::TokenIdent),
    Group(TokenGroup),
    Punct(punct::TokenPunct),
    Space(usize),
    Newline,
    Comment(TokenComment),
    EOF,
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
