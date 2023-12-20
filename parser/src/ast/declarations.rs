use crate::tokens::Span;

use super::{ ident::ExprIdent, expr::Expr, Spanned, Parse, keywords::Mut };

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ExprDecl {
    pub name: ExprIdent,
    pub ty: Option<ExprIdent>,
    pub value: Option<Box<Expr>>,
    pub mutable: bool,
}

impl Spanned for ExprDecl {
    fn span(&self) -> Span {
        self.name.span()
    }
}

impl Parse for ExprDecl {
    fn parse(stream: &mut super::stream::ParseStream) -> super::ParseResult<Self> where Self: Sized {
        let mutable = stream.parse::<Mut>().is_ok();
        let name = stream.parse::<ExprIdent>()?;
        let ty = stream
            .parse::<super::grouped::Bracketed<ExprIdent>>()
            .ok()
            .map(|bracket| *bracket.0);
        let value = if stream.parse::<super::punct::Eq>().is_ok() {
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
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ExprLet(ExprDecl);

impl Spanned for ExprLet {
    fn span(&self) -> Span {
        self.0.span()
    }
}

impl Parse for ExprLet {
    fn parse(stream: &mut super::stream::ParseStream) -> super::ParseResult<Self> where Self: Sized {
        stream.parse::<super::keywords::Let>()?;
        let decl = stream.parse::<ExprDecl>()?;
        Ok(Self(decl))
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
    fn parse(stream: &mut super::stream::ParseStream) -> super::ParseResult<Self> where Self: Sized {
        let reference = stream.parse::<super::punct::Ampersand>().is_ok();
        let decl = stream.parse::<ExprDecl>()?;
        Ok(Self {
            reference,
            decl,
        })
    }
}

#[cfg(test)]
mod test {
    use crate::tokens::stream::TokenStream;

    #[test]
    fn test_simple_untyped() {
        let tokens = TokenStream::from_string("a = 1 \n mut b = 2".to_string()).unwrap();
        let mut stream = super::super::stream::ParseStream::new(tokens);
        let decl = stream.parse::<super::ExprDecl>().unwrap();
        assert_eq!(decl.name.name, "a");
        assert!(decl.value.is_some());
        assert!(decl.ty.is_none());
        assert_eq!(decl.mutable, false);
        stream.skip_newlines();
        let decl = stream.parse::<super::ExprDecl>().unwrap();
        assert_eq!(decl.name.name, "b");
        assert!(decl.value.is_some());
        assert!(decl.ty.is_none());
        assert_eq!(decl.mutable, true);
    }

    #[test]
    fn test_simple_typed() {
        let tokens = TokenStream::from_string("a[i32] = 1 \n mut b[i32] = 2".to_string()).unwrap();
        let mut stream = super::super::stream::ParseStream::new(tokens);
        let decl = stream.parse::<super::ExprDecl>().unwrap();
        assert_eq!(decl.name.name, "a");
        assert!(decl.value.is_some());
        assert_eq!(decl.mutable, false);
        assert_eq!(decl.ty.unwrap().name, "i32");
        stream.skip_newlines();
        let decl = stream.parse::<super::ExprDecl>().unwrap();
        assert_eq!(decl.name.name, "b");
        assert!(decl.value.is_some());
        assert_eq!(decl.mutable, true);
        assert_eq!(decl.ty.unwrap().name, "i32");
    }

    #[test]
    fn test_simple_unassigned() {
        let tokens = TokenStream::from_string("a[i32] \n mut b[i32]".to_string()).unwrap();
        let mut stream = super::super::stream::ParseStream::new(tokens);
        let decl = stream.parse::<super::ExprDecl>().unwrap();
        assert_eq!(decl.name.name, "a");
        assert!(decl.value.is_none());
        assert_eq!(decl.mutable, false);
        assert_eq!(decl.ty.unwrap().name, "i32");
        stream.skip_newlines();
        let decl = stream.parse::<super::ExprDecl>().unwrap();
        assert_eq!(decl.name.name, "b");
        assert!(decl.value.is_none());
        assert_eq!(decl.mutable, true);
        assert_eq!(decl.ty.unwrap().name, "i32");
    }

    #[test]
    fn test_let_untyped() {
        let tokens = TokenStream::from_string("let a = 1 \n let mut b = 2".to_string()).unwrap();
        let mut stream = super::super::stream::ParseStream::new(tokens);
        let decl = stream.parse::<super::ExprLet>().unwrap();
        assert_eq!(decl.0.name.name, "a");
        assert!(decl.0.value.is_some());
        assert!(decl.0.ty.is_none());
        assert_eq!(decl.0.mutable, false);
        stream.skip_newlines();
        let decl = stream.parse::<super::ExprLet>().unwrap();
        assert_eq!(decl.0.name.name, "b");
        assert!(decl.0.value.is_some());
        assert!(decl.0.ty.is_none());
        assert_eq!(decl.0.mutable, true);
    }

    #[test]
    fn test_let_typed() {
        let tokens = TokenStream::from_string(
            "let a[i32] = 1 \n let mut b[i32] = 2".to_string()
        ).unwrap();
        let mut stream = super::super::stream::ParseStream::new(tokens);
        let decl = stream.parse::<super::ExprLet>().unwrap();
        assert_eq!(decl.0.name.name, "a");
        assert!(decl.0.value.is_some());
        assert_eq!(decl.0.mutable, false);
        assert_eq!(decl.0.ty.unwrap().name, "i32");
        stream.skip_newlines();
        let decl = stream.parse::<super::ExprLet>().unwrap();
        assert_eq!(decl.0.name.name, "b");
        assert!(decl.0.value.is_some());
        assert_eq!(decl.0.mutable, true);
        assert_eq!(decl.0.ty.unwrap().name, "i32");
    }

    #[test]
    fn test_let_unassigned() {
        let tokens = TokenStream::from_string("let a[i32] \n let mut b[i32]".to_string()).unwrap();
        let mut stream = super::super::stream::ParseStream::new(tokens);
        let decl = stream.parse::<super::ExprLet>().unwrap();
        assert_eq!(decl.0.name.name, "a");
        assert!(decl.0.value.is_none());
        assert_eq!(decl.0.mutable, false);
        assert_eq!(decl.0.ty.unwrap().name, "i32");
        stream.skip_newlines();
        let decl = stream.parse::<super::ExprLet>().unwrap();
        assert_eq!(decl.0.name.name, "b");
        assert!(decl.0.value.is_none());
        assert_eq!(decl.0.mutable, true);
        assert_eq!(decl.0.ty.unwrap().name, "i32");
    }

    #[test]
    fn test_fn_arg_untyped() {
        let tokens = TokenStream::from_string("a".to_string()).unwrap();
        let mut stream = super::super::stream::ParseStream::new(tokens);
        let arg = stream.parse::<super::FnArg>().unwrap();
        assert_eq!(arg.decl.name.name, "a");
        assert!(arg.decl.value.is_none());
        assert_eq!(arg.decl.mutable, false);
        assert!(arg.decl.ty.is_none());
        assert_eq!(arg.reference, false);
    }

    #[test]
    fn test_fn_arg_typed() {
        let tokens = TokenStream::from_string("a[i32]".to_string()).unwrap();
        let mut stream = super::super::stream::ParseStream::new(tokens);
        let arg = stream.parse::<super::FnArg>().unwrap();
        assert_eq!(arg.decl.name.name, "a");
        assert!(arg.decl.value.is_none());
        assert_eq!(arg.decl.mutable, false);
        assert_eq!(arg.decl.ty.unwrap().name, "i32");
        assert_eq!(arg.reference, false);
    }

    #[test]
    fn test_fn_arg_defaulted() {
        let tokens = TokenStream::from_string("a[i32] = 1".to_string()).unwrap();
        let mut stream = super::super::stream::ParseStream::new(tokens);
        let arg = stream.parse::<super::FnArg>().unwrap();
        assert_eq!(arg.decl.name.name, "a");
        assert!(arg.decl.value.is_some());
        assert_eq!(arg.decl.mutable, false);
        assert_eq!(arg.decl.ty.unwrap().name, "i32");
        assert_eq!(arg.reference, false);
    }

    #[test]
    fn test_fn_arg_ref() {
        let tokens = TokenStream::from_string("&a[i32]".to_string()).unwrap();
        let mut stream = super::super::stream::ParseStream::new(tokens);
        let arg = stream.parse::<super::FnArg>().unwrap();
        assert_eq!(arg.decl.name.name, "a");
        assert!(arg.decl.value.is_none());
        assert_eq!(arg.decl.mutable, false);
        assert_eq!(arg.decl.ty.unwrap().name, "i32");
        assert_eq!(arg.reference, true);
    }
}
