use crate::{ Span, tokens::TokenType };
use crate::tokens::group::Delim;

use super::expr::Expr;
use super::{ Spanned, ParseResult, Parse, ParseError };
use super::stream::{ TokenCursor, ParseStream };

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ExprGrouped<C: Parse + Sized = Expr> {
    Parenthesized(Parenthesized<C>),
    Bracketed(Bracketed<C>),
    Braced(Braced<C>),
}

impl Spanned for ExprGrouped {
    fn span(&self) -> Span {
        match self {
            Self::Parenthesized(paren) => paren.span(),
            Self::Bracketed(bracket) => bracket.span(),
            Self::Braced(brace) => brace.span(),
        }
    }
}

impl Parse for ExprGrouped {
    fn parse(stream: &mut ParseStream) -> ParseResult<Self> where Self: Sized {
        if let Ok(paren) = stream.parse::<Parenthesized>() {
            Ok(Self::Parenthesized(paren))
        } else if let Ok(bracket) = stream.parse::<Bracketed>() {
            Ok(Self::Bracketed(bracket))
        } else {
            let brace = stream.parse::<Braced>();
            Ok(Self::Braced(brace?))
        }
    }
}

macro_rules! group {
    ($name:ident, $delim:ident) => {
        #[derive(Debug, PartialEq, Eq, Clone)]
        pub struct $name<C: Parse + Sized = Expr>(pub Box<C>);

        impl<C: Parse + Sized> Spanned  for $name<C> {
            fn span(&self) -> Span {
                self.0.span()
            }
        }

        impl<C: Parse + Sized> Parse for $name<C> {
            fn parse(stream: &mut ParseStream) -> ParseResult<Self> {
                let mut inner = stream.parse_with(|cursor: &mut TokenCursor| {
                    match cursor.current().content.clone() {
                        TokenType::Group(group) => match group.delim {
                            Delim::$delim => {

                                cursor.advance();
                                Ok(ParseStream::new(*group.tokens))
                            }
                            _ => return Err(ParseError::UnexpectedToken {
                                expected: stringify!($name),
                                got: cursor.current().clone(),
                            })
                        },
                        _ => return Err(ParseError::UnexpectedToken {
                            expected: stringify!($name),
                            got: cursor.current().clone(),
                        })
                    }
                })?;

                Ok(Self(Box::new(inner.parse()?)))
            }
        }
    };
}

group!(Parenthesized, Paren);
group!(Bracketed, Bracket);
group!(Braced, Brace);

#[cfg(test)]
mod test {
    use crate::{
        ast::{ expr::{ Expr, ExprLit }, grouped::ExprGrouped, stream::ParseStream },
        tokens::stream::TokenStream,
    };

    use super::Parenthesized;

    #[test]
    fn test_paren() {
        let tokens = TokenStream::from_string("(1)").unwrap();
        let mut stream = ParseStream::new(tokens);
        let parens = stream.parse::<Parenthesized>().unwrap();
        assert!(matches!(*parens.0, super::Expr::Literal(ExprLit::Number(_))));
    }

    #[test]
    fn test_bracket() {
        let tokens = TokenStream::from_string("[1]").unwrap();
        let mut stream = ParseStream::new(tokens);
        let brackets = stream.parse::<super::Bracketed>().unwrap();
        assert!(matches!(*brackets.0, super::Expr::Literal(ExprLit::Number(_))));
    }

    #[test]
    fn test_brace() {
        let tokens = TokenStream::from_string("{1}").unwrap();
        let mut stream = ParseStream::new(tokens);
        let braces = stream.parse::<super::Braced>().unwrap();
        assert!(matches!(*braces.0, super::Expr::Literal(ExprLit::Number(_))));
    }

    #[test]
    fn test_nested() {
        let tokens = TokenStream::from_string("(1 + [2])").unwrap();
        let mut stream = ParseStream::new(tokens);

        let parens = stream.parse::<Parenthesized<Expr>>().unwrap();
        print!("{:#?}", parens.0);
        assert!(matches!(*parens.0, Expr::Binary(_)));
        if let Expr::Binary(binary) = *parens.0 {
            assert!(matches!(*binary.left, Expr::Literal(ExprLit::Number(_))));
            assert!(matches!(*binary.right, Expr::Grouped(ExprGrouped::Bracketed(_))));
        } else {
            panic!("No binary expression found")
        }
    }
}
