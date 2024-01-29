use crate::Span;

use super::{
    expr::Expr,
    block::Block,
    Spanned,
    Parse,
    keywords::{ Loop, While, In, For },
    ident::Ident,
    punct::Colon,
    stream::ParseStream,
    ParseResult,
    simple::ExprSimple,
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ExprLoop {
    pub body: Box<Block>,
    span: Span,
}

impl Spanned for ExprLoop {
    fn span(&self) -> Span {
        self.span
    }
}

impl Parse for ExprLoop {
    fn parse(token: &mut ParseStream) -> ParseResult<Self> where Self: Sized {
        let r#loop = token.parse::<Loop>()?;
        token.parse::<Colon>()?;
        let body = token.parse::<Block>()?;

        let span = r#loop.span().join(body.span());

        Ok(Self {
            body: Box::new(body),
            span,
        })
    }

    fn could_parse(stream: &mut ParseStream) -> bool {
        Loop::could_parse(stream)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ExprWhile {
    pub condition: Box<Expr>,
    pub body: Box<Block>,
    span: Span,
}

impl Spanned for ExprWhile {
    fn span(&self) -> Span {
        self.span
    }
}

impl Parse for ExprWhile {
    fn parse(token: &mut ParseStream) -> ParseResult<Self> where Self: Sized {
        let _while = token.parse::<While>()?;
        let condition = token.parse::<Expr>()?;

        token.parse::<Colon>()?;
        let body = token.parse::<Block>()?;
        let span = _while.span().join(body.span());

        Ok(Self {
            condition: Box::new(condition),
            body: Box::new(body),
            span,
        })
    }

    fn could_parse(stream: &mut ParseStream) -> bool {
        While::could_parse(stream)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ExprFor {
    pub ident: Box<Ident>,
    pub iter: Box<ExprSimple>,
    pub body: Box<Block>,
    span: Span,
}

impl Spanned for ExprFor {
    fn span(&self) -> Span {
        self.span
    }
}

impl Parse for ExprFor {
    fn parse(token: &mut ParseStream) -> ParseResult<Self> where Self: Sized {
        let _for = token.parse::<For>()?;
        let ident = token.parse::<Ident>()?;
        let _in = token.parse::<In>()?;
        let iter = token.parse()?;
        token.parse::<Colon>()?;
        let body = token.parse::<Block>()?;

        let span = _for.span().join(body.span());

        Ok(Self {
            ident: Box::new(ident),
            iter: Box::new(iter),
            body: Box::new(body),
            span,
        })
    }

    fn could_parse(stream: &mut ParseStream) -> bool {
        For::could_parse(stream)
    }
}

#[cfg(test)]
mod test {
    use crate::{
        ast::{ expr::{ ExprLit, Expr }, path::ExprPath, stream::ParseStream, simple::ExprSimple },
        tokens::stream::TokenStream,
    };

    use super::{ ExprWhile, ExprFor };

    #[test]
    fn test_loop() {
        let tokens = TokenStream::from_string("loop:\n    1").unwrap();

        let mut stream = ParseStream::new(tokens);

        let r#loop = stream.parse::<super::ExprLoop>().unwrap();

        assert_eq!((*r#loop.body).0.len(), 1);
    }

    #[test]
    fn test_while() {
        let tokens = TokenStream::from_string("while true:\n    1").unwrap();

        let mut stream = ParseStream::new(tokens);

        let r#while = stream.parse::<ExprWhile>().unwrap();

        assert!(matches!(*r#while.condition, Expr::Simple(ExprSimple::Literal(ExprLit::Bool(_)))));
        assert_eq!(r#while.body.0.len(), 1);
    }

    #[test]
    fn test_for() {
        let tokens = TokenStream::from_string("for i in array:\n    1").unwrap();

        let mut stream = ParseStream::new(tokens);

        let r#for = stream.parse::<ExprFor>().unwrap();

        assert_eq!(r#for.ident.name, "i");
        assert!(matches!(*r#for.iter, ExprSimple::Path(ExprPath { .. })));
        assert_eq!(r#for.body.0.len(), 1);
    }
}
