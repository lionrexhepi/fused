use crate::{ Span, tokens::{ TokenType, ident::TokenIdent } };

use super::{
    number::LitNumber,
    string::LitString,
    Spanned,
    Parse,
    ParseError,
    functions::ExprFunction,
    conditionals::ExprIf,
    loops::{ ExprWhile, ExprFor, ExprLoop },
    stream::ParseStream,
    ParseResult,
    simple::ExprSimple,
    declarations::ExprDecl,
};

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub enum Expr {
    Decl(ExprDecl),
    Simple(ExprSimple),
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
            Self::Decl(decl) => decl.span(),
            Self::Simple(simple) => simple.span(),
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
    fn parse(stream: &mut ParseStream) -> ParseResult<Self> {
        stream.skip_newlines();
        let result = if ExprFunction::could_parse(stream) {
            Self::Function(stream.parse()?)
        } else if ExprIf::could_parse(stream) {
            Self::If(stream.parse()?)
        } else if ExprWhile::could_parse(stream) {
            Self::While(stream.parse()?)
        } else if ExprFor::could_parse(stream) {
            Self::For(stream.parse()?)
        } else if ExprLoop::could_parse(stream) {
            Self::Loop(stream.parse()?)
        } else if ExprDecl::could_parse(stream) {
            Self::Decl(stream.parse()?)
        } else {
            Self::Simple(stream.parse()?)
        };

        Ok(result)
    }

    fn could_parse(stream: &mut ParseStream) -> bool {
        stream.current().content != TokenType::EOF
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
    fn parse(stream: &mut ParseStream) -> ParseResult<Self> where Self: Sized {
        if let Ok(bool) = stream.parse::<LitBool>() {
            Ok(Self::Bool(bool))
        } else if let Ok(number) = stream.parse::<LitNumber>() {
            Ok(Self::Number(number))
        } else if let Ok(string) = stream.parse::<LitString>() {
            Ok(Self::String(string))
        } else {
            Err(ParseError::UnexpectedToken {
                expected: "literal",
                got: stream.current().clone(),
            })
        }
    }

    fn could_parse(stream: &mut ParseStream) -> bool {
        LitBool::could_parse(stream) ||
            LitNumber::could_parse(stream) ||
            LitString::could_parse(stream)
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
    fn parse(stream: &mut ParseStream) -> ParseResult<Self> where Self: Sized {
        stream.parse_with(|cursor: &mut super::stream::TokenCursor| {
            let token = cursor.current().clone();
            if let TokenType::Ident(ident) = &token.content {
                if !ident.escaped && (ident.name == "true" || ident.name == "false") {
                    cursor.advance();
                    Ok(Self {
                        value: ident.name == "true",
                        span: token.span,
                    })
                } else {
                    Err(ParseError::UnexpectedToken { expected: "bool", got: token })
                }
            } else {
                Err(ParseError::UnexpectedToken { expected: "bool", got: token })
            }
        })
    }

    fn could_parse(stream: &mut ParseStream) -> bool {
        matches!(
            &stream.current().content,
            TokenType::Ident(TokenIdent { escaped: false, name }) if name == "true" || name == "false"
        )
    }
}

#[cfg(test)]
mod test {
    use crate::{ tokens::stream::TokenStream, ast::Parse };

    use super::{ LitBool, super::stream::ParseStream };

    #[test]
    fn test_bools() {
        let stream = TokenStream::from_string("true false").unwrap();
        let mut stream = ParseStream::new(stream);
        let token = LitBool::parse(&mut stream).unwrap();
        assert_eq!(token.value, true);
        assert_eq!(token.span, (0..4).into());
        let token = LitBool::parse(&mut stream).unwrap();
        assert_eq!(token.value, false);
        assert_eq!(token.span, (5..10).into());
    }
}
