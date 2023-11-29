use super::{number::LitNumber, ident::ExprIdent};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Expr {
    Literal(ExprLit),
    Ident(ExprIdent),
}


#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ExprLit {
    String,
    Number(LitNumber),
}
