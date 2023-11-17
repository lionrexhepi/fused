

use crate::file::Cursor;

use self::{literal::TokenLiteral, comment::TokenComment};

mod literal;
mod ident;
pub mod spacing;
pub mod punct;
pub mod comment;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub content: TokenType,
}

impl Token {
    pub fn try_read(cursor: &mut Cursor ) -> Option<Self> {
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
        } else if cursor.eof() {
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

        content.map(|content| Self { content })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenType {
    Literal(TokenLiteral),
    Ident(ident::TokenIdent),
    Group,
    Punct(punct::TokenPunct),
    Space(usize),
    Newline,
    Comment(TokenComment),
    EOF,
}

