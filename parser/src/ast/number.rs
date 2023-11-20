use crate::tokens::Span;

use super::{ Parse, Spanned, ParseResult, stream::ParseStream };

#[derive(Debug, Clone)]
enum Number {
    Int(i64),
    UInt(u64),
    Float(f64),
}

impl PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Int(l0), Self::Int(r0)) => l0 == r0,
            (Self::UInt(l0), Self::UInt(r0)) => l0 == r0,
            (Self::Float(l0), Self::Float(r0)) => l0 == r0,
            _ => false,
        }
    }
}

impl Eq for Number {}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct LitNumber {
    number: Number,
    span: Span,
}

impl Spanned for LitNumber {
    fn span(&self) -> Span {
        self.span
    }
}

impl Parse for LitNumber {
    fn parse(token: &mut ParseStream) -> ParseResult<Self> where Self: Sized {
        todo!()
    }
}
