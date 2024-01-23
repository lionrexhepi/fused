use std::{ fmt::Display, hash::Hash };

use crate::{ Span, tokens::TokenType };

use super::{ Parse, stream::{ TokenCursor, ParseStream }, ParseError, ParseResult };

#[derive(Debug, Eq, Clone)]
pub struct Ident {
    pub name: String,
    span: Span,
}

impl Display for Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "`{}`", self.name)
    }
}

impl PartialEq for Ident {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Hash for Ident {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl super::Spanned for Ident {
    fn span(&self) -> Span {
        self.span
    }
}

impl Parse for Ident {
    fn parse(stream: &mut ParseStream) -> ParseResult<Self> where Self: Sized {
        stream.parse_with(|cursor: &mut TokenCursor| {
            let token = cursor.current().clone();
            if let TokenType::Ident(ident) = &token.content {
                if is_keyword(&ident.name) && !ident.escaped {
                    return Err(ParseError::UnexpectedToken {
                        expected: "identifier",
                        got: token,
                    });
                }

                cursor.advance();
                Ok(Self {
                    name: ident.name.clone(),
                    span: token.span,
                })
            } else {
                Err(ParseError::UnexpectedToken { expected: "identifier", got: token })
            }
        })
    }

    fn could_parse(stream: &mut ParseStream) -> bool {
        matches!(&stream.current().content, TokenType::Ident(ident) if !ident.escaped && !is_keyword(&ident.name))
    }
}

fn is_keyword(ident: &str) -> bool {
    match ident {
        | "let"
        | "fn"
        | "if"
        | "else"
        | "while"
        | "for"
        | "in"
        | "return"
        | "break"
        | "continue"
        | "mut"
        | "true"
        | "false" => true,
        _ => false,
    }
}

#[cfg(test)]
mod test {
    use crate::{ tokens::stream::TokenStream, ast::Parse };

    use super::{ super::stream::ParseStream, Ident };

    #[test]
    fn test_ident() {
        let tokens = TokenStream::from_string("name").unwrap();
        let mut stream = ParseStream::new(tokens);

        let ident = Ident::parse(&mut stream).unwrap();
        assert_eq!(ident.name, "name");
    }

    #[test]
    fn test_keyword() {
        let tokens = TokenStream::from_string("let").unwrap();
        let mut stream = ParseStream::new(tokens);

        let ident = Ident::parse(&mut stream);
        assert!(ident.is_err());
    }

    #[test]
    fn test_escaped_keyword() {
        let tokens = TokenStream::from_string("@let").unwrap();
        let mut stream = ParseStream::new(tokens);

        let ident = Ident::parse(&mut stream).unwrap();
        assert_eq!(ident.name, "let");
    }
}
