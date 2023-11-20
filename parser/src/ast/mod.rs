use crate::tokens::{ Span, Token };

use self::stream::ParseStream;

pub mod keywords;
pub mod number;
pub mod stream;
pub mod expr;

pub struct Ast;

pub trait Spanned {
    fn span(&self) -> Span;
}

pub enum ParseError {
    UnexpectedToken(String, Token),
}

type ParseResult<T> = Result<T, ParseError>;

pub trait Parse: Spanned {
    fn parse(stream: &mut ParseStream) -> ParseResult<Self> where Self: Sized;
}
