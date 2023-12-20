use std::any::TypeId;

use crate::tokens::{ Span, TokenType, group };

use super::{
    number::LitNumber,
    ident::Ident,
    string::LitString,
    Spanned,
    Parse,
    ParseError,
    block::Block,
    operations::{ ExprUnary, ExprBinary },
    grouped::ExprGrouped,
    functions::ExprFunction,
    conditionals::ExprIf,
    loops::{ ExprWhile, ExprFor, ExprLoop },
    path::ExprPath,
};

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub enum Expr {
    Literal(ExprLit),
    Path(ExprPath),
    Unary(ExprUnary),
    Binary(ExprBinary),
    Grouped(ExprGrouped),
    Function(ExprFunction),
    If(ExprIf),
    While(ExprWhile),
    For(ExprFor),
    Loop(ExprLoop),
    #[default]
    Empty,
}

impl Spanned for Expr {
    fn span(&self) -> Span {
        match self {
            Self::Literal(lit) => lit.span(),
            Self::Path(path) => path.span(),
            Self::Unary(unary) => unary.span(),
            Self::Binary(binary) => binary.span(),
            Expr::Grouped(grouped) => grouped.span(),
            Expr::Function(function) => function.span(),
            Expr::If(r#if) => r#if.span(),
            Expr::While(r#while) => r#while.span(),
            Expr::For(r#for) => r#for.span(),
            Expr::Loop(r#loop) => r#loop.span(),
            Expr::Empty => Span::default(),
        }
    }
}

impl Parse for Expr {
    fn parse(stream: &mut super::stream::ParseStream) -> super::ParseResult<Self> {
        stream.skip_newlines();
        let result = if let Ok(op) = stream.parse::<ExprUnary>() {
            Ok(Self::Unary(op))
        } else if let Ok(group) = stream.parse() {
            Ok(Self::Grouped(group))
        } else if let Ok(op) = stream.parse() {
            Ok(Self::Binary(op))
        } else if let Ok(lit) = stream.parse() {
            Ok(Self::Literal(lit))
        } else if let Ok(path) = stream.parse() {
            Ok(Self::Path(path))
        } else if let Ok(function) = stream.parse() {
            Ok(Self::Function(function))
        } else if let Ok(r#if) = stream.parse() {
            Ok(Self::If(r#if))
        } else if let Ok(r#while) = stream.parse() {
            Ok(Self::While(r#while))
        } else if let Ok(r#for) = stream.parse() {
            Ok(Self::For(r#for))
        } else if let Ok(r#loop) = stream.parse() {
            Ok(Self::Loop(r#loop))
        } else {
            Err(ParseError::UnexpectedToken("expression", stream.current().clone()))
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
