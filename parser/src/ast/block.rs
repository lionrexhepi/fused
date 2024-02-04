use crate::{ tokens::TokenType, Span };

use super::{
    keywords::{ self, End },
    punct::Colon,
    statements::Statement,
    stream::ParseStream,
    Parse,
    ParseResult,
    Spanned,
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Block(pub Vec<Statement>);

impl Spanned for Block {
    fn span(&self) -> Span {
        self.0.first().unwrap().span().join(self.0.last().unwrap().span())
    }
}

impl Parse for Block {
    fn parse(stream: &mut ParseStream) -> ParseResult<Self> where Self: Sized {
        let first = stream.parse::<Statement>()?;
        let mut stmts = vec![first];
        while !End::could_parse(stream) {
            stmts.push(stream.parse()?);
        }
        stream.parse::<End>()?;
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

impl Parse for ExprBlock {
    fn parse(stream: &mut ParseStream) -> ParseResult<Self> where Self: Sized {
        stream.parse::<keywords::Block>()?;
        stream.parse::<Colon>()?;
        Ok(Self(stream.parse()?))
    }

    fn could_parse(stream: &mut ParseStream) -> bool {
        keywords::Block::could_parse(stream)
    }
}

#[cfg(test)]
mod test {
    use crate::{ tokens::stream::TokenStream, ast::{ block::ExprBlock, stream::ParseStream } };
    use super::Block;

    #[test]
    fn test_single_statement() {
        let tokens = TokenStream::from_string("1 end").unwrap();

        let mut stream = ParseStream::new(tokens);

        let block = stream.parse::<Block>().unwrap();

        assert_eq!(block.0.len(), 1);
    }

    #[test]
    fn test_several_statements() {
        let tokens = TokenStream::from_string("a:=1\nb:=2\nc:=3 end").unwrap();

        let mut stream = ParseStream::new(tokens);

        let block = stream.parse::<Block>().unwrap();

        assert_eq!(block.0.len(), 3);
    }

    #[test]
    fn test_empty_block() {
        let tokens = TokenStream::from_string("\nend").unwrap();

        let mut stream = ParseStream::new(tokens);

        let block = stream.parse::<Block>();

        assert!(block.is_err())
    }

    #[test]
    fn test_expr_block() {
        let tokens = TokenStream::from_string("block:\n    1\n    2\n    3\nend").unwrap();

        let mut stream = ParseStream::new(tokens);

        let block = stream.parse::<ExprBlock>().unwrap();

        assert_eq!(block.0.0.len(), 3)
    }

    #[test]
    fn test_expr_block_no_colon() {
        let tokens = TokenStream::from_string("block\n    1\n    2\n    3\nend").unwrap();

        let mut stream = ParseStream::new(tokens);

        let block = stream.parse::<ExprBlock>();

        assert!(block.is_err())
    }

    #[test]
    fn test_expr_block_no_newline() {
        let tokens = TokenStream::from_string("block:    1\n    2\n    3\n end").unwrap();

        let mut stream = ParseStream::new(tokens);

        let block = stream.parse::<ExprBlock>();

        assert!(block.is_err())
    }
}
