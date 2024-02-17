use crate::Span;

use super::{
    stream::ParseStream,
    ParseResult,
    Spanned,
    Parse,
    block::Block,
    keywords::{ If, self, Keyword },
    punct::Colon,
    simple::ExprSimple,
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ExprIf {
    pub condition: Box<ExprSimple>,
    pub body: Block,
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
        let condition = token.parse()?;
        token.parse::<Colon>()?;
        token.expect_newline()?;
        let mut content = Vec::new();
        let r#else = loop {
            if keywords::End::could_parse(token) {
                break None;
            } else if Else::could_parse(token) {
                break Some(token.parse()?);
            } else {
                content.push(token.parse()?);
            }
            token.expect_newline()?;
        };
        let body = Block(content);
        let span = match &r#else {
            Some(Else::If(expr)) => r#if.span().join(expr.span()),
            Some(Else::Body(block)) => r#if.span().join(block.span()),
            None => r#if.span().join(body.span()),
        };

        Ok(Self {
            condition: Box::new(condition),
            body,
            r#else,
            span,
        })
    }

    fn could_parse(stream: &mut ParseStream) -> bool {
        If::could_parse(stream)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Else {
    If(Box<ExprIf>),
    Body(Box<Block>),
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
            stream.parse::<Colon>()?;
            Ok(Self::Body(Box::new(stream.parse()?)))
        }
    }

    fn could_parse(stream: &mut ParseStream) -> bool {
        keywords::Else::could_parse(stream)
    }
}

#[cfg(test)]
mod test {
    use crate::{
        ast::{
            conditionals::{ Else, ExprIf },
            expr::{ Expr, ExprLit },
            simple::ExprSimple,
            stream::ParseStream,
            Ast,
        },
        tokens::stream::TokenStream,
    };

    #[test]
    fn test_if() {
        let tokens = TokenStream::from_string("if true:\n    1\nend").unwrap();

        let mut stream = ParseStream::new(tokens);

        let r#if = stream.parse::<ExprIf>().unwrap();

        assert!(matches!(*r#if.condition, ExprSimple::Literal(ExprLit::Bool(_))));
    }

    #[test]
    fn test_if_else() {
        let tokens = TokenStream::from_string("if true:\n    1\nelse:\n    2").unwrap();

        let mut stream = ParseStream::new(tokens);

        let r#if = stream.parse::<ExprIf>().unwrap();

        assert!(matches!(*r#if.condition, ExprSimple::Literal(ExprLit::Bool(_))));
        assert!(matches!(r#if.r#else.unwrap(), Else::Body(_)));
    }

    #[test]
    fn test_if_else_if() {
        let tokens = TokenStream::from_string("if true:\n    1\nelse if false:\n    2").unwrap();

        let mut stream = ParseStream::new(tokens);

        let r#if = stream.parse::<ExprIf>().unwrap();

        assert!(matches!(*r#if.condition, ExprSimple::Literal(ExprLit::Bool(_))));
        assert!(matches!(r#if.r#else.unwrap(), Else::If(_)));
    }

    #[test]
    fn test_nested() {
        let tokens = TokenStream::from_string(
            "if true:\n  if false:\n        1\nend\n    else:\n        2\nend\nelse:\n    3\nend\n"
        ).unwrap();

        let mut stream = ParseStream::new(tokens);

        let r#if = stream.parse::<ExprIf>().unwrap();

        assert!(matches!(*r#if.condition, ExprSimple::Literal(ExprLit::Bool(_))));
        assert!(matches!(r#if.r#else.as_ref().unwrap(), Else::Body(_)));
        assert!(matches!(r#if.r#else.unwrap(), Else::Body(block) if block.0.len() == 1));
    }

    #[test]
    fn test_nested_chained() {
        let tokens = TokenStream::from_string(
            "if true:\n  if false:\n        1\nend\n    else:\n        2\nend\nelse:\n    3\nend\nb:=4"
        ).unwrap();

        let mut stream = ParseStream::new(tokens);
        let ast = Ast::from_tokens(&mut stream).unwrap();

        assert_eq!(ast.items.len(), 2);
        let r#if = match &ast.items[0].content {
            crate::ast::statements::StatementContent::Expr(expr) => expr,
            _ => panic!(),
        };
        assert!(matches!(*r#if, Expr::If(ExprIf { .. })));
    }
}
