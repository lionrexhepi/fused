use std::any::TypeId;

use crate::tokens::{ Span, TokenType };

use super::{
    number::LitNumber,
    ident::ExprIdent,
    string::LitString,
    Spanned,
    Parse,
    ParseError,
    block::ExprBlock,
    operations::{ ExprUnary, ExprBinary },
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Expr {
    Literal(ExprLit),
    Ident(ExprIdent),
    Block(ExprBlock),
    Unary(ExprUnary),
    Binary(ExprBinary),
}

impl Spanned for Expr {
    fn span(&self) -> Span {
        match self {
            Self::Literal(lit) => lit.span(),
            Self::Ident(ident) => ident.span(),
            Self::Block(block) => block.span(),
            Self::Unary(unary) => unary.span(),
            Self::Binary(binary) => binary.span(),
        }
    }
}

impl Parse for Expr {
    fn parse(stream: &mut super::stream::ParseStream) -> super::ParseResult<Self> where Self: Sized {
        stream.skip_newlines();
        let result = if let Ok(op) = stream.parse::<ExprUnary>() {
            Ok(Self::Unary(op))
        } else if let Ok(op) = stream.parse::<ExprBinary>() {
            Ok(Self::Binary(op))
        } else if let Ok(lit) = stream.parse::<ExprLit>() {
            Ok(Self::Literal(lit))
        } else if let Ok(ident) = stream.parse::<ExprIdent>() {
            Ok(Self::Ident(ident))
        } else {
            let block = stream.parse::<ExprBlock>();
            Ok(Self::Block(block?))
        };

        result
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ExprLit {
    String(LitString),
    Number(LitNumber),
    Bool(LitBool),
}

impl Spanned for ExprLit {
    fn span(&self) -> Span {
        match self {
            Self::String(string) => string.span(),
            Self::Number(number) => number.span(),
            Self::Bool(bool) => bool.span(),
        }
    }
}

impl Parse for ExprLit {
    fn parse(stream: &mut super::stream::ParseStream) -> super::ParseResult<Self> where Self: Sized {
        if let Ok(bool) = stream.parse::<LitBool>() {
            Ok(Self::Bool(bool))
        } else if let Ok(number) = stream.parse::<LitNumber>() {
            Ok(Self::Number(number))
        } else if let Ok(string) = stream.parse::<LitString>() {
            Ok(Self::String(string))
        } else {
            Err(ParseError::UnexpectedToken("literal", stream.current().clone()))
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct LitBool {
    pub value: bool,
    pub span: Span,
}

impl Spanned for LitBool {
    fn span(&self) -> Span {
        self.span
    }
}

impl Parse for LitBool {
    fn parse(stream: &mut super::stream::ParseStream) -> super::ParseResult<Self> where Self: Sized {
        stream.parse_with(|cursor: &mut super::stream::Cursor| {
            let token = cursor.current().clone();
            if let TokenType::Ident(ident) = &token.content {
                if !ident.escaped && (ident.name == "true" || ident.name == "false") {
                    cursor.advance();
                    Ok(Self {
                        value: ident.name == "true",
                        span: token.span,
                    })
                } else {
                    Err(super::ParseError::UnexpectedToken("bool", token))
                }
            } else {
                Err(super::ParseError::UnexpectedToken("bool", token))
            }
        })
    }
}

#[cfg(test)]
mod test {
    use crate::{ tokens::stream::TokenStream, ast::Parse };

    #[test]
    fn test_bools() {
        let stream = TokenStream::from_string("true false".to_string()).unwrap();
        let mut stream = super::super::stream::ParseStream::new(stream);
        let token = super::LitBool::parse(&mut stream).unwrap();
        assert_eq!(token.value, true);
        assert_eq!(token.span, (0..4).into());
        let token = super::LitBool::parse(&mut stream).unwrap();
        assert_eq!(token.value, false);
        assert_eq!(token.span, (5..10).into());
    }
}
