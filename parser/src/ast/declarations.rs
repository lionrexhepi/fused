use crate::{ Span, tokens::{ punct::TokenPunct, TokenType, group::{ Delim, TokenGroup } } };

use super::{
    ident::Ident,
    expr::Expr,
    Spanned,
    Parse,
    keywords::Mut,
    punct::{ Eq, Colon },
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
        let name = stream.parse::<Ident>()?;
        let ty = stream
            .parse::<Bracketed<Ident>>()
            .ok()
            .map(|bracket| *bracket.0);
        stream.parse::<Colon>()?;
        let mutable = stream.parse::<Mut>().is_ok();
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
        Ident::could_parse(stream) &&
            matches!(
                stream.cursor().next().content,
                TokenType::Punct(TokenPunct::Colon) |
                    TokenType::Group(TokenGroup { delim: Delim::Bracket, .. })
            )
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FnArg {
    pub name: Ident,
    pub ty: Option<Ident>,
    pub mutable: bool,
    pub default: Option<Box<Expr>>,
}

impl Spanned for FnArg {
    fn span(&self) -> Span {
        self.name.span()
    }
}

impl Parse for FnArg {
    fn parse(stream: &mut ParseStream) -> ParseResult<Self> where Self: Sized {
        let name = stream.parse::<Ident>()?;
        let ty = stream
            .parse::<Bracketed<Ident>>()
            .ok()
            .map(|bracket| *bracket.0);
        let (mutable, default) = if stream.parse::<Colon>().is_ok() {
            let mutable = stream.parse::<Mut>().is_ok();
            let default = if stream.parse::<Eq>().is_ok() {
                Some(Box::new(stream.parse::<Expr>()?))
            } else {
                None
            };
            (mutable, default)
        } else {
            (false, None)
        };
        Ok(Self {
            name,
            ty,
            mutable,
            default,
        })
    }

    fn could_parse(stream: &mut ParseStream) -> bool {
        Ident::could_parse(stream)
    }
}

#[cfg(test)]
mod test {
    use crate::tokens::stream::TokenStream;

    use super::{ super::stream::ParseStream, FnArg, ExprDecl };

    #[test]
    fn test_simple_untyped() {
        let tokens = TokenStream::from_string("a:= 1 \n b:mut = 2").unwrap();
        let mut stream = ParseStream::new(tokens);
        let decl = stream.parse::<ExprDecl>().unwrap();
        assert_eq!(decl.name.name, "a");
        assert!(decl.value.is_some());
        assert!(decl.ty.is_none());
        assert_eq!(decl.mutable, false);

        let decl = stream.parse::<ExprDecl>().unwrap();
        assert_eq!(decl.name.name, "b");
        assert!(decl.value.is_some());
        assert!(decl.ty.is_none());
        assert_eq!(decl.mutable, true);
    }

    #[test]
    fn test_simple_typed() {
        let tokens = TokenStream::from_string("a[i32]:= 1 \n b[i32]:mut = 2").unwrap();
        let mut stream = ParseStream::new(tokens);
        let decl = stream.parse::<ExprDecl>().unwrap();
        assert_eq!(decl.name.name, "a");
        assert!(decl.value.is_some());
        assert_eq!(decl.mutable, false);
        assert_eq!(decl.ty.unwrap().name, "i32");

        let decl = stream.parse::<ExprDecl>().unwrap();
        assert_eq!(decl.name.name, "b");
        assert!(decl.value.is_some());
        assert_eq!(decl.mutable, true);
        assert_eq!(decl.ty.unwrap().name, "i32");
    }

    #[test]
    fn test_simple_uninit_fails() {
        let tokens = TokenStream::from_string("a[i32] \n b[i32]:mut").unwrap();
        let mut stream = ParseStream::new(tokens);
        let decl = stream.parse::<ExprDecl>();
        assert!(decl.is_err());
    }

    #[test]
    fn test_decl_untyped() {
        let tokens = TokenStream::from_string("a:= 1\n b:mut= 2").unwrap();
        let mut stream = ParseStream::new(tokens);
        let decl = stream.parse::<ExprDecl>().unwrap();
        assert_eq!(decl.name.name, "a");
        assert!(decl.value.is_some());
        assert!(decl.ty.is_none());
        assert_eq!(decl.mutable, false);

        let decl = stream.parse::<ExprDecl>().unwrap();
        assert_eq!(decl.name.name, "b");
        assert!(decl.value.is_some());
        assert!(decl.ty.is_none());
        assert_eq!(decl.mutable, true);
    }

    #[test]
    fn test_decl_typed() {
        let tokens = TokenStream::from_string("a[i32]:= 1 \n b[i32]:mut= 2").unwrap();
        let mut stream = ParseStream::new(tokens);
        let decl = stream.parse::<ExprDecl>().unwrap();
        assert_eq!(decl.name.name, "a");
        assert!(decl.value.is_some());
        assert_eq!(decl.mutable, false);
        assert_eq!(decl.ty.unwrap().name, "i32");

        let decl = stream.parse::<ExprDecl>().unwrap();
        assert_eq!(decl.name.name, "b");
        assert!(decl.value.is_some());
        assert_eq!(decl.mutable, true);
        assert_eq!(decl.ty.unwrap().name, "i32");
    }

    #[test]
    fn test_decl_uninit_fails() {
        let tokens = TokenStream::from_string("a[i32] \n let mut b[i32]").unwrap();
        let mut stream = ParseStream::new(tokens);
        let decl = stream.parse::<ExprDecl>();
        assert!(decl.is_err());
    }

    #[test]
    fn test_fn_arg_simple() {
        let tokens = TokenStream::from_string("a").unwrap();
        let mut stream = ParseStream::new(tokens);
        let arg = stream.parse::<FnArg>().unwrap();
        assert_eq!(arg.name.name, "a");
        assert!(arg.default.is_none());
        assert!(arg.ty.is_none());
        assert_eq!(arg.mutable, false);
    }

    #[test]
    fn test_fn_arg_typed() {
        let tokens = TokenStream::from_string("a[i32]").unwrap();
        let mut stream = ParseStream::new(tokens);
        let arg = stream.parse::<FnArg>().unwrap();
        assert_eq!(arg.name.name, "a");
        assert!(arg.default.is_none());
        assert_eq!(arg.ty.unwrap().name, "i32");
        assert_eq!(arg.mutable, false);
    }

    #[test]
    fn test_fn_arg_typed_mut() {
        let tokens = TokenStream::from_string("a[i32]:mut").unwrap();
        let mut stream = ParseStream::new(tokens);
        let arg = stream.parse::<FnArg>().unwrap();
        assert_eq!(arg.name.name, "a");
        assert!(arg.default.is_none());
        assert_eq!(arg.ty.unwrap().name, "i32");
        assert_eq!(arg.mutable, true);
    }

    #[test]
    fn test_fn_arg_typed_default() {
        let tokens = TokenStream::from_string("a[i32]:= 1").unwrap();
        let mut stream = ParseStream::new(tokens);
        let arg = stream.parse::<FnArg>().unwrap();
        assert_eq!(arg.name.name, "a");
        assert!(arg.default.is_some());
        assert_eq!(arg.ty.unwrap().name, "i32");
        assert_eq!(arg.mutable, false);
    }

    #[test]
    fn test_fn_arg_typed_mut_default() {
        let tokens = TokenStream::from_string("a[i32]:mut = 1").unwrap();
        let mut stream = ParseStream::new(tokens);
        let arg = stream.parse::<FnArg>().unwrap();
        assert_eq!(arg.name.name, "a");
        assert!(arg.default.is_some());
        assert_eq!(arg.ty.unwrap().name, "i32");
        assert_eq!(arg.mutable, true);
    }

    #[test]
    fn test_fn_arg_simple_default() {
        let tokens = TokenStream::from_string("a:= 1").unwrap();
        let mut stream = ParseStream::new(tokens);
        let arg = stream.parse::<FnArg>().unwrap();
        assert_eq!(arg.name.name, "a");
        assert!(arg.default.is_some());
        assert!(arg.ty.is_none());
        assert_eq!(arg.mutable, false);
    }
}
