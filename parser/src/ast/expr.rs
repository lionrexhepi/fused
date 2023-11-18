use super::number::LitNumber;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Expr {
    Literal,
    Ident(ExprIdent),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ExprIdent(String);

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ExprLit {
    String,
    Number(LitNumber),
}
