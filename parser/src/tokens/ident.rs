use crate::{ file::SourceCursor, reject_eof };

use super::{ TokenResult, TokenContent };

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TokenIdent {
    pub name: String,
    pub escaped: bool,
}

impl TokenIdent {
    pub fn new(name: impl ToString) -> Self {
        Self { name: name.to_string(), escaped: false }
    }
}

impl TokenContent for TokenIdent {
    fn try_read(cursor: &mut SourceCursor) -> TokenResult<Self> {
        let escaped = if cursor.current() == '@' && is_valid_ident_char(cursor.next()) {
            cursor.advance();
            true
        } else {
            false
        };
        if escaped || is_valid_ident_start(cursor.current()) {
            let mut name = String::new();

            while is_valid_ident_char(cursor.current()) {
                reject_eof!(cursor);
                name.push(cursor.current());
                cursor.advance();
            }

            Ok(Some(Self { name, escaped }))
        } else {
            Ok(None)
        }
    }
}

pub(crate) fn is_valid_ident_start(c: char) -> bool {
    matches!(c, 'a'..='z' | 'A'..='Z' | '_')
}

pub(crate) fn is_valid_ident_char(c: char) -> bool {
    matches!(c, 'a'..='z' | 'A'..='Z' | '_' | '0'..='9')
}

#[cfg(test)]
pub(crate) mod test {
    use crate::{ file::SourceCursor, tokens::TokenContent };
    use super::TokenIdent;

    #[test]
    fn test_ident() {
        let mut cursor = SourceCursor::new("test");
        let ident = TokenIdent::try_read(&mut cursor).unwrap().unwrap();
        assert_eq!(ident.name, "test");
        assert_eq!(ident.escaped, false);
    }

    #[test]
    fn test_escaped_ident() {
        let mut cursor = SourceCursor::new("@test");
        let ident = TokenIdent::try_read(&mut cursor).unwrap().unwrap();
        assert_eq!(ident.name, "test");
        assert_eq!(ident.escaped, true);
    }

    #[test]
    fn test_nonvalid_start() {
        let mut cursor = SourceCursor::new("1test");
        let ident = TokenIdent::try_read(&mut cursor).unwrap();
        assert!(ident.is_none());
    }

    #[test]
    fn test_nonvalid_char() {
        let mut cursor = SourceCursor::new("te+t");
        let ident = TokenIdent::try_read(&mut cursor).unwrap().unwrap();
        assert_eq!(ident.name, "te");
        assert_eq!(ident.escaped, false);

        assert_eq!(cursor.current(), '+');
        cursor.advance();

        let next_ident = TokenIdent::try_read(&mut cursor).unwrap().unwrap();
        assert_eq!(next_ident.name, "t");
        assert_eq!(next_ident.escaped, false);
    }
}
