use crate::Span;

use super::{
    ident::Ident,
    Spanned,
    Parse,
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
            return Err(super::ParseError::UnexpectedToken {
                expected: "path",
                got: stream.current().clone(),
            });
        }

        Ok(Self {
            segments: segments,
        })
    }

    fn could_parse(stream: &mut ParseStream) -> bool {
        PathSegment::could_parse(stream)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum PathSegment {
    Ident(Ident),
    Generics(Generics),
}

impl Spanned for PathSegment {
    fn span(&self) -> Span {
        match self {
            Self::Ident(ident) => ident.span(),
            Self::Generics( generics) => generics.0.last().unwrap().span(),
        }
    }
}

impl Parse for PathSegment {
    fn parse(stream: &mut ParseStream) -> ParseResult<Self> where Self: Sized {
        if Ident::could_parse(stream) {
            return Ok(Self::Ident(stream.parse::<Ident>()?));
        } else {
            return Ok(Self::Generics(stream.parse::<Generics>()?));
        }
    }

    fn could_parse(stream: &mut ParseStream) -> bool {
        Ident::could_parse(stream) || Generics::could_parse(stream)
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

    fn could_parse(stream: &mut ParseStream) -> bool {
        Lt::could_parse(stream) 
    }
}

#[cfg(test)]
mod test {
    use crate::tokens::stream::TokenStream;

    use super::{ super::stream::ParseStream, ExprPath, PathSegment };

    #[test]
    fn test_simple_path() {
        let tokens = TokenStream::from_string("name.field").unwrap();
        let mut stream = ParseStream::new(tokens);
        let path = stream.parse::<ExprPath>().unwrap();
        assert_eq!(path.segments.len(), 2);
    }

    #[test]
    fn test_path_with_generics() {
        let tokens = TokenStream::from_string("name.<type1, type2>").unwrap();
        let mut stream = ParseStream::new(tokens);
        let path = stream.parse::<ExprPath>().unwrap();
        assert_eq!(path.segments.len(), 1);
        assert!(matches!(path.segments.first().unwrap(), PathSegment::Generics( _)))
    }

    #[test]
    fn test_complex() {
        let tokens = TokenStream::from_string("name.<type1, type2>.field.method<type3>").unwrap();
        let mut stream = ParseStream::new(tokens);
        let path = stream.parse::<ExprPath>().unwrap();
        assert_eq!(path.segments.len(), 3);
        assert!(matches!(path.segments.first().unwrap(), PathSegment::Generics( _)));
        assert!(matches!(path.segments.get(1).unwrap(), PathSegment::Ident(_)));
    }
}
