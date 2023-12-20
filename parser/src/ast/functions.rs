use crate::tokens::Span;

use super::{
    ident::Ident,
    block::Block,
    Spanned,
    Parse,
    keywords::Fn,
    grouped::Parenthesized,
    separated::Separated,
    punct::Colon,
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ExprFunction {
    pub name: Ident,
    pub args: Vec<Ident>,
    pub body: Box<Block>,
}

impl Spanned for ExprFunction {
    fn span(&self) -> Span {
        self.name.span().join(self.body.span())
    }
}

impl Parse for ExprFunction {
    fn parse(stream: &mut super::stream::ParseStream) -> super::ParseResult<Self> {
        stream.parse::<Fn>()?;
        let name = stream.parse::<Ident>()?;
        let args = stream.parse::<Parenthesized<Separated<Ident>>>()?;

        stream.parse::<Colon>()?;
        let body = stream.parse::<Block>()?;

        Ok(Self {
            name,
            args: args.0.into_iter().collect(),
            body: Box::new(body),
        })
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_simple_noargs() {
        let tokens = crate::tokens::stream::TokenStream
            ::from_string("fn foo():\n1".to_string())
            .unwrap();
        let mut stream = crate::ast::stream::ParseStream::new(tokens);

        let func = stream.parse::<crate::ast::functions::ExprFunction>().unwrap();

        assert!(matches!(func.name, crate::ast::ident::Ident { .. }));
        assert!(func.args.is_empty());
        assert!(matches!(*func.body, crate::ast::block::Block { .. }));
    }

    #[test]
    fn test_simple_args() {
        let tokens = crate::tokens::stream::TokenStream
            ::from_string("fn foo(a, b, c):\n1".to_string())
            .unwrap();
        let mut stream = crate::ast::stream::ParseStream::new(tokens);

        let func = stream.parse::<crate::ast::functions::ExprFunction>().unwrap();

        assert!(matches!(func.name, crate::ast::ident::Ident { .. }));
        assert_eq!(func.args.len(), 3);
        assert!(matches!(*func.body, crate::ast::block::Block { .. }));
    }

    #[test]
    fn test_empty_block_fails() {
        let tokens = crate::tokens::stream::TokenStream
            ::from_string("fn foo():\n".to_string())
            .unwrap();
        let mut stream = crate::ast::stream::ParseStream::new(tokens);

        let func = stream.parse::<crate::ast::functions::ExprFunction>();

        assert!(func.is_err());
    }
}
