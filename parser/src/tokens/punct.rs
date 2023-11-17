use crate::file::Cursor;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenPunct {
    Plus, //+
    Minus, //-
    Star, //*
    Slash, // /
    Percent, // %
    Caret, // ^
    Ampersand, // &
    Pipe, // |
    DoublePipe, // ||
    PlusEq, // +=
    MinusEq, // -=
    StarEq, // *=
    SlashEq, // /=
    PercentEq, // %=
    CaretEq, // ^=
    AmpersandEq, // &=
    PipeEq, // |=
    Tilde, // ~
    Question, // ?
    Exclamation, // !
    Dot, // .
    Comma, // ,
    Colon, // :
    SemiColon, // ;
    Eq, // =
    DoubleEq, // ==
    NotEq, // !=
    Lt, // <
    LeftShift, // <<
    LeftShiftEq, // <<=
    LtEq, // <=
    Gt, // >
    RightShift, // >>
    RightShiftEq, // >>=
    GtEq, // >=
    Arrow, // ->
    FatArrow, // =>
    Dollar, // $
    Backslash, // \
    DoubleAmpersand, // &&
}

impl TokenPunct {
    pub fn try_read(cursor: &mut Cursor) -> Option<Self> {
        let result = match cursor.current() {
            '+' => {
                if cursor.next() == '=' {
                    cursor.advance();
                    Some(Self::PlusEq)
                } else {
                    Some(Self::Plus)
                }
            }
            '-' => {
                if cursor.next() == '=' {
                    cursor.advance();
                    Some(Self::MinusEq)
                } else if cursor.next() == '>' {
                    cursor.advance();
                    Some(Self::Arrow)
                } else {
                    Some(Self::Minus)
                }
            }
            '*' => {
                if cursor.next() == '=' {
                    cursor.advance();
                    Some(Self::StarEq)
                } else {
                    Some(Self::Star)
                }
            }
            '/' => {
                if cursor.next() == '=' {
                    cursor.advance();
                    Some(Self::SlashEq)
                } else {
                    Some(Self::Slash)
                }
            }
            '%' => {
                if cursor.next() == '=' {
                    cursor.advance();
                    Some(Self::PercentEq)
                } else {
                    Some(Self::Percent)
                }
            }
            '^' => {
                if cursor.next() == '=' {
                    cursor.advance();
                    Some(Self::CaretEq)
                } else {
                    Some(Self::Caret)
                }
            }
            '&' => {
                if cursor.next() == '=' {
                    cursor.advance();
                    Some(Self::AmpersandEq)
                } else if cursor.next() == '&' {
                    cursor.advance();
                    Some(Self::DoubleAmpersand)
                } else {
                    Some(Self::Ampersand)
                }
            }
            '|' => {
                if cursor.next() == '=' {
                    cursor.advance();
                    Some(Self::PipeEq)
                } else if cursor.next() == '|' {
                    cursor.advance();
                    Some(Self::DoublePipe)
                } else {
                    Some(Self::Pipe)
                }
            }
            '!' => {
                if cursor.next() == '=' {
                    cursor.advance();
                    Some(Self::NotEq)
                } else {
                    Some(Self::Exclamation)
                }
            }
            '=' => {
                if cursor.next() == '=' {
                    cursor.advance();
                    Some(Self::DoubleEq)
                } else if cursor.next() == '>' {
                    cursor.advance();
                    Some(Self::FatArrow)
                } else {
                    Some(Self::Eq)
                }
            }
            '<' => {
                if cursor.next() == '=' {
                    cursor.advance();
                    Some(Self::LtEq)
                } else if cursor.next() == '<' {
                    cursor.advance();
                    if cursor.next() == '=' {
                        cursor.advance();
                        Some(Self::LeftShiftEq)
                    } else {
                        Some(Self::LeftShift)
                    }
                } else {
                    Some(Self::Lt)
                }
            }
            '>' => {
                if cursor.next() == '=' {
                    cursor.advance();
                    Some(Self::GtEq)
                } else if cursor.next() == '>' {
                    cursor.advance();
                    if cursor.next() == '=' {
                        cursor.advance();
                        Some(Self::RightShiftEq)
                    } else {
                        Some(Self::RightShift)
                    }
                } else {
                    Some(Self::Gt)
                }
            }
            '.' => Some(Self::Dot),
            ',' => Some(Self::Comma),
            ':' => Some(Self::Colon),
            ';' => Some(Self::SemiColon),

            '~' => Some(Self::Tilde),

            '?' => Some(Self::Question),
            '$' => Some(Self::Dollar),
            '\\' => Some(Self::Backslash),

            _ => None,
        };

        if result.is_some() {
            cursor.advance();
        }

        result
    }
}

#[cfg(test)]
mod test {
    use crate::{ file::Cursor, tokens::punct::TokenPunct };

    #[test]
    fn test_single_puncts() {
        let mut cursor = Cursor::new("+ - * / % ^ & | ~ ? $ . \\ < > ! ; : ,");

        assert_eq!(TokenPunct::try_read(&mut cursor), Some(TokenPunct::Plus));
        cursor.advance();
        assert_eq!(TokenPunct::try_read(&mut cursor), Some(TokenPunct::Minus));
        cursor.advance();
        assert_eq!(TokenPunct::try_read(&mut cursor), Some(TokenPunct::Star));
        cursor.advance();
        assert_eq!(TokenPunct::try_read(&mut cursor), Some(TokenPunct::Slash));
        cursor.advance();
        assert_eq!(TokenPunct::try_read(&mut cursor), Some(TokenPunct::Percent));
        cursor.advance();
        assert_eq!(TokenPunct::try_read(&mut cursor), Some(TokenPunct::Caret));
        cursor.advance();
        assert_eq!(TokenPunct::try_read(&mut cursor), Some(TokenPunct::Ampersand));
        cursor.advance();
        assert_eq!(TokenPunct::try_read(&mut cursor), Some(TokenPunct::Pipe));
        cursor.advance();
        assert_eq!(TokenPunct::try_read(&mut cursor), Some(TokenPunct::Tilde));
        cursor.advance();
        assert_eq!(TokenPunct::try_read(&mut cursor), Some(TokenPunct::Question));
        cursor.advance();
        assert_eq!(TokenPunct::try_read(&mut cursor), Some(TokenPunct::Dollar));
        cursor.advance();
        assert_eq!(TokenPunct::try_read(&mut cursor), Some(TokenPunct::Dot));
        cursor.advance();
        assert_eq!(TokenPunct::try_read(&mut cursor), Some(TokenPunct::Backslash));
        cursor.advance();
        assert_eq!(TokenPunct::try_read(&mut cursor), Some(TokenPunct::Lt));
        cursor.advance();
        assert_eq!(TokenPunct::try_read(&mut cursor), Some(TokenPunct::Gt));
        cursor.advance();
        assert_eq!(TokenPunct::try_read(&mut cursor), Some(TokenPunct::Exclamation));
        cursor.advance();
        assert_eq!(TokenPunct::try_read(&mut cursor), Some(TokenPunct::SemiColon));
        cursor.advance();
        assert_eq!(TokenPunct::try_read(&mut cursor), Some(TokenPunct::Colon));
        cursor.advance();
        assert_eq!(TokenPunct::try_read(&mut cursor), Some(TokenPunct::Comma));
    }

    #[test]
    fn test_complex() {
        let mut cursor = Cursor::new("+= -= *= /= ^= |= <= <<= || && == >>= != => ->");

        assert_eq!(TokenPunct::try_read(&mut cursor), Some(TokenPunct::PlusEq));
        cursor.advance();
        assert_eq!(TokenPunct::try_read(&mut cursor), Some(TokenPunct::MinusEq));
        cursor.advance();
        assert_eq!(TokenPunct::try_read(&mut cursor), Some(TokenPunct::StarEq));
        cursor.advance();
        assert_eq!(TokenPunct::try_read(&mut cursor), Some(TokenPunct::SlashEq));
        cursor.advance();
        assert_eq!(TokenPunct::try_read(&mut cursor), Some(TokenPunct::CaretEq));
        cursor.advance();
        assert_eq!(TokenPunct::try_read(&mut cursor), Some(TokenPunct::PipeEq));
        cursor.advance();
        assert_eq!(TokenPunct::try_read(&mut cursor), Some(TokenPunct::LtEq));
        cursor.advance();
        assert_eq!(TokenPunct::try_read(&mut cursor), Some(TokenPunct::LeftShiftEq));
        cursor.advance();
        assert_eq!(TokenPunct::try_read(&mut cursor), Some(TokenPunct::DoublePipe));
        cursor.advance();
        assert_eq!(TokenPunct::try_read(&mut cursor), Some(TokenPunct::DoubleAmpersand));
        cursor.advance();
        assert_eq!(TokenPunct::try_read(&mut cursor), Some(TokenPunct::DoubleEq));
        cursor.advance();
        assert_eq!(TokenPunct::try_read(&mut cursor), Some(TokenPunct::RightShiftEq));
        cursor.advance();
        assert_eq!(TokenPunct::try_read(&mut cursor), Some(TokenPunct::NotEq));
        cursor.advance();
        assert_eq!(TokenPunct::try_read(&mut cursor), Some(TokenPunct::FatArrow));
        cursor.advance();
        assert_eq!(TokenPunct::try_read(&mut cursor), Some(TokenPunct::Arrow));
    }
}
