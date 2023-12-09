use crate::tokens::Span;

use super::{ expr::Expr, Parse, punct::Colon, Newline, stream::ParseStream, ParseResult, Spanned };

#[derive(Debug, PartialEq, Eq, Clone)]
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
        println!("{:?}", token.cursor().current());
        token.parse::<Colon>()?;

        let width = token.parse::<Newline>()?.follwing_spaces;
        let mut exprs = vec![token.parse::<Expr>()?];

        while let Ok(Newline { follwing_spaces, .. }) = token.parse() {
            if follwing_spaces != width {
                break;
            }
            println!("eae");
            println!("{:?}", token.cursor().current());
            let expr = token.parse::<Expr>();
            exprs.push(expr?);
        }

        let span = exprs.iter().fold(Span::default(), |acc, expr| acc.join(expr.span()));

        Ok(Self {
            exprs,
            span,
        })
    }
}

#[cfg(test)]
mod test {
    use crate::{ tokens::stream::TokenStream, ast::{ stream::ParseStream, expr::Expr } };
    use super::ExprBlock;

    #[test]
    fn test_single_statement() {
        let tokens = TokenStream::from_string(":\n    1".to_string()).unwrap();

        let mut stream = ParseStream::new(tokens);

        let block = stream.parse::<ExprBlock>().unwrap();

        assert_eq!(block.exprs.len(), 1);
    }

    #[test]
    fn test_several_statements() {
        let tokens = TokenStream::from_string(":\n    1\n    2\n    3".to_string()).unwrap();

        let mut stream = ParseStream::new(tokens);

        let block = stream.parse::<ExprBlock>().unwrap();

        assert_eq!(block.exprs.len(), 3);
    }

    #[test]
    fn test_empty_block() {
        let tokens = TokenStream::from_string(":".to_string()).unwrap();

        let mut stream = ParseStream::new(tokens);

        let block = stream.parse::<ExprBlock>();

        assert!(block.is_err())
    }

    #[test]
    fn test_uneven_spacing() {
        let tokens = TokenStream::from_string(":\n    1\n    2\n 3".to_string()).unwrap();

        let mut stream = ParseStream::new(tokens);

        let block = stream.parse::<ExprBlock>().unwrap();

        assert_eq!(block.exprs.len(), 2)
    }

    #[test]
    fn test_nested_blocks() {
        let tokens = TokenStream::from_string(
            r#":
    1
    2
    :
        1
        2"#.to_string()
        ).unwrap();

        let mut stream = ParseStream::new(tokens);

        let block = stream.parse::<ExprBlock>().unwrap();

        assert_eq!(block.exprs.len(), 3);
        let inner = block.exprs.last().unwrap();

        assert!(matches!(inner, Expr::Block(ExprBlock { exprs, .. }) if exprs.len() == 2));
    }
}
