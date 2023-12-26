use std::{ collections::HashMap, fmt::Binary };

use crate::{ Span, tokens::{ TokenType, punct::TokenPunct, group::{ TokenGroup, Delim } } };

use super::{
    expr::{ Expr, ExprLit },
    Spanned,
    Parse,
    stream::{ ParseStream, TokenCursor, self },
    ParseError,
    ParseResult,
    punct::{
        Eq,
        PlusEq,
        MinusEq,
        DoublePipe,
        StarEq,
        SlashEq,
        PercentEq,
        AmpersandEq,
        PipeEq,
        DoubleAmpersandEq,
        DoublePipeEq,
        CaretEq,
        DoubleLtEq,
        DoubleGtEq,
        Plus,
        Minus,
        Star,
        Slash,
        Percent,
        DoubleAmpersand,
        Ampersand,
        DoubleEq,
        Caret,
        Pipe,
        NotEq,
        Lt,
        Gt,
        LtEq,
        GtEq,
        DoubleLt,
        DoubleGt,
        Comma,
        Exclamation,
    },
    path::ExprPath,
    grouped::{ ExprGrouped, Parenthesized },
    separated::Separated,
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ExprSimple {
    Binary(Box<Self>, BinaryType, Box<Self>),
    Unary(Box<Self>, UnaryType),
    Call(Box<Self>, Separated<Expr, Comma>),
    Grouped(Parenthesized<Expr>),
    Path(ExprPath),
    Literal(ExprLit),
}

impl Spanned for ExprSimple {
    fn span(&self) -> Span {
        match self {
            Self::Binary(left, _, right) => left.span().join(right.span()),
            Self::Unary(expr, _) => expr.span(),
            Self::Grouped(grouped) => grouped.span(),
            Self::Path(path) => path.span(),
            Self::Literal(lit) => lit.span(),
            Self::Call(callee, args) => callee.span().join(args.span()),
        }
    }
}

impl Parse for ExprSimple {
    fn parse(stream: &mut ParseStream) -> ParseResult<Self> where Self: Sized {
        Self::parse_binary(stream, BinaryType::Assign)
    }

    fn could_parse(stream: &mut ParseStream) -> bool {
        Parenthesized::<Expr>::could_parse(stream) ||
            ExprPath::could_parse(stream) ||
            ExprLit::could_parse(stream) ||
            Exclamation::could_parse(stream) ||
            Minus::could_parse(stream)
    }
}

impl ExprSimple {
    fn parse_binary(stream: &mut ParseStream, operator: BinaryType) -> ParseResult<Self> {
        let left = if let Some(next) = operator.next_by_precedence() {
            Self::parse_binary(stream, next)?
        } else {
            Self::parse_unary_prefix(stream)?
        };
        if operator.matches(stream) {
            let right = Self::parse_binary(stream, operator)?; //Try the first operator in case of chaining
            Ok(Self::Binary(Box::new(left), operator, Box::new(right)))
        } else {
            Ok(left)
        }
    }

    fn parse_unary_prefix(stream: &mut ParseStream) -> ParseResult<Self> {
        let r#ty = if stream.parse::<Exclamation>().is_ok() {
            UnaryType::Not
        } else if stream.parse::<Minus>().is_ok() {
            UnaryType::Neg
        } else {
            return Self::parse_call(stream);
        };

        let arg = Self::parse_unary_prefix(stream)?;

        Ok(Self::Unary(Box::new(arg), r#ty))
    }

    fn parse_call(stream: &mut ParseStream) -> ParseResult<Self> {
        let mut callee = Self::parse_grouped(stream)?;

        while Parenthesized::<Separated<Expr>>::could_parse(stream) {
            let args = stream.parse::<Parenthesized<_>>()?;
            callee = Self::Call(Box::new(callee), *args.0);
        }

        Ok(callee)
    }

    fn parse_grouped(stream: &mut ParseStream) -> ParseResult<Self> {
        if Parenthesized::<Expr>::could_parse(stream) {
            Ok(Self::Grouped(stream.parse()?))
        } else {
            Self::parse_path(stream)
        }
    }

    fn parse_path(stream: &mut ParseStream) -> ParseResult<Self> {
        if ExprPath::could_parse(stream) {
            Ok(Self::Path(stream.parse()?))
        } else {
            Self::parse_literal(stream)
        }
    }

    fn parse_literal(stream: &mut ParseStream) -> ParseResult<Self> {
        Ok(Self::Literal(ExprLit::parse(stream)?))
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum UnaryType {
    Not,
    Neg,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum BinaryType {
    Assign,
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
    ModAssign,
    AndAssign,
    OrAssign,
    BitAndAssign,
    BitOrAssign,
    BitXorAssign,
    LeftShiftAssign,
    RightShiftAssign,
    Or,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    BitAnd,
    And,
    BitOr,
    BitXor,
    Eq,
    Neq,
    Lt,
    Gt,
    Leq,
    Geq,
    LeftShift,
    RightShift,
}

impl BinaryType {
    fn next_by_precedence(&self) -> Option<Self> {
        Some(match self {
            Self::Assign => Self::AddAssign,
            Self::AddAssign => Self::SubAssign,
            Self::SubAssign => Self::MulAssign,
            Self::MulAssign => Self::DivAssign,
            Self::DivAssign => Self::ModAssign,
            Self::ModAssign => Self::AndAssign,
            Self::AndAssign => Self::OrAssign,
            Self::OrAssign => Self::BitAndAssign,
            Self::BitAndAssign => Self::BitOrAssign,
            Self::BitOrAssign => Self::BitXorAssign,
            Self::BitXorAssign => Self::LeftShiftAssign,
            Self::LeftShiftAssign => Self::RightShiftAssign,
            Self::RightShiftAssign => Self::Or,
            Self::Or => Self::Add,
            Self::Add => Self::Sub,
            Self::Sub => Self::Mul,
            Self::Mul => Self::Div,
            Self::Div => Self::Mod,
            Self::Mod => Self::And,
            Self::And => Self::BitAnd,
            Self::BitAnd => Self::BitOr,
            Self::BitOr => Self::BitXor,
            Self::BitXor => Self::Eq,
            Self::Eq => Self::Neq,
            Self::Neq => Self::Lt,
            Self::Lt => Self::Gt,
            Self::Gt => Self::Leq,
            Self::Leq => Self::Geq,
            Self::Geq => Self::LeftShift,
            Self::LeftShift => Self::RightShift,
            Self::RightShift => {
                return None;
            }
        })
    }

    fn matches(&self, stream: &mut ParseStream) -> bool {
        match self {
            Self::Assign => stream.parse::<Eq>().is_ok(),
            Self::AddAssign => stream.parse::<PlusEq>().is_ok(),
            Self::SubAssign => stream.parse::<MinusEq>().is_ok(),
            Self::MulAssign => stream.parse::<StarEq>().is_ok(),
            Self::DivAssign => stream.parse::<SlashEq>().is_ok(),
            Self::ModAssign => stream.parse::<PercentEq>().is_ok(),
            Self::AndAssign => stream.parse::<DoubleAmpersandEq>().is_ok(),
            Self::OrAssign => stream.parse::<DoublePipeEq>().is_ok(),
            Self::BitAndAssign => stream.parse::<AmpersandEq>().is_ok(),
            Self::BitOrAssign => stream.parse::<PipeEq>().is_ok(),
            Self::BitXorAssign => stream.parse::<CaretEq>().is_ok(),
            Self::LeftShiftAssign => stream.parse::<DoubleLtEq>().is_ok(),
            Self::RightShiftAssign => stream.parse::<DoubleGtEq>().is_ok(),
            Self::Or => stream.parse::<DoublePipe>().is_ok(),
            Self::Add => stream.parse::<Plus>().is_ok(),
            Self::Sub => stream.parse::<Minus>().is_ok(),
            Self::Mul => stream.parse::<Star>().is_ok(),
            Self::Div => stream.parse::<Slash>().is_ok(),
            Self::Mod => stream.parse::<Percent>().is_ok(),
            Self::And => stream.parse::<DoubleAmpersand>().is_ok(),
            Self::BitAnd => stream.parse::<Ampersand>().is_ok(),
            Self::BitOr => stream.parse::<Pipe>().is_ok(),
            Self::BitXor => stream.parse::<Caret>().is_ok(),
            Self::Eq => stream.parse::<DoubleEq>().is_ok(),
            Self::Neq => stream.parse::<NotEq>().is_ok(),
            Self::Lt => stream.parse::<Lt>().is_ok(),
            Self::Gt => stream.parse::<Gt>().is_ok(),
            Self::Leq => stream.parse::<LtEq>().is_ok(),
            Self::Geq => stream.parse::<GtEq>().is_ok(),
            Self::LeftShift => stream.parse::<DoubleLt>().is_ok(),
            Self::RightShift => stream.parse::<DoubleGt>().is_ok(),
        }
    }

    fn left_associative(&self) -> bool {
        match self {
            | Self::Assign
            | Self::AddAssign
            | Self::SubAssign
            | Self::MulAssign
            | Self::DivAssign
            | Self::ModAssign
            | Self::AndAssign
            | Self::OrAssign
            | Self::BitAndAssign
            | Self::BitOrAssign
            | Self::BitXorAssign
            | Self::LeftShiftAssign
            | Self::RightShiftAssign => false,
            _ => true,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        ast::{
            simple::{ ExprSimple, BinaryType },
            stream::ParseStream,
            Parse,
            expr::{ ExprLit, Expr },
            grouped::Parenthesized,
        },
        tokens::stream::TokenStream,
    };

    #[test]
    fn test_lit_num() {
        let mut stream = ParseStream::new(TokenStream::from_string("123").unwrap());
        let lit = ExprSimple::parse(&mut stream).unwrap();
        assert!(matches!(lit, ExprSimple::Literal(ExprLit::Number(_))));
    }

    #[test]
    fn test_lit_bool() {
        let mut stream = ParseStream::new(TokenStream::from_string("true").unwrap());
        let lit = ExprSimple::parse(&mut stream).unwrap();
        assert!(matches!(lit, ExprSimple::Literal(ExprLit::Bool(_))));
    }

    #[test]
    fn test_lit_string() {
        let mut stream = ParseStream::new(TokenStream::from_string("\"hello\"").unwrap());
        let lit = ExprSimple::parse(&mut stream).unwrap();
        assert!(matches!(lit, ExprSimple::Literal(ExprLit::String(_))));
    }

    #[test]
    fn test_path() {
        let mut stream = ParseStream::new(TokenStream::from_string("hello").unwrap());
        let path = ExprSimple::parse(&mut stream).unwrap();
        assert!(matches!(path, ExprSimple::Path(_)));
    }

    #[test]
    fn test_parens() {
        let mut stream = ParseStream::new(TokenStream::from_string("(123)").unwrap());
        let expr = ExprSimple::parse(&mut stream).unwrap();
        assert!(matches!(expr, ExprSimple::Grouped(_)));
    }

    #[test]
    fn test_call_noargs() {
        let mut stream = ParseStream::new(TokenStream::from_string("hello()").unwrap());
        let expr = ExprSimple::parse(&mut stream).unwrap();
        assert!(matches!(expr, ExprSimple::Call(fun, _) if matches!(*fun, ExprSimple::Path(_))));
    }

    #[test]
    fn test_call_args() {
        let mut stream = ParseStream::new(TokenStream::from_string("hello(1, 2, 3)").unwrap());
        let expr = ExprSimple::parse(&mut stream).unwrap();
        assert!(matches!(expr, ExprSimple::Call(fun, _) if matches!(*fun, ExprSimple::Path(_))));
    }

    #[test]
    fn test_unary() {
        let mut stream = ParseStream::new(TokenStream::from_string("!true").unwrap());
        let expr = ExprSimple::parse(&mut stream).unwrap();
        assert!(matches!(expr, ExprSimple::Unary(_, _)));
    }

    #[test]
    fn test_stacked_unary() {
        let mut stream = ParseStream::new(TokenStream::from_string("!!true").unwrap());
        let expr = ExprSimple::parse(&mut stream).unwrap();
        assert!(
            matches!(expr, ExprSimple::Unary(inner, _) if matches!(*inner, ExprSimple::Unary(_, _)))
        );
    }

    #[test]
    fn test_simple_binary() {
        let mut stream = ParseStream::new(TokenStream::from_string("1 + 2").unwrap());
        let expr = ExprSimple::parse(&mut stream).unwrap();
        assert!(matches!(expr, ExprSimple::Binary(_, BinaryType::Add, _)));
    }

    #[test]
    fn test_binary_precedence() {
        let mut stream = ParseStream::new(TokenStream::from_string("1 + 2 * 3").unwrap());
        let expr = ExprSimple::parse(&mut stream).unwrap();
        assert!(
            matches!(expr, ExprSimple::Binary(_, BinaryType::Add, right) if matches!(*right, ExprSimple::Binary(_, BinaryType::Mul, _)))
        );
    }

    #[test]
    fn test_complex() {
        let mut stream = ParseStream::new(
            TokenStream::from_string("test = a * (1 + hello(34)) - 8 << 7").unwrap()
        );
        let expr = ExprSimple::parse(&mut stream).unwrap();
        if let ExprSimple::Binary(left_assign, BinaryType::Assign, right_assign) = expr {
            assert!(matches!(*left_assign, ExprSimple::Path(_)));

            if let ExprSimple::Binary(left_sub, BinaryType::Sub, right_sub) = *right_assign {
                assert!(matches!(*right_sub, ExprSimple::Binary(_, BinaryType::LeftShift, _)));
                if let ExprSimple::Binary(left_mul, BinaryType::Mul, right_mul) = *left_sub {
                    assert!(matches!(*left_mul, ExprSimple::Path(_)));
                    if let ExprSimple::Grouped(Parenthesized(paren)) = *right_mul {
                        if
                            let Expr::Simple(
                                ExprSimple::Binary(left_add, BinaryType::Add, right_add),
                            ) = *paren
                        {
                            assert!(matches!(*left_add, ExprSimple::Literal(ExprLit::Number(_))));
                            assert!(matches!(*right_add, ExprSimple::Call(_, _)));
                            return;
                        }
                    }
                }
            }
        }

        panic!("Expression was parsed incorrectly.")
    }
}
