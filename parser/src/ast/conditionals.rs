use crate::tokens::Span;

use super::{
    expr::Expr,
    stream::{ Parser, ParseStream },
    ParseResult,
    Spanned,
    Parse,
    block::ExprBlock,
};

pub struct ExprIf {
    pub condition: Box<Expr>,
    pub body: Box<Expr>,
    pub r#else: Option<Else>,
    span: Span,
}

impl Spanned for ExprIf {
    fn span(&self) -> Span {
        self.span
    }
}

impl Parse for ExprIf {
    fn parse(token: &mut ParseStream) -> ParseResult<Self> where Self: Sized {
        let r#if = token.parse::<ExprIf>()?;
        let condition = token.parse::<Expr>()?;
        let body = token.parse::<Expr>()?;
        let r#else = token.parse().ok();

        let span = match &r#else {
            Some(Else::If(expr)) => r#if.span().join(expr.span()),
            Some(Else::Body(block)) => r#if.span().join(block.span()),
            None => r#if.span().join(body.span()),
        };

        Ok(Self {
            condition: Box::new(condition),
            body: Box::new(body),
            r#else,
            span,
        })
    }
}

pub enum Else {
    If(Box<ExprIf>),
    Body(Box<ExprBlock>),
}

impl Spanned for Else {
    fn span(&self) -> Span {
        match self {
            Self::If(expr) => expr.span(),
            Self::Body(block) => block.span(),
        }
    }
}

impl Parse for Else {
    fn parse(stream: &mut ParseStream) -> ParseResult<Self> where Self: Sized {
        stream.parse::<Else>()?;
        if stream.parse::<ExprIf>().is_ok() {
            Ok(Self::If(Box::new(stream.parse()?)))
        } else {
            Ok(Self::Body(Box::new(stream.parse()?)))
        }
    }
}

#[cfg(test)]
mod test {
    use crate::file::Cursor;
}
