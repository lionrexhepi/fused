use crate::file::Cursor;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TokenIdent {
    pub name: String,
    pub escaped: bool,
}

impl TokenIdent {
    pub const fn new(name: String) -> Self {
        Self { name, escaped: false }
    }

    pub fn try_read(cursor: &mut Cursor) -> Option<Self> {
        let escaped = if cursor.current() == '@' {
            cursor.advance();
            true
        } else {
            false
        };
        if is_valid_ident_start(cursor.current()) {
            let mut name = String::new();

            while is_valid_ident_char(cursor.current()) {
                name.push(cursor.current());
                cursor.advance();
            }

            Some(Self { name, escaped })
        } else {
            print!("bad: {}", cursor.current());
            None
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
    use crate::file::Cursor;
    use super::TokenIdent;

    #[test]
    fn test_ident() {
        let mut cursor = Cursor::new("test");
        let ident = TokenIdent::try_read(&mut cursor).unwrap();
        assert_eq!(ident.name, "test");
        assert_eq!(ident.escaped, false);
    }

    #[test]
    fn test_escaped_ident() {
        let mut cursor = Cursor::new("@test");
        let ident = TokenIdent::try_read(&mut cursor).unwrap();
        assert_eq!(ident.name, "test");
        assert_eq!(ident.escaped, true);
    }

    #[test]
    fn test_nonvalid_start() {
        let mut cursor = Cursor::new("1test");
        let ident = TokenIdent::try_read(&mut cursor);
        assert!(ident.is_none());
    }

    #[test]
    fn test_nonvalid_char() {
        let mut cursor = Cursor::new("te+t");
        let ident = TokenIdent::try_read(&mut cursor).unwrap();
        assert_eq!(ident.name, "te");
        assert_eq!(ident.escaped, false);

        assert_eq!(cursor.current(), '+');
        cursor.advance();

        let next_ident = TokenIdent::try_read(&mut cursor).unwrap();
        assert_eq!(next_ident.name, "t");
        assert_eq!(next_ident.escaped, false);
    }
}
