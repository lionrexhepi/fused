use crate::tokens::{ Span, Token };

use super::{
    expr::Expr,
    stream::{ Parser, ParseStream },
    ParseResult,
    Spanned,
    Parse,
    block::ExprBlock,
    keywords::{ If, self, Keyword },
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
        let r#if = token.parse::<If>()?;
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
        stream.parse::<keywords::Else>()?;
        if If::from_token(stream.cursor().current()).is_some() {
            Ok(Self::If(Box::new(stream.parse()?)))
        } else {
            Ok(Self::Body(Box::new(stream.parse()?)))
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        tokens::stream::TokenStream,
        ast::{ stream::ParseStream, conditionals::ExprIf, expr::{ ExprLit, Expr } },
    };

    #[test]
    fn test_if() {
        let tokens = TokenStream::from_string("if true:\n    1".to_string()).unwrap();

        let mut stream = ParseStream::new(tokens);

        let r#if = stream.parse::<ExprIf>().unwrap();

        assert!(matches!(*r#if.condition, Expr::Literal(ExprLit::Bool(_))));
        assert!(matches!(*r#if.body, Expr::Block(_)));
    }

    #[test]
    fn test_if_else() {
        let tokens = TokenStream::from_string("if true:\n    1\nelse:\n    2".to_string()).unwrap();

        let mut stream = ParseStream::new(tokens);

        let r#if = stream.parse::<ExprIf>().unwrap();

        assert!(matches!(*r#if.condition, Expr::Literal(ExprLit::Bool(_))));
        assert!(matches!(*r#if.body, Expr::Block(_)));
        assert!(matches!(r#if.r#else.unwrap(), super::Else::Body(_)));
    }

    #[test]
    fn test_if_else_if() {
        let tokens = TokenStream::from_string(
            "if true:\n    1\nelse if false:\n    2".to_string()
        ).unwrap();

        let mut stream = ParseStream::new(tokens);

        let r#if = stream.parse::<ExprIf>().unwrap();

        assert!(matches!(*r#if.condition, Expr::Literal(ExprLit::Bool(_))));
        assert!(matches!(*r#if.body, Expr::Block(_)));
        assert!(matches!(r#if.r#else.unwrap(), super::Else::If(_)));
    }
}
