use crate::ast::{ Parse, ParseResult, Spanned, ParseStream, stream::TokenCursor, ParseError };
use crate::tokens::{ TokenType, punct::TokenPunct };
pub trait Punct: Parse {}

macro_rules! define_punct {
    ($name:ident) => {
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct $name($crate::tokens::Span);

        impl Punct for $name {}

        impl Spanned for $name {
            fn span(&self) -> $crate::tokens::Span {
                self.0
            }
        }

        impl Parse for $name {
            fn parse(token: &mut ParseStream) -> ParseResult<Self> {
                token.parse_with(|cursor: &mut TokenCursor| {
                    let token = cursor.current().clone();
                    if let TokenType::Punct(punct) = &token.content {
                        if punct == &TokenPunct::$name {
                            cursor.advance();
                            Ok(Self(token.span))
                        } else {
                            Err(ParseError::UnexpectedToken(stringify!($name), token.clone()))
                        }
                    } else {
                        Err(ParseError::UnexpectedToken(stringify!($name), token.clone()))
                    }
                })
            }
        }
    };

    ($($names:ident),*) => {
        $(
            define_punct!($names);
        )*
    };
}

define_punct!(
    Plus, //+
    Minus, //-
    Star, //*
    Slash, // /
    Percent, // %
    Caret, // ^
    Ampersand, // &
    Pipe, // |
    DoublePipe, // ||
    PlusEq, // +=
    MinusEq, // -=
    StarEq, // *=
    SlashEq, // /=
    PercentEq, // %=
    CaretEq, // ^=
    AmpersandEq, // &=
    PipeEq, // |=
    Tilde, // ~
    Question, // ?
    Exclamation, // !
    Dot, // .
    Comma, // ,
    Colon, // :
    SemiColon, // ;
    Eq, // =
    DoubleEq, // ==
    NotEq, // !=
    Lt, // <
    DoubleLt, // <<
    DoubleLtEq, // <<=
    LtEq, // <=
    Gt, // >
    DoubleGt, // >>
    DoubleGtEq, // >>=
    GtEq, // >=
    Arrow, // ->
    FatArrow, // =>
    Dollar, // $
    Backslash, // \
    DoubleAmpersand // &&
);
