use crate::{ Span, ast::{ keywords::Mod, grouped::Braced } };

use super::{
    path::ExprPath,
    block::Block,
    Spanned,
    Parse,
    punct::{ Colon, Dot, Star },
    ident::Ident,
    separated::Separated,
    stream::ParseStream,
    ParseResult,
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Module {
    pub path: ExprPath,
    pub content: Option<Block>,
}

impl Spanned for Module {
    fn span(&self) -> Span {
        self.path.span()
    }
}

impl Parse for Module {
    fn parse(stream: &mut ParseStream) -> ParseResult<Self> where Self: Sized {
        stream.parse::<Mod>()?;
        let path = stream.parse::<ExprPath>()?;
        print!("{:#?}", stream.current());
        let content = if stream.parse::<Colon>().is_ok() {
            Some(stream.parse::<Block>()?)
        } else {
            None
        };

        Ok(Self {
            path,
            content,
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct UsePath {
    pub regular: Vec<UsePathSegment>,
    pub extract: Option<Vec<UsePath>>,
}

impl Spanned for UsePath {
    fn span(&self) -> Span {
        let mut span = self.regular[0].span();
        for ident in self.regular.iter().skip(1) {
            span = span.join(ident.span());
        }

        if let Some(extract) = &self.extract {
            span = span.join(extract[0].span());
            for path in extract.iter().skip(1) {
                span = span.join(path.span());
            }
        }

        span
    }
}

impl Parse for UsePath {
    fn parse(stream: &mut ParseStream) -> ParseResult<Self> where Self: Sized {
        let first = stream.parse::<UsePathSegment>()?;
        let mut regular = vec![first];
        while stream.parse::<Dot>().is_ok() {
            regular.push(stream.parse::<UsePathSegment>()?);
        }
        println!("{:#?}", stream.current());
        let extract = stream
            .parse::<Braced<Separated<UsePath>>>()
            .ok()

            .map(|bracketed| {
                println!("{:#?}", bracketed.0);
                bracketed.0.into_iter().collect()
            });

        Ok(Self {
            regular,
            extract,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UsePathSegment {
    Item(Ident),
    All(Span),
}

impl Spanned for UsePathSegment {
    fn span(&self) -> Span {
        match self {
            UsePathSegment::Item(item) => item.span(),
            UsePathSegment::All(span) => *span,
        }
    }
}

impl Parse for UsePathSegment {
    fn parse(stream: &mut ParseStream) -> ParseResult<Self> where Self: Sized {
        if let Ok(name) = stream.parse() {
            Ok(Self::Item(name))
        } else if let Ok(all) = stream.parse::<Star>() {
            Ok(Self::All(all.span()))
        } else {
            Err(super::ParseError::UnexpectedToken {
                expected: "<module pat/wildcard>",
                got: stream.current().clone(),
            })
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{ tokens::stream::TokenStream, ast::stream };

    use super::{ UsePath, super::stream::ParseStream, Module };

    #[test]
    fn test_glob_module() {
        let tokens = TokenStream::from_string("mod test").unwrap();
        let mut stream = ParseStream::new(tokens);

        let module = stream.parse::<Module>().unwrap();

        assert!(module.content.is_none());
        assert!(module.path.segments.len() == 1);
    }

    #[test]
    fn test_module() {
        let tokens = TokenStream::from_string("mod test:\n    1").unwrap();
        let mut stream = ParseStream::new(tokens);

        let module = stream.parse::<Module>().unwrap();

        assert!(module.content.is_some());
        assert!(module.path.segments.len() == 1);
    }

    #[test]
    fn test_use_path() {
        let tokens = TokenStream::from_string("test.test.test").unwrap();
        let mut stream = ParseStream::new(tokens);

        let path = stream.parse::<UsePath>().unwrap();

        assert!(path.regular.len() == 3);
        assert!(path.extract.is_none());
    }

    #[test]
    fn test_simple_extract() {
        let tokens = TokenStream::from_string("test.test { test } ").unwrap();
        let mut stream = ParseStream::new(tokens);

        let path = stream.parse::<UsePath>().unwrap();

        assert!(path.regular.len() == 2);
        assert!(path.extract.is_some());
        assert!(path.extract.unwrap().len() == 1);
    }

    #[test]
    fn test_multiple_extract() {
        let tokens = TokenStream::from_string("test.test { test.test { test }, test2 }").unwrap();
        let mut stream = ParseStream::new(tokens);

        let path = stream.parse::<UsePath>().unwrap();

        assert!(path.regular.len() == 2);
        assert!(path.extract.is_some());
        assert!(path.extract.unwrap().len() == 2);
    }

    #[test]
    fn test_simple_wildcard() {
        let tokens = TokenStream::from_string("test.test.*").unwrap();
        let mut stream = ParseStream::new(tokens);

        let path = stream.parse::<UsePath>().unwrap();

        assert!(path.regular.len() == 3);
        assert!(path.extract.is_none());
    }

    #[test]
    fn test_wildcard_extract() {
        let tokens = TokenStream::from_string("test.test { *, test.one } ").unwrap();
        let mut stream = ParseStream::new(tokens);

        let path = stream.parse::<UsePath>().unwrap();

        assert!(path.regular.len() == 2);
        assert!(path.extract.is_some());
        assert!(path.extract.unwrap().len() == 2);
    }
}
