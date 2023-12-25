use crate::Span;

use super::{
    ident::Ident,
    expr::Expr,
    Spanned,
    Parse,
    keywords::{ Mut, Let },
    punct::{ Ampersand, Eq },
    stream::ParseStream,
    ParseResult,
    grouped::Bracketed,
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ExprDecl {
    pub name: Ident,
    pub ty: Option<Ident>,
    pub value: Option<Box<Expr>>,
    pub mutable: bool,
}

impl Spanned for ExprDecl {
    fn span(&self) -> Span {
        self.name.span()
    }
}

impl Parse for ExprDecl {
    fn parse(stream: &mut ParseStream) -> ParseResult<Self> where Self: Sized {
        let mutable = stream.parse::<Mut>().is_ok();
        let name = stream.parse::<Ident>()?;
        let ty = stream
            .parse::<Bracketed<Ident>>()
            .ok()
            .map(|bracket| *bracket.0);
        let value = if stream.parse::<Eq>().is_ok() {
            Some(Box::new(stream.parse::<Expr>()?))
        } else {
            None
        };
        Ok(Self {
            name,
            ty,
            value,
            mutable,
        })
    }

    fn could_parse(stream: &mut ParseStream) -> bool {
        Mut::could_parse(stream) || Ident::could_parse(stream)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ExprLet(ExprDecl);

impl Spanned for ExprLet {
    fn span(&self) -> Span {
        self.0.span()
    }
}

impl Parse for ExprLet {
    fn parse(stream: &mut ParseStream) -> ParseResult<Self> where Self: Sized {
        stream.parse::<Let>()?;
        let decl = stream.parse::<ExprDecl>()?;
        Ok(Self(decl))
    }

    fn could_parse(stream: &mut ParseStream) -> bool {
        Let::could_parse(stream)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FnArg {
    pub reference: bool,
    pub decl: ExprDecl,
}

impl Spanned for FnArg {
    fn span(&self) -> Span {
        self.decl.span()
    }
}

impl Parse for FnArg {
    fn parse(stream: &mut ParseStream) -> ParseResult<Self> where Self: Sized {
        let reference = stream.parse::<Ampersand>().is_ok();
        let decl = stream.parse::<ExprDecl>()?;
        Ok(Self {
            reference,
            decl,
        })
    }

    fn could_parse(stream: &mut ParseStream) -> bool {
        Ampersand::could_parse(stream) || ExprDecl::could_parse(stream)
    }
}

#[cfg(test)]
mod test {
    use crate::tokens::stream::TokenStream;

    use super::{ super::stream::ParseStream, FnArg, ExprLet, ExprDecl };

    #[test]
    fn test_simple_untyped() {
        let tokens = TokenStream::from_string("a = 1 \n mut b = 2").unwrap();
        let mut stream = ParseStream::new(tokens);
        let decl = stream.parse::<ExprDecl>().unwrap();
        assert_eq!(decl.name.name, "a");
        assert!(decl.value.is_some());
        assert!(decl.ty.is_none());
        assert_eq!(decl.mutable, false);
        stream.skip_newlines();
        let decl = stream.parse::<ExprDecl>().unwrap();
        assert_eq!(decl.name.name, "b");
        assert!(decl.value.is_some());
        assert!(decl.ty.is_none());
        assert_eq!(decl.mutable, true);
    }

    #[test]
    fn test_simple_typed() {
        let tokens = TokenStream::from_string("a[i32] = 1 \n mut b[i32] = 2").unwrap();
        let mut stream = ParseStream::new(tokens);
        let decl = stream.parse::<ExprDecl>().unwrap();
        assert_eq!(decl.name.name, "a");
        assert!(decl.value.is_some());
        assert_eq!(decl.mutable, false);
        assert_eq!(decl.ty.unwrap().name, "i32");
        stream.skip_newlines();
        let decl = stream.parse::<ExprDecl>().unwrap();
        assert_eq!(decl.name.name, "b");
        assert!(decl.value.is_some());
        assert_eq!(decl.mutable, true);
        assert_eq!(decl.ty.unwrap().name, "i32");
    }

    #[test]
    fn test_simple_unassigned() {
        let tokens = TokenStream::from_string("a[i32] \n mut b[i32]").unwrap();
        let mut stream = ParseStream::new(tokens);
        let decl = stream.parse::<ExprDecl>().unwrap();
        assert_eq!(decl.name.name, "a");
        assert!(decl.value.is_none());
        assert_eq!(decl.mutable, false);
        assert_eq!(decl.ty.unwrap().name, "i32");
        stream.skip_newlines();
        let decl = stream.parse::<ExprDecl>().unwrap();
        assert_eq!(decl.name.name, "b");
        assert!(decl.value.is_none());
        assert_eq!(decl.mutable, true);
        assert_eq!(decl.ty.unwrap().name, "i32");
    }

    #[test]
    fn test_let_untyped() {
        let tokens = TokenStream::from_string("let a = 1 \n let mut b = 2").unwrap();
        let mut stream = ParseStream::new(tokens);
        let decl = stream.parse::<ExprLet>().unwrap();
        assert_eq!(decl.0.name.name, "a");
        assert!(decl.0.value.is_some());
        assert!(decl.0.ty.is_none());
        assert_eq!(decl.0.mutable, false);
        stream.skip_newlines();
        let decl = stream.parse::<ExprLet>().unwrap();
        assert_eq!(decl.0.name.name, "b");
        assert!(decl.0.value.is_some());
        assert!(decl.0.ty.is_none());
        assert_eq!(decl.0.mutable, true);
    }

    #[test]
    fn test_let_typed() {
        let tokens = TokenStream::from_string("let a[i32] = 1 \n let mut b[i32] = 2").unwrap();
        let mut stream = ParseStream::new(tokens);
        let decl = stream.parse::<ExprLet>().unwrap();
        assert_eq!(decl.0.name.name, "a");
        assert!(decl.0.value.is_some());
        assert_eq!(decl.0.mutable, false);
        assert_eq!(decl.0.ty.unwrap().name, "i32");
        stream.skip_newlines();
        let decl = stream.parse::<ExprLet>().unwrap();
        assert_eq!(decl.0.name.name, "b");
        assert!(decl.0.value.is_some());
        assert_eq!(decl.0.mutable, true);
        assert_eq!(decl.0.ty.unwrap().name, "i32");
    }

    #[test]
    fn test_let_unassigned() {
        let tokens = TokenStream::from_string("let a[i32] \n let mut b[i32]").unwrap();
        let mut stream = ParseStream::new(tokens);
        let decl = stream.parse::<ExprLet>().unwrap();
        assert_eq!(decl.0.name.name, "a");
        assert!(decl.0.value.is_none());
        assert_eq!(decl.0.mutable, false);
        assert_eq!(decl.0.ty.unwrap().name, "i32");
        stream.skip_newlines();
        let decl = stream.parse::<ExprLet>().unwrap();
        assert_eq!(decl.0.name.name, "b");
        assert!(decl.0.value.is_none());
        assert_eq!(decl.0.mutable, true);
        assert_eq!(decl.0.ty.unwrap().name, "i32");
    }

    #[test]
    fn test_fn_arg_untyped() {
        let tokens = TokenStream::from_string("a").unwrap();
        let mut stream = ParseStream::new(tokens);
        let arg = stream.parse::<FnArg>().unwrap();
        assert_eq!(arg.decl.name.name, "a");
        assert!(arg.decl.value.is_none());
        assert_eq!(arg.decl.mutable, false);
        assert!(arg.decl.ty.is_none());
        assert_eq!(arg.reference, false);
    }

    #[test]
    fn test_fn_arg_typed() {
        let tokens = TokenStream::from_string("a[i32]").unwrap();
        let mut stream = ParseStream::new(tokens);
        let arg = stream.parse::<FnArg>().unwrap();
        assert_eq!(arg.decl.name.name, "a");
        assert!(arg.decl.value.is_none());
        assert_eq!(arg.decl.mutable, false);
        assert_eq!(arg.decl.ty.unwrap().name, "i32");
        assert_eq!(arg.reference, false);
    }

    #[test]
    fn test_fn_arg_defaulted() {
        let tokens = TokenStream::from_string("a[i32] = 1").unwrap();
        let mut stream = ParseStream::new(tokens);
        let arg = stream.parse::<FnArg>().unwrap();
        assert_eq!(arg.decl.name.name, "a");
        assert!(arg.decl.value.is_some());
        assert_eq!(arg.decl.mutable, false);
        assert_eq!(arg.decl.ty.unwrap().name, "i32");
        assert_eq!(arg.reference, false);
    }

    #[test]
    fn test_fn_arg_ref() {
        let tokens = TokenStream::from_string("&a[i32]").unwrap();
        let mut stream = ParseStream::new(tokens);
        let arg = stream.parse::<FnArg>().unwrap();
        assert_eq!(arg.decl.name.name, "a");
        assert!(arg.decl.value.is_none());
        assert_eq!(arg.decl.mutable, false);
        assert_eq!(arg.decl.ty.unwrap().name, "i32");
        assert_eq!(arg.reference, true);
    }
}
