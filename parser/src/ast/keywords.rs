use crate::tokens::{ Token, Span, TokenType };

use super::{ Spanned, Parse, ParseResult, ParseError, stream::{ ParseStream, Cursor } };

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
        token.parse_with(|cursor: &mut Cursor| {
            let token = cursor.current();
            if let Some(keyword) = Self::from_token(token) {
                cursor.advance();
                Ok(keyword)
            } else {
                Err(ParseError::UnexpectedToken(Self::name(), token.clone()))
            }
        })
    }
}

macro_rules! define_keyword {
    ($kw:ident, $name:expr) => {
        #[derive(Debug, PartialEq, Eq, Clone)]
        pub struct $kw(pub(crate) $crate::tokens::Span);

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
define_keyword!(Mut, "mut");
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
define_keyword!(Loop, "loop");
define_keyword!(Mod, "mod");
define_keyword!(Use, "use");

#[cfg(test)]
mod test {
    use crate::{ ast::{ keywords::*, stream::ParseStream }, tokens::stream::TokenStream };

    #[test]
    fn test_keywords() {
        let stream = TokenStream::from_string(
            "if else elif while for in break continue return fn let class enum struct impl this super".to_string()
        ).unwrap();
        let mut stream = ParseStream::new(stream);

        assert_eq!(stream.parse::<If>().unwrap(), If((0..2).into()));
        assert_eq!(stream.parse::<Else>().unwrap(), Else((3..7).into()));
        assert_eq!(stream.parse::<ElseIf>().unwrap(), ElseIf((8..12).into()));
        assert_eq!(stream.parse::<While>().unwrap(), While((13..18).into()));
        assert_eq!(stream.parse::<For>().unwrap(), For((19..22).into()));
        assert_eq!(stream.parse::<In>().unwrap(), In((23..25).into()));
        assert_eq!(stream.parse::<Break>().unwrap(), Break((26..31).into()));
        assert_eq!(stream.parse::<Continue>().unwrap(), Continue((32..40).into()));
        assert_eq!(stream.parse::<Return>().unwrap(), Return((41..47).into()));
        assert_eq!(stream.parse::<Fn>().unwrap(), Fn((48..50).into()));
        assert_eq!(stream.parse::<Let>().unwrap(), Let((51..54).into()));
        assert_eq!(stream.parse::<Class>().unwrap(), Class((55..60).into()));
        assert_eq!(stream.parse::<Enum>().unwrap(), Enum((61..65).into()));
        assert_eq!(stream.parse::<Struct>().unwrap(), Struct((66..72).into()));
        assert_eq!(stream.parse::<Impl>().unwrap(), Impl((73..77).into()));
        assert_eq!(stream.parse::<This>().unwrap(), This((78..82).into()));
        assert_eq!(stream.parse::<Super>().unwrap(), Super((83..88).into()));
    }

    #[test]
    fn test_regular_ident() {
        let stream = TokenStream::from_string("test".to_string()).unwrap();
        let mut stream = ParseStream::new(stream);

        assert!(matches!(stream.parse::<If>(), Err(_)));
    }
}
