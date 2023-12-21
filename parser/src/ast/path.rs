use crate::tokens::Span;

use super::{
    ident::Ident,
    expr::Expr,
    Spanned,
    Parse,
    grouped::Parenthesized,
    separated::Separated,
    punct::{ Lt, Gt },
    stream::ParseStream,
    ParseResult,
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ExprPath {
    pub segments: Vec<PathSegment>,
}

impl Spanned for ExprPath {
    fn span(&self) -> Span {
        self.segments.first().unwrap().span().join(self.segments.last().unwrap().span())
    }
}

impl Parse for ExprPath {
    fn parse(stream: &mut ParseStream) -> ParseResult<Self> where Self: Sized {
        let segments: Vec<_> = stream
            .parse::<Separated<PathSegment, super::punct::Dot>>()?
            .into_iter()
            .collect();

        if segments.is_empty() {
            return Err(super::ParseError::UnexpectedToken("path", stream.current().clone()));
        }

        Ok(Self {
            segments: segments,
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum PathSegment {
    Ident(Ident),
    Generics(Ident, Generics),
    Call(Ident, Generics, Vec<Expr>),
}

impl Spanned for PathSegment {
    fn span(&self) -> Span {
        match self {
            Self::Ident(ident) => ident.span(),
            Self::Generics(ident, generics) => ident.span().join(generics.0.last().unwrap().span()),
            Self::Call(ident, _, args) => ident.span().join(args.last().unwrap().span()),
        }
    }
}

impl Parse for PathSegment {
    fn parse(stream: &mut ParseStream) -> ParseResult<Self> where Self: Sized {
        let ident = stream.parse::<Ident>()?;
        print!("{:#?}", stream.current());
        let generics = stream.parse::<Generics>().unwrap_or_default();

        if let Some(args) = stream.parse::<Parenthesized<Separated<Expr>>>().ok() {
            Ok(Self::Call(ident, generics, args.0.into_iter().collect()))
        } else if !generics.0.is_empty() {
            Ok(Self::Generics(ident, generics))
        } else {
            Ok(Self::Ident(ident))
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct Generics(Vec<ExprPath>);

impl Spanned for Generics {
    fn span(&self) -> Span {
        if !self.0.is_empty() {
            self.0.first().unwrap().span().join(self.0.last().unwrap().span())
        } else {
            Span::default()
        }
    }
}

impl Parse for Generics {
    fn parse(stream: &mut ParseStream) -> ParseResult<Self> where Self: Sized {
        stream.parse::<Lt>()?;
        let mut split = stream.clone();
        let generics = split.parse::<Separated<ExprPath>>()?;
        split.parse::<Gt>()?;
        *stream = split;
        Ok(Self(generics.into_iter().collect()))
    }
}

#[cfg(test)]
mod test {
    use crate::tokens::stream::TokenStream;

    use super::{ super::stream::ParseStream, ExprPath, PathSegment };

    #[test]
    fn test_simple_path() {
        let tokens = TokenStream::from_string("name.field".to_string()).unwrap();
        let mut stream = ParseStream::new(tokens);
        let path = stream.parse::<ExprPath>().unwrap();
        assert_eq!(path.segments.len(), 2);
    }

    #[test]
    fn test_path_with_generics() {
        let tokens = TokenStream::from_string("name<type1, type2>".to_string()).unwrap();
        let mut stream = ParseStream::new(tokens);
        let path = stream.parse::<ExprPath>().unwrap();
        assert_eq!(path.segments.len(), 1);
        assert!(matches!(path.segments.first().unwrap(), PathSegment::Generics(_, _)))
    }

    #[test]
    fn test_call() {
        let tokens = TokenStream::from_string("name()".to_string()).unwrap();
        let mut stream = ParseStream::new(tokens);
        let path = stream.parse::<ExprPath>().unwrap();
        assert_eq!(path.segments.len(), 1);
        assert!(matches!(path.segments.first().unwrap(), PathSegment::Call(_, _, _)))
    }

    #[test]
    fn test_call_with_generics() {
        let tokens = TokenStream::from_string("name<type>()".to_string()).unwrap();
        let mut stream = ParseStream::new(tokens);
        let path = stream.parse::<ExprPath>().unwrap();
        assert_eq!(path.segments.len(), 1);
        assert!(matches!(path.segments.first().unwrap(), PathSegment::Call(_, _, _)))
    }

    #[test]
    fn test_complex() {
        let tokens = TokenStream::from_string(
            "name<type1, type2>.field.method<type3>()".to_string()
        ).unwrap();
        let mut stream = ParseStream::new(tokens);
        let path = stream.parse::<ExprPath>().unwrap();
        assert_eq!(path.segments.len(), 3);
        assert!(matches!(path.segments.first().unwrap(), PathSegment::Generics(_, _)));
        assert!(matches!(path.segments.get(1).unwrap(), PathSegment::Ident(_)));
        assert!(matches!(path.segments.last().unwrap(), PathSegment::Call(_, _, _)));
    }
}
