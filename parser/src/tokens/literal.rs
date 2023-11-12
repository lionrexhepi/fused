use core::num;
use std::{ char, thread::current };

use crate::file::Cursor;

const fn is_digit(char: char) -> bool {
    matches!(char, '0'..='9')
}

pub enum TokenLiteral {
    String(LiteralString),
    Number(LiteralNumber),
}

impl TokenLiteral {
    pub fn try_read(cursor: &mut Cursor) -> Option<Self> {
        if let Some(number) = LiteralNumber::try_read(cursor) {
            Some(Self::Number(number))
        } else if let Some(string) = LiteralString::try_read(cursor) {
            Some(Self::String(string))
        } else {
            None
        }
    }
}

pub struct LiteralNumber {
    r#type: NumberType,
    digits: Option<String>,
}

impl LiteralNumber {
    pub fn try_read(cursor: &mut Cursor) -> Option<Self> {
        if is_digit(cursor.current()) {
            let mut num_type = NumberType::Decimal;
            let digits = if cursor.current() == '0' && matches!(cursor.next(), 'x' | 'b') {
                cursor.advance();
                match cursor.current() {
                    'x' => {
                        num_type = NumberType::Hexadecimal;
                        cursor.advance();
                        read_hexadecimal(cursor)
                    }
                    'b' => {
                        num_type = NumberType::Binary;
                        cursor.advance();
                        read_binary(cursor)
                    }
                    _ => unreachable!(),
                }
            } else {
                //Since we checked that the first character is a digit, we can assume that the number has at least one valid digit.
                Some(read_decimal(cursor))
            };

            Some(LiteralNumber {
                r#type: num_type,
                digits,
            })
        } else {
            None
        }
    }
}

pub enum NumberType {
    Decimal,
    Hexadecimal,
    Binary,
}

fn read_decimal(cursor: &mut Cursor) -> String {
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

    number
}

fn read_binary(cursor: &mut Cursor) -> Option<String> {
    let mut number = String::new();

    //Read all digits
    while matches!(cursor.current(), '0' | '1') {
        number.push(cursor.current());
        cursor.advance();
    }

    if number.is_empty() {
        None
    } else {
        Some(number)
    }
}

fn read_hexadecimal(cursor: &mut Cursor<'_>) -> Option<String> {
    let mut number = String::new();

    while matches!(cursor.current(), '0'..='9' | 'a'..='f' | 'A'..='F') {
        number.push(cursor.current());
        cursor.advance();
    }

    if number.is_empty() {
        None
    } else {
        Some(number)
    }
}

pub struct LiteralString {
    r#type: StringType,
    content: String,
}

impl LiteralString {
    pub fn try_read(cursor: &mut Cursor) -> Option<Self> {
        if cursor.current() == '"' {
            cursor.advance();
            let content = read_string(cursor, 1);

            Some(Self {
                r#type: StringType::Regular,
                content,
            })
        } else if cursor.current() == 'r' && cursor.next() == '"' {
            cursor.advance();
            let mut quotes = 1;
            while cursor.current() == '"' {
                quotes += 1;
                cursor.advance();
            }
            let content = read_string(cursor, quotes);

            Some(Self { r#type: StringType::Raw(quotes), content })
        } else {
            None
        }
    }
}

fn read_string(cursor: &mut Cursor, quotes: usize) -> String {
    let mut content = String::new();

    loop {
        let current = cursor.current();
        if current == '"' {
            let mut quotes_found = 1;
            cursor.advance();
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
    todo!()
}

pub enum StringType {
    Regular,
    Raw(usize),
}
