use crate::Span;

use super::{
    ident::Ident,
    block::Block,
    Spanned,
    Parse,
    keywords::Fn,
    grouped::Parenthesized,
    separated::Separated,
    punct::Colon,
    stream::ParseStream,
    ParseResult,
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
    fn parse(stream: &mut ParseStream) -> ParseResult<Self> {
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
    use crate::{
        ast::{ ident::Ident, stream::ParseStream, functions::ExprFunction, block::Block },
        tokens::stream::TokenStream,
    };

    #[test]
    fn test_simple_noargs() {
        let tokens = TokenStream::from_string("fn foo():\n1".to_string()).unwrap();
        let mut stream = ParseStream::new(tokens);

        let func = stream.parse::<ExprFunction>().unwrap();

        assert!(matches!(func.name, Ident { .. }));
        assert!(func.args.is_empty());
        assert!(matches!(*func.body, Block { .. }));
    }

    #[test]
    fn test_simple_args() {
        let tokens = TokenStream::from_string("fn foo(a, b, c):\n1".to_string()).unwrap();
        let mut stream = ParseStream::new(tokens);

        let func = stream.parse::<ExprFunction>().unwrap();

        assert!(matches!(func.name, Ident { .. }));
        assert_eq!(func.args.len(), 3);
        assert!(matches!(*func.body, Block { .. }));
    }

    #[test]
    fn test_empty_block_fails() {
        let tokens = TokenStream::from_string("fn foo():\n".to_string()).unwrap();
        let mut stream = ParseStream::new(tokens);

        let func = stream.parse::<ExprFunction>();

        assert!(func.is_err());
    }
}
