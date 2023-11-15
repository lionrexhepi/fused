use crate::file::Cursor;

use self::literal::TokenLiteral;

mod literal;
mod ident;

pub struct Token {
    pub content: TokenType,
}

pub enum TokenType {
    Literal(TokenLiteral),
    Ident(ident::TokenIdent),
    Group,
    Punct,
    Space(usize),
    Comment,
    EOF,
}

fn is_eof(cursor: &Cursor) -> bool {
    cursor.eof()
}

