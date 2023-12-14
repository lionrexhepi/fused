use crate::tokens::{ Span, TokenType, punct::TokenPunct };

use super::{ expr::Expr, Spanned, Parse, stream::{ ParseStream, Cursor }, ParseError, ParseResult };

pub struct Unary {
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

impl Spanned for Unary {
    fn span(&self) -> Span {
        self.span
    }
}

impl Parse for Unary {
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

pub struct Binary {
    pub left: Box<Expr>,
    pub right: Box<Expr>,
    span: Span,
    pub ty: BinaryType,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum BinaryType {
    Assign, //Do NOT shorten.
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    And,
    Or,
    Eq,
    Neq,
    Lt,
    Gt,
    Leq,
    Geq,
    
}

impl Spanned for Binary {
    fn span(&self) -> Span {
        self.span
    }
}

impl Parse for Binary {
    fn parse(stream: &mut ParseStream) -> ParseResult<Self> where Self: Sized {
        let mut split = stream.clone();
        let left = split.parse::<Expr>()?;
        let span = left.span();
        let r#type = split.parse_with(|cursor: &mut Cursor| {
            let ty = match &cursor.current().content {
                TokenType::Punct(punct) => match punct {
                    TokenPunct::Plus => BinaryType::Add,
                    TokenPunct::Minus => BinaryType::Sub,
                    TokenPunct::Star => BinaryType::Mul,
                    TokenPunct::Slash => BinaryType::Div,
                    TokenPunct::Percent => BinaryType::Mod,
                    TokenPunct::Ampersand => BinaryType::And,
                    TokenPunct::DoublePipe => BinaryType::Or,
                    TokenPunct::DoubleEq => BinaryType::Eq,
                    TokenPunct::NotEq => BinaryType::Neq,
                    TokenPunct::Lt => BinaryType::Lt,
                    TokenPunct::Gt => BinaryType::Gt,
                    TokenPunct::LtEq => BinaryType::Leq,
                    TokenPunct::GtEq => BinaryType::Geq,
                    TokenPunct::Eq => BinaryType::Assign,
                    _ => return Err(ParseError::UnexpectedToken("binary operator", cursor.current().clone())),
                },
                _ => return Err(ParseError::UnexpectedToken("binary operator", cursor.current().clone())),
            };
            cursor.advance();
            Ok(ty)
            
        
        })?;
        let right = split.parse::<Expr>()?;
        *stream = split;
        Ok(Self {
            left: Box::new(left),
            right: Box::new(right),
            span,
            ty: r#type,
        })
    }
}


mod test {
    use crate::ast::number::LitNumber;

    #[test]
    fn test_unary() {
        let  stream = crate::tokens::stream::TokenStream::from_string("!1 *2".to_string()).unwrap()  ;
        let mut stream = crate::ast::stream::ParseStream::new(stream);
        let unary = stream.parse::<crate::ast::operations::Unary>().unwrap();
        assert_eq!(unary.ty, crate::ast::operations::UnaryType::Not);
        assert!(matches!(*unary.arg, crate::ast::expr::Expr::Literal(crate::ast::expr::ExprLit::Number(_))));
        let unary = stream.parse::<crate::ast::operations::Unary>().unwrap();
        assert_eq!(unary.ty, crate::ast::operations::UnaryType::Deref);
    }

    #[test]
    fn test_binary() {
        let  stream = crate::tokens::stream::TokenStream::from_string("1 + 2".to_string()).unwrap()  ;
        let mut stream = crate::ast::stream::ParseStream::new(stream);
        let binary = stream.parse::<crate::ast::operations::Binary>().unwrap();
        assert_eq!(binary.ty, crate::ast::operations::BinaryType::Add);
        assert!(matches!(*binary.left, crate::ast::expr::Expr::Literal(crate::ast::expr::ExprLit::Number(_))));
        assert!(matches!(*binary.right, crate::ast::expr::Expr::Literal(crate::ast::expr::ExprLit::Number(_))));
    }
}