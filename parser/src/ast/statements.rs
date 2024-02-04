use crate::{ tokens::Token, Span };

use super::{
    expr::Expr,
    stream::ParseStream,
    ParseResult,
    Parse,
    Spanned,
    modules::{ Module, UsePath },
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Statement {
    pub content: StatementContent,
}

impl Spanned for Statement {
    fn span(&self) -> Span {
        self.content.span()
    }
}

impl Parse for Statement {
    fn parse(stream: &mut ParseStream) -> ParseResult<Self> where Self: Sized {
        let content = if let Some(module) = stream.parse::<Module>().ok() {
            StatementContent::Module(module)
        } else if let Some(use_path) = stream.parse::<UsePath>().ok() {
            StatementContent::Use(use_path)
        } else {
            let expr = stream.parse::<Expr>()?;
            if expr == Expr::Empty {
                return Err(super::ParseError::UnexpectedToken {
                    expected: "<statement>",
                    got: stream.current().clone(),
                });
            }
            StatementContent::Expr(expr)
        };

        Ok(Self {
            content,
        })
    }

    fn could_parse(stream: &mut ParseStream) -> bool {
        Expr::could_parse(stream) || Module::could_parse(stream) || UsePath::could_parse(stream)
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
