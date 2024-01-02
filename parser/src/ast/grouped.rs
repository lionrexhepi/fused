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

impl<C: Parse + Sized> Spanned for ExprGrouped<C> {
    fn span(&self) -> Span {
        match self {
            Self::Parenthesized(paren) => paren.span(),
            Self::Bracketed(bracket) => bracket.span(),
            Self::Braced(brace) => brace.span(),
        }
    }
}

impl<C: Parse + Sized> Parse for ExprGrouped<C> {
    fn parse(stream: &mut ParseStream) -> ParseResult<Self> where Self: Sized {
        if let Ok(paren) = stream.parse::<Parenthesized<C>>() {
            Ok(Self::Parenthesized(paren))
        } else if let Ok(bracket) = stream.parse::<Bracketed<C>>() {
            Ok(Self::Bracketed(bracket))
        } else {
            let brace = stream.parse::<Braced<C>>();
            Ok(Self::Braced(brace?))
        }
    }

    fn could_parse(stream: &mut ParseStream) -> bool {
        Parenthesized::<C>::could_parse(stream) ||
            Bracketed::<C>::could_parse(stream) ||
            Braced::<C>::could_parse(stream)
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

            fn could_parse(stream: &mut ParseStream) -> bool {
                matches!(&stream.current().content, TokenType::Group(group) if group.delim == Delim::$delim)
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
        ast::{ expr::{ Expr, ExprLit }, stream::ParseStream, simple::{ ExprSimple, BinaryType } },
        tokens::stream::TokenStream,
    };

    use super::Parenthesized;

    #[test]
    fn test_paren() {
        let tokens = TokenStream::from_string("(1)").unwrap();
        let mut stream = ParseStream::new(tokens);
        let parens = stream.parse::<Parenthesized<ExprSimple>>().unwrap();
        assert!(matches!(*parens.0, ExprSimple::Literal(ExprLit::Number(_))));
    }

    #[test]
    fn test_bracket() {
        let tokens = TokenStream::from_string("[1]").unwrap();
        let mut stream = ParseStream::new(tokens);
        let brackets = stream.parse::<super::Bracketed>().unwrap();
        assert!(matches!(*brackets.0, Expr::Simple(ExprSimple::Literal(ExprLit::Number(_)))));
    }

    #[test]
    fn test_brace() {
        let tokens = TokenStream::from_string("{1}").unwrap();
        let mut stream = ParseStream::new(tokens);
        let braces = stream.parse::<super::Braced>().unwrap();
        assert!(matches!(*braces.0, Expr::Simple(ExprSimple::Literal(ExprLit::Number(_)))));
    }

    #[test]
    fn test_nested() {
        let tokens = TokenStream::from_string("(1 + (2))").unwrap();
        let mut stream = ParseStream::new(tokens);

        let parens = stream.parse::<Parenthesized<Expr>>().unwrap();
        assert!(matches!(*parens.0, Expr::Simple(ExprSimple::Binary(_, BinaryType::Add, _))));
        if let Expr::Simple(ExprSimple::Binary(left, _, right)) = *parens.0 {
            assert!(matches!(*left, ExprSimple::Literal(ExprLit::Number(_))));
            assert!(matches!(*right, ExprSimple::Grouped(Parenthesized(_))));
        } else {
            panic!("No binary expression found")
        }
    }
}
