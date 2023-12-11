use crate::tokens::{ Span, TokenType, punct::TokenPunct };

use super::{ expr::Expr, Spanned, Parse, stream::{ ParseStream, Cursor }, ParseError, ParseResult };

pub struct Unary {
    pub arg: Box<Expr>,
    span: Span,
    pub ty: UnaryType,
}


pub enum UnaryType {
    Not,
    Deref,
    Incr,
    Decr,
}

impl Spanned for Unary {
    fn span(&self) -> Span {
        self.span
    }
}

impl Parse for Unary {
    fn parse(stream: &mut ParseStream) -> ParseResult<Self> where Self: Sized {
        let ty = stream.parse_with(|cursor: &mut Cursor| {
            let token = cursor.current().clone();
            match &token.content {
                TokenType::Punct(punct) =>
                    match punct {
                        TokenPunct::Exclamation => {
                            cursor.advance();
                            Ok(UnaryType::Not)
                        }
                        TokenPunct::Star => {
                            cursor.advance();
                            Ok(UnaryType::Deref)
                        }

                        _ => Err(ParseError::UnexpectedToken("unary operator", token)),
                    }
                _ => Err(ParseError::UnexpectedToken("unary operator", token)),
            }
        })?;

        let arg = stream.parse::<Expr>()?;
        let span = arg.span();
        Ok(Self {
            arg: Box::new(arg),
            span,
            ty,
        })
    }
}

pub struct Binary {
    pub left: Box<Expr>,
    pub right: Box<Expr>,
    span: Span,
    pub ty: BinaryType,
}
pub enum BinaryType {
    Assign, //Do NOT shorten.
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    And,
    Or,
    Eq,
    Neq,
    Lt,
    Gt,
    Leq,
    Geq,
    
}