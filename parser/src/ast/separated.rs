use std::marker::PhantomData;

use crate::Span;

use super::{ punct::{ Comma, Punct }, Parse, Spanned, stream::ParseStream, ParseResult };

#[derive(Debug, PartialEq, Eq, Clone)]
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

impl<T: Parse, P: Punct> IntoIterator for Separated<T, P> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

impl<T: Parse, P: Punct> Separated<T, P> {
    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.inner.iter()
    }
}

impl<T: Parse, P: Punct> Parse for Separated<T, P> {
    fn parse(stream: &mut ParseStream) -> ParseResult<Self> where Self: Sized {
        let mut inner = Vec::new();

        while let Ok(item) = stream.parse::<T>() {
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
    use crate::{ tokens::stream::TokenStream, ast::{ Parse, ParseStream, separated::Separated } };

    use super::super::expr::ExprLit;

    #[test]
    fn test_separated() {
        let tokens = TokenStream::from_string("1, 2, 3, 4".to_string()).unwrap();
        let mut stream = ParseStream::new(tokens);

        let separated = Separated::<ExprLit>::parse(&mut stream).unwrap();
        assert_eq!(separated.inner.len(), 4);
    }

    #[test]
    fn test_single() {
        let tokens = TokenStream::from_string("1".to_string()).unwrap();
        let mut stream = ParseStream::new(tokens);

        let separated = Separated::<ExprLit>::parse(&mut stream).unwrap();
        assert_eq!(separated.inner.len(), 1);
    }
}
