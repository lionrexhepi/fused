use crate::{ file::SourceCursor, reject_eof };

use super::{ TokenContent, TokenResult, TokenError };

const fn is_digit(char: char) -> bool {
    matches!(char, '0'..='9')
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenLiteral {
    String(LiteralString),
    Number(LiteralNumber),
}

impl TokenContent for TokenLiteral {
    fn try_read(cursor: &mut SourceCursor) -> TokenResult<Self> {
        Ok(
            if let Some(number) = LiteralNumber::try_read(cursor)? {
                Some(Self::Number(number))
            } else if let Some(string) = LiteralString::try_read(cursor)? {
                Some(Self::String(string))
            } else {
                None
            }
        )
    }
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct LiteralNumber {
    pub r#type: NumberType,
    pub digits: String,
}

impl TokenContent for LiteralNumber {
    fn try_read(cursor: &mut SourceCursor) -> TokenResult<Self> {
        if is_digit(cursor.current()) {
            let mut num_type = NumberType::Decimal;
            let digits = if cursor.current() == '0' && matches!(cursor.next(), 'x' | 'b') {
                cursor.advance();
                match cursor.current() {
                    'x' => {
                        num_type = NumberType::Hexadecimal;
                        cursor.advance();
                        read_hexadecimal(cursor)?
                    }
                    'b' => {
                        num_type = NumberType::Binary;
                        cursor.advance();
                        read_binary(cursor)?
                    }
                    _ => unreachable!(),
                }
            } else {
                //TODO:
                //Since we checked that the first character is a digit, we can assume that the number has at least one valid digit.
                //Therefore, an EOF wouldn't really matter. So does it really make sense to have read_decimal return a Result?
                read_decimal(cursor)?
            };

            Ok(
                Some(LiteralNumber {
                    r#type: num_type,
                    digits,
                })
            )
        } else {
            Ok(None)
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NumberType {
    Decimal,
    Hexadecimal,
    Binary,
}

fn read_decimal(cursor: &mut SourceCursor) -> Result<String, TokenError> {
    let mut number = String::new();
    let mut decimal = false;

    loop {
        let current = cursor.current();
        if matches!(current, '0'..='9') {
            //Digit
            number.push(current);
        } else if current == '.' && !decimal {
            //Decimal point
            decimal = true;
            number.push(current);
        } else if current != '_' {
            //Underscores in number literals are ignored; Otherwise, break
            break;
        }

        cursor.advance();
    }

    Ok(number)
}

fn read_binary(cursor: &mut SourceCursor) -> Result<String, TokenError> {
    let mut number = String::new();

    //Read all digits
    while matches!(cursor.current(), '0' | '1') {
        reject_eof!(cursor);
        number.push(cursor.current());
        cursor.advance();
    }

    if number.is_empty() {
        if cursor.eof() {
            Err(TokenError::UnexpectedEof)
        } else {
            Err(TokenError::InvalidChar(cursor.current()))
        }
    } else {
        Ok(number)
    }
}

fn read_hexadecimal(cursor: &mut SourceCursor<'_>) -> Result<String, TokenError> {
    let mut number = String::new();

    while matches!(cursor.current(), '0'..='9' | 'a'..='f' | 'A'..='F') {
        reject_eof!(cursor);
        number.push(cursor.current());
        cursor.advance();
    }

    if number.is_empty() {
        if cursor.eof() {
            Err(TokenError::UnexpectedEof)
        } else {
            Err(TokenError::InvalidChar(cursor.current()))
        }
    } else {
        Ok(number)
    }
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct LiteralString {
    pub r#type: StringType,
    pub content: String,
}

impl TokenContent for LiteralString {
    fn try_read(cursor: &mut SourceCursor) -> TokenResult<Self> {
        if cursor.current() == '"' {
            cursor.advance();

            let content = read_string(cursor, 1)?;

            Ok(
                Some(Self {
                    r#type: StringType::Regular,
                    content,
                })
            )
        } else if cursor.current() == '@' && cursor.next() == '"' {
            cursor.advance();
            let mut quotes = 0;
            while cursor.current() == '"' {
                quotes += 1;
                cursor.advance();
            }
            let content = read_string(cursor, quotes)?;

            Ok(Some(Self { r#type: StringType::Raw(quotes), content }))
        } else {
            Ok(None)
        }
    }
}

fn read_string(cursor: &mut SourceCursor, quotes: usize) -> Result<String, TokenError> {
    let mut content = String::new();

    loop {
        reject_eof!(cursor);
        let current = cursor.current();
        if current == '"' {
            let mut quotes_found = 0;
            while cursor.current() == '"' {
                quotes_found += 1;
                cursor.advance();
            }

            if quotes_found == quotes {
                break;
            } else {
                content.push(current);
            }
        } else if current == '\\' {
            todo!("Escape sequences");
        } else if current == '{' {
            todo!("Format args");
        } else {
            content.push(current);
            cursor.advance();
        }
    }
    Ok(content)
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum StringType {
    Regular,
    Raw(usize),
}

#[cfg(test)]
mod test {
    use crate::{
        file::SourceCursor,
        tokens::{ literal::{ LiteralNumber, NumberType, LiteralString, StringType }, TokenContent },
    };

    #[test]
    fn test_integer() {
        let mut cursor = SourceCursor::new("123");
        let number = LiteralNumber::try_read(&mut cursor).unwrap().unwrap();
        assert_eq!(number.r#type, NumberType::Decimal);
        assert_eq!(number.digits, "123".to_string());
    }

    #[test]
    fn test_decimals() {
        let mut cursor = SourceCursor::new("123.456");
        let number = LiteralNumber::try_read(&mut cursor).unwrap().unwrap();
        assert_eq!(number.r#type, NumberType::Decimal);
        assert_eq!(number.digits, "123.456".to_string());
    }

    #[test]
    fn test_binary() {
        let mut cursor = SourceCursor::new("0b1010");
        let number = LiteralNumber::try_read(&mut cursor).unwrap().unwrap();
        assert_eq!(number.r#type, NumberType::Binary);
        assert_eq!(number.digits, "1010".to_string());
    }

    #[test]
    fn test_hexadecimal() {
        let mut cursor = SourceCursor::new("0x123abc");
        let number = LiteralNumber::try_read(&mut cursor).unwrap().unwrap();
        assert_eq!(number.r#type, NumberType::Hexadecimal);
        assert_eq!(number.digits, "123abc".to_string());
    }

    #[test]
    fn test_string() {
        let mut cursor = SourceCursor::new("\"Hello, world!\"");

        let string = LiteralString::try_read(&mut cursor).unwrap().unwrap();
        assert_eq!(string.r#type, StringType::Regular);
        assert_eq!(string.content, "Hello, world!".to_string());
    }

    #[test]
    fn escaped_string() {
        for n in 1..5 {
            let test_str = format!("@{q}Hello, world!{q}", q = "\"".repeat(n));
            let mut cursor = SourceCursor::new(&test_str);
            let string = LiteralString::try_read(&mut cursor).unwrap().unwrap();
            assert_eq!(string.r#type, StringType::Raw(n));
            assert_eq!(string.content, "Hello, world!".to_string());
        }
    }

    #[test]
    fn contains_quotes() {
        let test_string = r#" @""Hello,"World" """#.to_string();
        let mut cursor = SourceCursor::new(&test_string);
        cursor.advance(); //Skip the whitespace
        let string = LiteralString::try_read(&mut cursor).unwrap().unwrap();
        assert_eq!(string.r#type, StringType::Raw(2));
        assert_eq!(string.content, r#"Hello,"World" "#.to_string());
    }
}
