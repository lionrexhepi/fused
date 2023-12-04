use crate::tokens::Span;

use super::{
    expr::Expr,
    Parse,
    punct::Colon,
    Newline,
    stream::{ Parser, ParseStream },
    ParseResult,
    Spanned,
};

pub struct ExprBlock {
    pub exprs: Vec<Expr>,
    span: Span,
}
impl Spanned for ExprBlock {
    fn span(&self) -> Span {
        self.span
    }
}

impl Parse for ExprBlock {
    fn parse(token: &mut ParseStream) -> ParseResult<Self> where Self: Sized {
        token.parse::<Colon>()?;
        let mut width = -1;
        let mut exprs = Vec::new();
        loop {
            let newline = token.parse::<Newline>()?;
            if width == -1 {
                width = newline.follwing_spaces as i32;
            } else if width != (newline.follwing_spaces as i32) {
                break;
            } else {
                exprs.push(token.parse::<Expr>()?);
            }
        }

        let span = exprs.iter().fold(Span::default(), |acc, expr| acc.join(expr.span()));

        Ok(Self {
            exprs,
            span,
        })
    }
}
