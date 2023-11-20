use crate::tokens::{ Token, Span, TokenType };

use super::{ Spanned, Parse, ParseResult, ParseError, stream::ParseStream };

pub trait Keyword: Spanned {
    fn name() -> &'static str;

    fn new(span: Span) -> Self;

    fn from_token(token: &Token) -> Option<Self> where Self: Sized {
        let Token { span, content } = token;
        if let TokenType::Ident(ident) = content {
            if !ident.escaped && ident.name == Self::name() {
                return Some(Self::new(*span));
            }
        }

        None
    }
}

impl<K: Keyword> Parse for K {
    fn parse(token: &mut ParseStream) -> ParseResult<Self> {
        token.keyword()
    }
}

macro_rules! define_keyword {
    ($kw:ident, $name:expr) => {
        pub struct $kw($crate::tokens::Span);

        impl $crate::ast::Spanned for $kw {
            fn span(&self) -> $crate::tokens::Span {
                self.0
            }
        }

        impl Keyword for  $kw {
            fn new(span: $crate::tokens::Span) -> Self {
                Self(span)
            }

            fn name() -> &'static str {
                $name
            }

    
        }
    };
}

define_keyword!(If, "if");

define_keyword!(Else, "else");
define_keyword!(ElseIf, "elif");
define_keyword!(While, "while");
define_keyword!(For, "for");
define_keyword!(In, "in");
define_keyword!(Break, "break");
define_keyword!(Continue, "continue");
define_keyword!(Return, "return");
define_keyword!(Fn, "fn");
define_keyword!(Let, "let");
define_keyword!(Class, "class");
define_keyword!(Enum, "enum");
define_keyword!(Struct, "struct");
define_keyword!(Impl, "impl");
define_keyword!(This, "this");
define_keyword!(Super, "super");

define_keyword!(True, "true");
define_keyword!(False, "false");
