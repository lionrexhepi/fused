use crate::{ tokens::TokenType, Span };

use super::{ statements::Statement, stream::ParseStream, Newline, Parse, ParseResult, Spanned };

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Block(pub Vec<Statement>);

impl Spanned for Block {
    fn span(&self) -> Span {
        self.0.first().unwrap().span().join(self.0.last().unwrap().span())
    }
}

impl Parse for Block {
    fn parse(stream: &mut ParseStream) -> ParseResult<Self> where Self: Sized {
        let first_indent = if let Ok(newline) = stream.parse::<Newline>() {
            newline.follwing_spaces
        } else {
            0
        };
        let first = stream.parse::<Statement>()?;
        let mut stmts = vec![first];
        while let TokenType::Newline(indent) = stream.current().content {
            if indent != first_indent {
                break;
            }

            stmts.push(stream.parse()?);
        }

        Ok(Self(stmts))
    }

    fn could_parse(stream: &mut ParseStream) -> bool {
        Statement::could_parse(stream)
    }
}

pub struct ExprBlock(pub Block);

impl Spanned for ExprBlock {
    fn span(&self) -> Span {
        self.0.span()
    }
}

#[cfg(test)]
mod test {
    use crate::{ tokens::stream::TokenStream, ast::stream::ParseStream };
    use super::Block;

    #[test]
    fn test_single_statement() {
        let tokens = TokenStream::from_string("\n    1").unwrap();

        let mut stream = ParseStream::new(tokens);

        let block = stream.parse::<Block>().unwrap();

        assert_eq!(block.0.len(), 1);
    }

    #[test]
    fn test_several_statements() {
        let tokens = TokenStream::from_string("\n    a:=1\n    b:=2\n    c:=3").unwrap();

        let mut stream = ParseStream::new(tokens);

        let block = stream.parse::<Block>().unwrap();

        assert_eq!(block.0.len(), 3);
    }

    #[test]
    fn test_empty_block() {
        let tokens = TokenStream::from_string("\n").unwrap();

        let mut stream = ParseStream::new(tokens);

        let block = stream.parse::<Block>();

        assert!(block.is_err())
    }

    #[test]
    fn test_uneven_spacing() {
        let tokens = TokenStream::from_string("\n    1\n    2\n 3").unwrap();

        let mut stream = ParseStream::new(tokens);

        let block = stream.parse::<Block>().unwrap();

        assert_eq!(block.0.len(), 2)
    }
}
