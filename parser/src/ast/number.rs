use crate::tokens::Span;

use super::{ Expr, Spanned };

enum Number {
    Int(i64),
    UInt(u64),
    Float(f64),
}

pub struct ExprNumber {
    number: Number,
    span: Span,
}

impl Spanned for ExprNumber {
    fn span(&self) -> Span {
        self.span
    }
}

impl Expr for ExprNumber {
    fn from_token(token: &crate::tokens::Token) -> Option<Self> where Self: Sized {
        todo!()
    }
}
