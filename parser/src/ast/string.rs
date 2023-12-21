use crate::tokens::{ Span, TokenType, literal::TokenLiteral };

use super::{ Spanned, Parse, ParseError, ParseResult, stream::{ ParseStream, Cursor } };

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LitString {
    pub value: String,
    span: Span,
}

impl Spanned for LitString {
    fn span(&self) -> Span {
        self.span
    }
}

impl Parse for LitString {
    fn parse(stream: &mut ParseStream) -> ParseResult<Self> where Self: Sized {
        stream.parse_with(|cursor: &mut Cursor| {
            let token = cursor.current().clone();
            if let TokenType::Literal(TokenLiteral::String(string)) = &token.content {
                cursor.advance();
                Ok(Self {
                    value: string.content.clone(),
                    span: token.span,
                })
            } else {
                Err(ParseError::UnexpectedToken("string literal", token))
            }
        })
    }
}

#[cfg(test)]
mod test {
    use crate::{ tokens::stream::TokenStream, ast::Parse };

    use super::{ super::stream::ParseStream, LitString };

    #[test]
    fn test_string() {
        let tokens = TokenStream::from_string("\"Hello, world!\"".to_string()).unwrap();
        let mut stream = ParseStream::new(tokens);
        let string = LitString::parse(&mut stream).unwrap();
        assert_eq!(string.value, "Hello, world!");
    }
}
