use crate::tokens::{ Span, Token, TokenType };

use self::stream::{ ParseStream, Cursor };

pub mod keywords;
pub mod number;
pub mod stream;
pub mod expr;
pub mod ident;
pub mod string;
pub mod punct;
mod conditionals;
mod block;
pub mod loops;
pub mod operations;

pub struct Ast;

pub trait Spanned {
    fn span(&self) -> Span;
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ParseError {
    UnexpectedToken(&'static str, Token),
    BadLiteral(String),
}

type ParseResult<T> = Result<T, ParseError>;

pub trait Parse: Spanned {
    fn parse(stream: &mut ParseStream) -> ParseResult<Self> where Self: Sized;
}

pub struct Newline {
    follwing_spaces: usize,
    span: Span,
}

impl Spanned for Newline {
    fn span(&self) -> Span {
        self.span
    }
}

impl Parse for Newline {
    fn parse(stream: &mut ParseStream) -> ParseResult<Self> where Self: Sized {
        stream.parse_with(|cursor: &mut Cursor| {
            let token = cursor.current().clone();
            if let TokenType::Newline(spaces) = &token.content {
                cursor.advance();
                Ok(Self {
                    follwing_spaces: *spaces,
                    span: token.span,
                })
            } else {
                Err(ParseError::UnexpectedToken("newline", token.clone()))
            }
        })
    }
}
