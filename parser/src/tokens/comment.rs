use crate::file::Cursor;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenComment {
    Line(String),
    Block(String),
    Doc(String),
}

impl TokenComment {
    pub fn try_read(cursor: &mut Cursor) -> Option<Self> {
        if cursor.current() == '#' {
            let mut content = String::new();
            match cursor.next() {
                '!' => {
                    cursor.advance();
                    cursor.advance();
                    while cursor.current() != '!' && cursor.next() != '#' {
                        content.push(cursor.current());
                        cursor.advance();
                    }
                    Some(Self::Doc(content))
                }
                '-' => {
                    cursor.advance();
                    cursor.advance();
                    while cursor.current() != '-' && cursor.next() != '#' {
                        content.push(cursor.current());
                        cursor.advance();
                    }
                    cursor.advance();
                    cursor.advance();
                    Some(Self::Block(content))
                }
                _ => {
                    cursor.advance();
                    while cursor.current() != '\n' {
                        content.push(cursor.current());
                        cursor.advance();
                    }
                    cursor.advance();
                    Some(Self::Line(content))
                }
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use crate::file::Cursor;

    #[test]
    fn test_singleline() {
        let mut cursor = Cursor::new("# Hello, world!\n");
        let comment = super::TokenComment::try_read(&mut cursor).unwrap();
        assert_eq!(comment, super::TokenComment::Line(" Hello, world!".to_string()));
        assert_eq!(cursor.current(), '\0'); //Artifcially added to make sure the current character isn't the #
    }

    #[test]
    pub fn test_multiline() {
        let mut cursor = Cursor::new("#- Hello,\n world! -#");
        let comment = super::TokenComment::try_read(&mut cursor).unwrap();
        assert_eq!(comment, super::TokenComment::Block(" Hello,\n world! ".to_string()));
        assert_eq!(cursor.current(), '\0'); //Artifcially added to make sure the current character isn't the #
    }
}
