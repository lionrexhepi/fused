use crate::file::Cursor;

use super::Token;


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Delim {
    Paren,
    Bracket,
    Brace,
    Angle,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TokenGroup {
    pub delim: Delim,
    pub tokens: Vec<Token>,
}

impl TokenGroup {
    pub fn try_read(cursor: &mut Cursor) -> Option<Self> {
        let (delim, close) = if cursor.current() == '(' {
            (Delim::Paren, ')')
        } else if cursor.current() == '[' {
            (Delim::Bracket, ']')
        } else if cursor.current() == '{' {
            (Delim::Brace, '}')
        } else if cursor.current() == '<' {
            (Delim::Angle,'>')
        } else {
            return None;
        };

        cursor.advance();

        let mut tokens = Vec::new();

        while cursor.current() != close && !cursor.eof() {
            tokens.push(Token::try_read(cursor)?);
        }
        if cursor.eof() {
            return None;
        } else {
        cursor.advance();
        
        Some(Self { delim, tokens })
        }
    }
}

#[cfg(test)]
mod test{
    use crate::{file::Cursor, tokens::{group::{TokenGroup, Delim}, Token, ident::TokenIdent, TokenType}};

    #[test]
    fn test_paren() {
        let mut cursor = Cursor::new("(test)");
        let group = TokenGroup::try_read(&mut cursor).unwrap();
        assert_eq!(group.delim, Delim::Paren);
        assert_eq!(group.tokens.len(), 1);
        assert_eq!(group.tokens.first().unwrap().content, TokenType::Ident(TokenIdent::new("test".to_string())));
    }

    #[test]
    fn test_bracket() {
        let mut cursor = Cursor::new("[test]");
        let group = TokenGroup::try_read(&mut cursor).unwrap();
        assert_eq!(group.delim, Delim::Bracket);
        assert_eq!(group.tokens.len(), 1);
        assert_eq!(group.tokens.first().unwrap().content, TokenType::Ident(TokenIdent::new("test".to_string())));
    }

    #[test]
    fn test_braces() {
        let mut cursor = Cursor::new("{test}");
        let group = TokenGroup::try_read(&mut cursor).unwrap();
        assert_eq!(group.delim, Delim::Brace);
        assert_eq!(group.tokens.len(), 1);
        assert_eq!(group.tokens.first().unwrap().content, TokenType::Ident(TokenIdent::new("test".to_string())));
    }

    #[test]
    fn test_angle() {
        let mut cursor = Cursor::new("<test>");
        let group = TokenGroup::try_read(&mut cursor).unwrap();
        assert_eq!(group.delim, Delim::Angle);
        assert_eq!(group.tokens.len(), 1);
        assert_eq!(group.tokens.first().unwrap().content, TokenType::Ident(TokenIdent::new("test".to_string())));
    }

    #[test]
    fn test_nonterminated() {
        let mut cursor = Cursor::new("(test");
        let group = TokenGroup::try_read(&mut cursor);
        assert_eq!(group, None);
    }
}