use crate::Span;

use super::{
    expr::{ Expr, ExprLit },
    Spanned,
    Parse,
    stream::ParseStream,
    ParseResult,
    punct::{
        Eq,
        DoublePipe,
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
    grouped::Parenthesized,
    separated::Separated,
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ExprSimple {
    Assign(ExprPath, Box<Self>),
    Binary(Box<Self>, BinaryType, Box<Self>),
    Unary(Box<Self>, UnaryType),
    Call(Box<Self>, Separated<ExprSimple, Comma>),
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
            Self::Assign(path, val) => path.span().join(val.span()),
        }
    }
}

impl Parse for ExprSimple {
    fn parse(stream: &mut ParseStream) -> ParseResult<Self> where Self: Sized {
        Self::parse_assign(stream)
    }

    fn could_parse(stream: &mut ParseStream) -> bool {
        Parenthesized::<ExprSimple>::could_parse(stream) ||
            ExprPath::could_parse(stream) ||
            ExprLit::could_parse(stream) ||
            Exclamation::could_parse(stream) ||
            Minus::could_parse(stream)
    }
}

impl ExprSimple {
    fn parse_assign(stream: &mut ParseStream) -> ParseResult<Self> {
        let target = Self::parse_binary_level(stream, 0)?;
        match target {
            Self::Path(path) if stream.parse::<Eq>().is_ok() => {
                let value = Self::parse_assign(stream)?;
                Ok(Self::Assign(path, Box::new(value)))
            }
            other => Ok(other),
        }
    }

    fn parse_binary_level(stream: &mut ParseStream, precedence: usize) -> ParseResult<Self> {
        if precedence >= BinaryType::PRECEDENCE_LEVELS.len() {
            return Self::parse_unary_prefix(stream);
        }

        let left = Self::parse_binary_level(stream, precedence + 1)?;
        let mut ty = None;

        for operator in BinaryType::PRECEDENCE_LEVELS[precedence] {
            if operator.matches(stream) {
                ty = Some(*operator);
            }
            println!("{:?} not found, instead: {:?}", operator, stream.current().content);
        }

        if let Some(ty) = ty {
            let right = Self::parse_binary_level(stream, precedence)?;
            Ok(Self::Binary(Box::new(left), ty, Box::new(right)))
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

        while Parenthesized::<Separated<ExprSimple>>::could_parse(stream) {
            let args = stream.parse::<Parenthesized<_>>()?;
            callee = Self::Call(Box::new(callee), *args.0);
        }

        Ok(callee)
    }

    fn parse_grouped(stream: &mut ParseStream) -> ParseResult<Self> {
        if Parenthesized::<ExprSimple>::could_parse(stream) {
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
    pub const PRECEDENCE_LEVELS: [&'static [Self]; 7] = [
        &[Self::And, Self::Or],
        &[Self::Eq, Self::Neq],
        &[Self::Lt, Self::Gt, Self::Leq, Self::Geq],
        &[Self::Add, Self::Sub],
        &[Self::Mul, Self::Div, Self::Mod],
        &[Self::BitAnd, Self::BitOr, Self::BitXor],
        &[Self::LeftShift, Self::RightShift],
    ];

    fn matches(&self, stream: &mut ParseStream) -> bool {
        match self {
            Self::Or => stream.parse::<DoublePipe>().is_ok(),
            Self::Eq => stream.parse::<DoubleEq>().is_ok(),
            Self::Neq => stream.parse::<NotEq>().is_ok(),
            Self::Lt => stream.parse::<Lt>().is_ok(),
            Self::Gt => stream.parse::<Gt>().is_ok(),
            Self::Leq => stream.parse::<LtEq>().is_ok(),
            Self::Geq => stream.parse::<GtEq>().is_ok(),
            Self::Add => stream.parse::<Plus>().is_ok(),
            Self::Sub => stream.parse::<Minus>().is_ok(),
            Self::Mul => stream.parse::<Star>().is_ok(),
            Self::Div => stream.parse::<Slash>().is_ok(),
            Self::Mod => stream.parse::<Percent>().is_ok(),
            Self::And => stream.parse::<DoubleAmpersand>().is_ok(),
            Self::BitAnd => stream.parse::<Ampersand>().is_ok(),
            Self::BitOr => stream.parse::<Pipe>().is_ok(),
            Self::BitXor => stream.parse::<Caret>().is_ok(),
            Self::LeftShift => stream.parse::<DoubleLt>().is_ok(),
            Self::RightShift => stream.parse::<DoubleGt>().is_ok(),
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
        if let ExprSimple::Assign(_, right_assign) = expr {
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

    #[test]
    fn test_chained_assign() {
        let mut stream = ParseStream::new(TokenStream::from_string("a = b = c").unwrap());
        let expr = ExprSimple::parse(&mut stream).unwrap();
        assert!(
            matches!(expr, ExprSimple::Assign(_, right) if matches!(*right, ExprSimple::Assign(_, _)))
        );
    }
}
