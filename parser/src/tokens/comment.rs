use crate::{ file::Cursor, reject_eof };

use super::{ TokenResult, TokenContent };

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenComment {
    Line(String),
    Block(String),
    Doc(String),
}

impl TokenContent for TokenComment {
    fn try_read(cursor: &mut Cursor) -> TokenResult<Self> {
        if cursor.current() == '#' {
            let mut content = String::new();
            Ok(
                Some(match cursor.next() {
                    '!' => {
                        cursor.advance();
                        cursor.advance();
                        while cursor.current() != '!' && cursor.next() != '#' {
                            reject_eof!(cursor);
                            content.push(cursor.current());
                            cursor.advance();
                        }
                        Self::Doc(content)
                    }
                    '-' => {
                        cursor.advance();
                        cursor.advance();
                        while cursor.current() != '-' && cursor.next() != '#' && !cursor.eof() {
                            reject_eof!(cursor);
                            content.push(cursor.current());
                            cursor.advance();
                        }
                        cursor.advance();
                        cursor.advance();
                        Self::Block(content)
                    }
                    _ => {
                        cursor.advance();
                        while cursor.current() != '\n' && !cursor.eof() {
                            reject_eof!(cursor);
                            content.push(cursor.current());
                            cursor.advance();
                        }
                        cursor.advance();
                        Self::Line(content)
                    }
                })
            )
        } else {
            Ok(None)
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{ file::Cursor, tokens::TokenContent };

    #[test]
    fn test_singleline() {
        let mut cursor = Cursor::new("# Hello, world!\n");
        let comment = super::TokenComment::try_read(&mut cursor).unwrap().unwrap();
        assert_eq!(comment, super::TokenComment::Line(" Hello, world!".to_string()));
        assert_eq!(cursor.current(), '\0'); //Artifcially added to make sure the current character isn't the #
    }

    #[test]
    pub fn test_multiline() {
        let mut cursor = Cursor::new("#- Hello,\n world! -#");
        let comment = super::TokenComment::try_read(&mut cursor).unwrap().unwrap();
        assert_eq!(comment, super::TokenComment::Block(" Hello,\n world! ".to_string()));
        assert_eq!(cursor.current(), '\0'); //Artifcially added to make sure the current character isn't the #
    }
}
