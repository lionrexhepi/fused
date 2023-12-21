use crate::Span;

use super::{
    ident::Ident,
    block::Block,
    Spanned,
    Parse,
    keywords::Fn,
    grouped::{ Parenthesized, Bracketed },
    separated::Separated,
    punct::Colon,
    stream::{ ParseStream, UnexpectedToken },
    ParseResult,
    declarations::FnArg,
    ParseError,
    path::ExprPath,
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ExprFunction {
    pub name: Ident,
    pub args: Vec<FnArg>,
    pub ret: Option<ExprPath>,
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
        let args = stream.parse::<Parenthesized<Separated<FnArg>>>()?;

        let ret = stream
            .parse::<Bracketed<ExprPath>>()
            .ok()
            .map(|bracketed| *bracketed.0);

        stream.parse::<Colon>()?;
        let body = stream.parse::<Block>()?;

        if body.0.is_empty() {
            return Err(ParseError::UnexpectedToken {
                expected: "block",
                got: stream.current().clone(),
            });
        }

        Ok(Self {
            name,
            args: args.0.into_iter().collect(),
            ret,
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
        let tokens = TokenStream::from_string("fn foo():\n1").unwrap();
        let mut stream = ParseStream::new(tokens);

        let func = stream.parse::<ExprFunction>().unwrap();

        assert!(matches!(func.name, Ident { .. }));
        assert!(func.args.is_empty());
        assert!(matches!(*func.body, Block { .. }));
    }

    #[test]
    fn test_simple_args() {
        let tokens = TokenStream::from_string("fn foo(a, b, c):\n1").unwrap();
        let mut stream = ParseStream::new(tokens);

        let func = stream.parse::<ExprFunction>().unwrap();

        assert!(matches!(func.name, Ident { .. }));
        assert_eq!(func.args.len(), 3);
        assert!(matches!(*func.body, Block { .. }));
    }

    #[test]
    fn test_empty_block_fails() {
        let tokens = TokenStream::from_string("fn foo():\n").unwrap();
        let mut stream = ParseStream::new(tokens);

        let func = stream.parse::<ExprFunction>();

        assert!(func.is_err());
    }

    #[test]
    fn test_return_type() {
        let tokens = TokenStream::from_string("fn foo()[i32]:\n1").unwrap();
        let mut stream = ParseStream::new(tokens);

        let func = stream.parse::<ExprFunction>().unwrap();

        assert!(matches!(func.name, Ident { .. }));
        assert!(func.args.is_empty());
        assert!(matches!(*func.body, Block { .. }));
        assert!(matches!(func.ret, Some(_)));
    }
}
