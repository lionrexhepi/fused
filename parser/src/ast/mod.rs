use crate::tokens::{ Span, Token };

pub mod keywords;
pub mod number;
pub mod stream;

pub struct Ast;

pub trait Spanned {
    fn span(&self) -> Span;
}

pub trait Expr: Spanned {
    fn from_token(token: &Token) -> Option<Self> where Self: Sized;
}
