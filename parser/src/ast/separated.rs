use std::marker::PhantomData;

use crate::tokens::Span;

use super::{punct::{Comma, Punct}, Parse, Spanned, stream::ParseStream, ParseResult};

pub struct Separated<T: Parse, P: Punct = Comma> {
    inner: Vec<T>,
    _puncts: PhantomData<P>,
}

impl<T: Parse, P: Punct> Spanned for Separated<T, P> {
    fn span(&self) -> Span {
        debug_assert!(!self.inner.is_empty());
        self.inner.first().unwrap().span().join(self.inner.last().unwrap().span())
    }
}



impl<T: Parse, P: Punct> Parse for Separated<T, P> {
    fn parse(stream: &mut ParseStream) -> ParseResult<Self> where Self: Sized {
        let mut inner = Vec::new();

        loop {
            let item = stream.parse::<T>()?;
            inner.push(item);

            

            if stream.parse::<P>().is_err() {
                break;
            }
        }

        Ok(Self {
            inner,
            _puncts: PhantomData,
        })
    }
}

#[cfg(test)]
mod test {
    use crate::{
        tokens::stream::TokenStream,
        ast::{Parse, ParseStream, separated::Separated,},
        
    };

    #[test]
    fn test_separated() {
        let tokens = TokenStream::from_string("1, 2, 3, 4".to_string()).unwrap();
        let mut stream = ParseStream::new(tokens);

        let separated = Separated::<super::super::expr::ExprLit>::parse(&mut stream).unwrap();
        assert_eq!(separated.inner.len(), 4);
    }

    #[test]
    fn test_single() {
        let tokens = TokenStream::from_string("1".to_string()).unwrap();
        let mut stream = ParseStream::new(tokens);

        let separated = Separated::<super::super::expr::ExprLit>::parse(&mut stream).unwrap();
        assert_eq!(separated.inner.len(), 1);
    }
}