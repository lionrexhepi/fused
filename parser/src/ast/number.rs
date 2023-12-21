use crate::{
    Span,
    tokens::{ Token, TokenType, literal::{ TokenLiteral, LiteralNumber, NumberType } },
};

use super::{ Parse, Spanned, ParseResult, stream::{ ParseStream, TokenCursor }, ParseError };

#[derive(Debug, Clone)]
enum Number {
    Int(i64),
    UInt(u64),
    Float(f64),
}

impl PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Int(l0), Self::Int(r0)) => l0 == r0,
            (Self::UInt(l0), Self::UInt(r0)) => l0 == r0,
            (Self::Float(l0), Self::Float(r0)) => l0 == r0,
            _ => false,
        }
    }
}

impl Eq for Number {}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct LitNumber {
    number: Number,
    span: Span,
}

impl Spanned for LitNumber {
    fn span(&self) -> Span {
        self.span
    }
}

impl Parse for LitNumber {
    fn parse(token: &mut ParseStream) -> ParseResult<Self> where Self: Sized {
        token.parse_with(|cursor: &mut TokenCursor| {
            let Token { content, span } = cursor.current().clone();
            if
                let TokenType::Literal(TokenLiteral::Number(LiteralNumber { r#type, digits })) =
                    content
            {
                cursor.advance();
                let value = match r#type {
                    NumberType::Decimal => {
                        let signed = if matches!(digits.chars().next(), Some('-') | Some('+')) {
                            true
                        } else {
                            false
                        };

                        let float = digits.chars().any(|c| (c == '.' || c == 'e' || c == 'E'));
                        if float {
                            let number = digits.parse::<f64>().map_err(|err| {
                                let err = err.to_string();
                                let err = err.split(':').last().unwrap().trim();
                                ParseError::BadLiteral(err.to_string())
                            })?;
                            Number::Float(number)
                        } else if signed {
                            //If there is a '+' sign in front of an integer we assume that that means that the user wants the value to also be able to be negative (when talking about type inference)
                            let number = digits.parse::<i64>().map_err(|err| {
                                let err = err.to_string();
                                let err = err.split(':').last().unwrap().trim();
                                ParseError::BadLiteral(err.to_string())
                            })?;
                            Number::Int(number)
                        } else {
                            let number = digits.parse::<u64>().map_err(|err| {
                                let err = err.to_string();
                                let err = err.split(':').last().unwrap().trim();
                                ParseError::BadLiteral(err.to_string())
                            })?;
                            Number::UInt(number)
                        }
                    }
                    NumberType::Hexadecimal => {
                        let number = u64::from_str_radix(&digits, 16).unwrap();
                        Number::UInt(number)
                    }
                    NumberType::Binary => {
                        let number = u64::from_str_radix(&digits, 2).unwrap();
                        Number::UInt(number)
                    }
                };

                Ok(Self { number: value, span })
            } else {
                Err(ParseError::UnexpectedToken("number", cursor.current().clone()))
            }
        })
    }
}
