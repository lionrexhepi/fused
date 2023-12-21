use crate::tokens::{ Span, TokenType };

use super::{
    path::ExprPath,
    expr::Expr,
    stream::{ ParseStream, Cursor },
    ParseResult,
    Parse,
    Spanned,
    Newline,
    modules::Module,
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Statement {
    pub content: StatementContent,
    pub indent: usize,
}

impl Spanned for Statement {
    fn span(&self) -> Span {
        self.content.span()
    }
}

impl Parse for Statement {
    fn parse(stream: &mut ParseStream) -> ParseResult<Self> where Self: Sized {
        let indent = stream.parse::<Newline>()?.follwing_spaces;

        let expr = stream.parse::<Expr>()?;

        Ok(Self {
            content: StatementContent::Expr(expr),
            indent,
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum StatementContent {
    Expr(Expr),
    Module(Module),
}

impl Spanned for StatementContent {
    fn span(&self) -> Span {
        match self {
            Self::Expr(expr) => expr.span(),
            StatementContent::Module(module) => module.span(),
        }
    }
}
