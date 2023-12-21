use crate::Span;

use super::{
    expr::Expr,
    stream::ParseStream,
    ParseResult,
    Parse,
    Spanned,
    Newline,
    modules::{ Module, UsePath },
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

        let content = if let Some(module) = stream.parse::<Module>().ok() {
            StatementContent::Module(module)
        } else if let Some(use_path) = stream.parse::<UsePath>().ok() {
            StatementContent::Use(use_path)
        } else {
            StatementContent::Expr(stream.parse::<Expr>()?)
        };

        Ok(Self {
            content,
            indent,
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum StatementContent {
    Expr(Expr),
    Module(Module),
    Use(UsePath),
}

impl Spanned for StatementContent {
    fn span(&self) -> Span {
        match self {
            Self::Expr(expr) => expr.span(),
            StatementContent::Module(module) => module.span(),
            StatementContent::Use(use_path) => use_path.span(),
        }
    }
}
