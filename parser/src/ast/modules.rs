use crate::{ Span, ast::{ keywords::Mod, grouped::Braced } };

use super::{
    path::ExprPath,
    block::Block,
    Spanned,
    Parse,
    punct::{ Colon, Dot },
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
    pub regular: Vec<Ident>,
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
        let mut regular = vec![];
        while let Ok(ident) = stream.parse::<Ident>() {
            regular.push(ident);
            if stream.parse::<Dot>().is_err() {
                break;
            }
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

#[cfg(test)]
mod test {
    use crate::tokens::stream::TokenStream;

    use super::{ UsePath, super::stream::ParseStream, Module };

    #[test]
    fn test_glob_module() {
        let tokens = TokenStream::from_string("mod test".to_string()).unwrap();
        let mut stream = ParseStream::new(tokens);

        let module = stream.parse::<Module>().unwrap();

        assert!(module.content.is_none());
        assert!(module.path.segments.len() == 1);
    }

    #[test]
    fn test_module() {
        let tokens = TokenStream::from_string("mod test:\n    1".to_string()).unwrap();
        let mut stream = ParseStream::new(tokens);

        let module = stream.parse::<Module>().unwrap();

        assert!(module.content.is_some());
        assert!(module.path.segments.len() == 1);
    }

    #[test]
    fn test_use_path() {
        let tokens = TokenStream::from_string("test.test.test".to_string()).unwrap();
        let mut stream = ParseStream::new(tokens);

        let path = stream.parse::<UsePath>().unwrap();

        assert!(path.regular.len() == 3);
        assert!(path.extract.is_none());
    }

    #[test]
    fn test_simple_extract() {
        let tokens = TokenStream::from_string("test.test { test } ".to_string()).unwrap();
        let mut stream = ParseStream::new(tokens);

        let path = stream.parse::<UsePath>().unwrap();

        assert!(path.regular.len() == 2);
        assert!(path.extract.is_some());
        assert!(path.extract.unwrap().len() == 1);
    }

    #[test]
    fn test_multiple_extract() {
        let tokens = TokenStream::from_string(
            "test.test { test.test { test }, test2 }".to_string()
        ).unwrap();
        let mut stream = ParseStream::new(tokens);

        let path = stream.parse::<UsePath>().unwrap();

        assert!(path.regular.len() == 2);
        assert!(path.extract.is_some());
        assert!(path.extract.unwrap().len() == 2);
    }
}
