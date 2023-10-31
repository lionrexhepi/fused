use crate::location::SourceLocation;


struct Ident(String);

enum Literal {

}

pub enum Punct {
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Caret,
    Ampersand,
    Pipe,
    Tilde,
    Question,
    Exclamation,
    Dot,
    Comma,
    Colon,
    SemiColon,
    Eq,
    NotEq,
    Lt,
    LtEq,
    Gt,
    GtEq,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    LParen,
    RParen,
    Arrow,
    FatArrow,
    Hash,
    Dollar,
    At,
    Underscore,
    Backslash,
    SingleQuote,
    DoubleQuote,

}

pub enum TokenType {
    Ident(Ident),
    Literal(Literal),
    Punct(Punct),
    Space(usize),
}


pub type Token = (TokenType, SourceLocation);

