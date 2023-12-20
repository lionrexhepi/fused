use std::{ collections::HashMap, mem::MaybeUninit };

use crate::{
    tokens::{ Span, TokenType, punct::TokenPunct },
    ast::{ stream::UnexpectedToken, punct::StarEq },
};

use super::{
    expr::{ Expr, ExprLit },
    Spanned,
    Parse,
    stream::{ ParseStream, Cursor },
    ParseError,
    ParseResult,
    ident::Ident,
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ExprUnary {
    pub arg: Box<Expr>,
    span: Span,
    pub ty: UnaryType,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum UnaryType {
    Not,
    Deref,
    Incr,
    Decr,
}

impl Spanned for ExprUnary {
    fn span(&self) -> Span {
        self.span
    }
}

impl Parse for ExprUnary {
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

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ExprBinary {
    pub left: Box<Expr>,
    pub right: Box<Expr>,
    span: Span,
    pub ty: BinaryType,
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
    Access,
    LeftShift,
    RightShift,
}

impl BinaryType {
    ///Get the operator's precedence and whether it is left-associative.
    fn precedence(&self) -> u8 {
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
            | Self::RightShiftAssign => 0,
            Self::Or => 1,
            Self::And => 2,
            Self::Eq | Self::Neq | Self::Lt | Self::Gt | Self::Leq | Self::Geq => 3,
            Self::BitOr => 4,
            Self::BitXor => 5,
            Self::BitAnd => 6,
            Self::LeftShift | Self::RightShift => 7,
            Self::Add | Self::Sub => 8,
            Self::Mul | Self::Div | Self::Mod => 9,
            Self::Access => 10,
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

impl Spanned for ExprBinary {
    fn span(&self) -> Span {
        self.span
    }
}

impl Parse for ExprBinary {
    fn parse(stream: &mut ParseStream) -> ParseResult<Self> where Self: Sized {
        let mut split = stream.clone();
        let mut operators = Vec::with_capacity(1);
        let mut arguments = Vec::with_capacity(2);
        let mut left_arg = 0usize;
        loop {
            arguments.push(
                Self::read_fragment(&mut split).ok_or_else(|| {
                    ParseError::UnexpectedToken("expression", split.current().clone())
                })?
            );

            if let Some(operator) = Self::read_operator(&mut split) {
                operators.push((left_arg, operator));
                left_arg += 1;
            } else {
                break;
            }
        }

        if operators.is_empty() {
            return Err(ParseError::UnexpectedToken("", split.current().clone()));
        } else if arguments.len() < operators.len() + 1 {
            return Err(ParseError::UnexpectedToken("Expression", split.current().clone()));
        } else {
            operators.sort_by(|(arg_left, left), (arg_right, right)| {
                let precedences = right.precedence().cmp(&left.precedence());
                if precedences == std::cmp::Ordering::Equal {
                    if left.left_associative() {
                        arg_left.cmp(arg_right)
                    } else {
                        arg_right.cmp(arg_left)
                    }
                } else {
                    precedences
                }
            });

            let mut lookup = arguments
                .iter()
                .enumerate()
                .map(|(i, _)| (i, i))
                .collect::<HashMap<_, _>>();

            for (argument, operator) in operators {
                let left_index = *lookup.get(&argument).unwrap();
                let right_index = *lookup.get(&(argument + 1)).unwrap();
                lookup.insert(right_index, left_index); //The left and right are now merged and should point to the same expr

                let right = arguments.remove(right_index);

                //SAFETY: left_index will immediately be set to a value again, making this safe
                #[allow(invalid_value)]
                let left = unsafe {
                    std::mem::replace(
                        &mut arguments[left_index],
                        MaybeUninit::uninit().assume_init()
                    )
                };
                let span = left.span().join(right.span());
                arguments[left_index] = Expr::Binary(Self {
                    left: Box::new(left),
                    right: Box::new(right),
                    span,
                    ty: operator,
                });
            }

            debug_assert!(arguments.len() == 1);
            Ok(match arguments.remove(0) {
                Expr::Binary(binary) => {
                    *stream = split;
                    binary
                }
                _ => unreachable!(),
            })
        }
    }
}

impl ExprBinary {
    fn read_fragment(stream: &mut ParseStream) -> Option<Expr> {
        if let Ok(unary) = stream.parse() {
            Some(Expr::Unary(unary))
        } else if let Ok(literal) = stream.parse() {
            Some(Expr::Literal(literal))
        } else if let Ok(ident) = stream.parse() {
            Some(Expr::Ident(ident))
        } else if let Ok(parenthesized) = stream.parse() {
            Some(Expr::Grouped(parenthesized))
        } else {
            None
        }
    }

    fn read_operator(stream: &mut ParseStream) -> Option<BinaryType> {
        stream
            .parse_with(|cursor: &mut Cursor| {
                let operator = match &cursor.current().content {
                    TokenType::Punct(punct) =>
                        match punct {
                            TokenPunct::Plus => BinaryType::Add,
                            TokenPunct::Minus => BinaryType::Sub,
                            TokenPunct::Star => BinaryType::Mul,
                            TokenPunct::Slash => BinaryType::Div,
                            TokenPunct::Percent => BinaryType::Mod,
                            TokenPunct::Ampersand => BinaryType::BitAnd,
                            TokenPunct::DoubleAmpersand => BinaryType::And,
                            TokenPunct::Pipe => BinaryType::BitOr,
                            TokenPunct::Caret => BinaryType::BitXor,
                            TokenPunct::DoublePipe => BinaryType::Or,
                            TokenPunct::DoubleEq => BinaryType::Eq,
                            TokenPunct::NotEq => BinaryType::Neq,
                            TokenPunct::Lt => BinaryType::Lt,
                            TokenPunct::Gt => BinaryType::Gt,
                            TokenPunct::LtEq => BinaryType::Leq,
                            TokenPunct::GtEq => BinaryType::Geq,
                            TokenPunct::Eq => BinaryType::Assign,
                            TokenPunct::PlusEq => BinaryType::AddAssign,
                            TokenPunct::MinusEq => BinaryType::SubAssign,
                            TokenPunct::StarEq => BinaryType::MulAssign,
                            TokenPunct::SlashEq => BinaryType::DivAssign,
                            TokenPunct::PercentEq => BinaryType::ModAssign,
                            TokenPunct::DoubleAmpersandEq => BinaryType::AndAssign,
                            TokenPunct::DoublePipeEq => BinaryType::OrAssign,
                            TokenPunct::AmpersandEq => BinaryType::BitAndAssign,
                            TokenPunct::PipeEq => BinaryType::BitOrAssign,
                            TokenPunct::CaretEq => BinaryType::BitXorAssign,
                            TokenPunct::Dot => BinaryType::Access,
                            TokenPunct::DoubleLt => BinaryType::LeftShift,
                            TokenPunct::DoubleGt => BinaryType::RightShift,
                            TokenPunct::DoubleLtEq => BinaryType::LeftShiftAssign,
                            TokenPunct::DoubleGtEq => BinaryType::RightShiftAssign,

                            _ => {
                                return Err(
                                    ParseError::UnexpectedToken(
                                        "operator",
                                        cursor.current().clone()
                                    )
                                );
                            }
                        }
                    _ => {
                        return Err(
                            ParseError::UnexpectedToken("operator", cursor.current().clone())
                        );
                    }
                };
                cursor.advance();
                Ok(operator)
            })
            .ok()
    }
}

mod test {
    use crate::ast::number::LitNumber;

    #[test]
    fn test_unary() {
        let stream = crate::tokens::stream::TokenStream::from_string("*1 !2".to_string()).unwrap();
        let mut stream = crate::ast::stream::ParseStream::new(stream);
        let unary = stream.parse::<crate::ast::operations::ExprUnary>().unwrap();
        assert_eq!(unary.ty, crate::ast::operations::UnaryType::Deref);
        let unary = stream.parse::<crate::ast::operations::ExprUnary>().unwrap();
        assert_eq!(unary.ty, crate::ast::operations::UnaryType::Not);
        assert!(
            matches!(
                *unary.arg,
                crate::ast::expr::Expr::Literal(crate::ast::expr::ExprLit::Number(_))
            )
        );
    }

    #[test]
    fn test_binary() {
        let stream = crate::tokens::stream::TokenStream::from_string("1 + 2".to_string()).unwrap();
        let mut stream = crate::ast::stream::ParseStream::new(stream);
        let binary = stream.parse::<crate::ast::operations::ExprBinary>().unwrap();
        assert_eq!(binary.ty, crate::ast::operations::BinaryType::Add);
        assert!(
            matches!(
                *binary.left,
                crate::ast::expr::Expr::Literal(crate::ast::expr::ExprLit::Number(_))
            )
        );
        assert!(
            matches!(
                *binary.right,
                crate::ast::expr::Expr::Literal(crate::ast::expr::ExprLit::Number(_))
            )
        );
    }

    #[test]
    fn test_binary_precedence() {
        let stream = crate::tokens::stream::TokenStream
            ::from_string("1 + 2 * 3".to_string())
            .unwrap();
        let mut stream = crate::ast::stream::ParseStream::new(stream);
        let binary = stream.parse::<crate::ast::operations::ExprBinary>().unwrap();

        assert_eq!(binary.ty, crate::ast::operations::BinaryType::Add);
        assert!(matches!(*binary.right, crate::ast::expr::Expr::Binary(_)));
        assert!(
            matches!(
                *binary.left,
                crate::ast::expr::Expr::Literal(crate::ast::expr::ExprLit::Number(_))
            )
        );
    }
}
