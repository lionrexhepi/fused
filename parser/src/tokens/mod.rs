use crate::file::Cursor;

use self::literal::TokenLiteral;

mod literal;

pub struct Token {
    content: TokenType,
}

pub enum TokenType {
    Literal(TokenLiteral),
    Ident(TokenIdent),
    Group,
    Punct,
    Space(usize),
    Comment,
    EOF,
}

pub struct TokenIdent {
    name: String,
    escaped: bool,
}

impl TokenIdent {
    pub const fn new(name: String) -> Self {
        Self { name, escaped: false }
    }

    pub fn try_read(cursor: &mut Cursor) -> Option<Self> {
        let escaped = if cursor.current() == '@' {
            cursor.advance();
            true
        } else {
            false
        };
        if is_valid_ident_start(cursor.current()) {
            let mut name = String::new();
            name.push(cursor.current());
            cursor.advance();

            while is_valid_ident_char(cursor.current()) {
                name.push(cursor.current());
                cursor.advance();
            }

            Some(Self { name, escaped })
        } else {
            None
        }
    }
}

fn is_valid_ident_start(c: char) -> bool {
    matches!(c, 'a'..='z' | 'A'..='Z' | '_')
}

fn is_valid_ident_char(c: char) -> bool {
    matches!(c, 'a'..='z' | 'A'..='Z' | '_' | '0'..='9')
}
