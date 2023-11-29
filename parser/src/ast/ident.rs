use crate::tokens::Span;

use super::{Parse, stream::Cursor, ParseResult, ParseError};

pub struct Ident {
    pub name: String,
     span: Span,
}

impl super::Spanned for Ident {
    fn span(&self) -> Span {
        self.span
    }
}

impl Parse for Ident {
    fn parse(stream: &mut super::stream::ParseStream) -> super::ParseResult<Self> where Self: Sized {
        stream.parse_with(|cursor: &mut Cursor| {
            let token = cursor.current().clone();
            if let crate::tokens::TokenType::Ident(ident) = &token.content {
                
                if is_keyword(&ident.name) && !ident.escaped {
                    return Err(super::ParseError::UnexpectedToken(
                        "identifier".to_string(),
                        token,
                    ));
                }

                cursor.advance();
                Ok(Self {
                    name: ident.name.clone(),
                    span: token.span,
                })
            } else {
                Err(super::ParseError::UnexpectedToken(
                    "identifier".to_string(),
                    token,
                ))
            }
        })
    }
}

fn is_keyword(ident: &str) -> bool {
    match ident {
        "let" | "fn" | "if" | "else" | "while" | "for" | "in" | "return" | "break" | "continue" => true,
        _ => false,
    }
}

#[cfg(test)]
mod test {
    use crate::{tokens::stream::TokenStream, ast::Parse};

    #[test]
    fn test_ident() {
        let tokens = TokenStream::from_string("name".to_string()).unwrap();
        let mut stream = super::super::stream::ParseStream::new(tokens);

        let ident = super::Ident::parse(&mut stream).unwrap();
        assert_eq!(ident.name, "name");
    }

    #[test]
    fn test_keyword() {
        let tokens = TokenStream::from_string("let".to_string()).unwrap();
        let mut stream = super::super::stream::ParseStream::new(tokens);

        let ident = super::Ident::parse(&mut stream);
        assert!(ident.is_err());
    }

    #[test]
    fn test_escaped_keyword() {
        let tokens = TokenStream::from_string("@let".to_string()).unwrap();
        let mut stream = super::super::stream::ParseStream::new(tokens);

        let ident = super::Ident::parse(&mut stream).unwrap();
        assert_eq!(ident.name, "let");
    }
}