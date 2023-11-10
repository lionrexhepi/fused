use crate::location::SourceLocation;

struct Token {
    location: SourceLocation,

    content: TokenType,
}

pub enum TokenType {
    Literal,
    Ident,
    Group,
    Punct,
    Space(usize),
    EOF,
}
