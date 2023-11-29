use super::{number::LitNumber, ident::ExprIdent, string::LitString};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Expr {
    Literal(ExprLit),
    Ident(ExprIdent),
}


#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ExprLit {
    String(LitString),
    Number(LitNumber),
}
