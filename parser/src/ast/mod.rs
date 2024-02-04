use thiserror::Error;

use crate::{ Span, tokens::{ TokenType, Token } };

use self::stream::{ ParseStream, TokenCursor };

pub mod keywords;
pub mod number;
pub mod stream;
pub mod expr;
pub mod ident;
pub mod string;
pub mod punct;
pub mod conditionals;
pub mod block;
pub mod loops;

pub mod grouped;
pub mod separated;
pub mod functions;
pub mod declarations;
pub mod path;
pub mod statements;
pub mod modules;
pub mod simple;

pub struct Ast;

pub trait Spanned {
    fn span(&self) -> Span;
}

#[derive(Debug, PartialEq, Eq, Clone, Error)]
pub enum ParseError {
    #[error("Unexpected `{}`. Expected `{expected}`", got.content)] UnexpectedToken {
        expected: &'static str,
        got: Token,
    },
    #[error("Invalid literal `{0}`")] BadLiteral(String),
}

type ParseResult<T> = Result<T, ParseError>;

pub trait Parse: Spanned {
    fn parse(stream: &mut ParseStream) -> ParseResult<Self> where Self: Sized;

    fn could_parse(stream: &mut ParseStream) -> bool;
}
