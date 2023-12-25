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
mod conditionals;
mod block;
pub mod loops;

pub mod grouped;
pub mod separated;
pub mod functions;
pub mod declarations;
pub mod path;
pub mod statements;
pub mod modules;
mod simple;

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

pub trait ParseDescend: Parse {
    type DescendTo: ParseDescend;
    fn parse_descend(stream: &mut ParseStream) -> ParseResult<(Self, bool)> where Self: Sized;

    fn unwrap_descend(self) -> Self::DescendTo;
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
        stream.parse_with(|cursor: &mut TokenCursor| {
            let token = cursor.current().clone();
            if let TokenType::Newline(spaces) = &token.content {
                cursor.advance();
                Ok(Self {
                    follwing_spaces: *spaces,
                    span: token.span,
                })
            } else {
                Err(ParseError::UnexpectedToken { expected: "newline", got: token.clone() })
            }
        })
    }

    fn could_parse(stream: &mut ParseStream) -> bool {
        matches!(stream.current().content, TokenType::Newline(_))
    }
}
