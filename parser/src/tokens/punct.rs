use crate::file::SourceCursor;

use super::{ TokenContent, TokenResult };

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
    DoubleAmpersandEq, // &&=
    PipeEq, // |=
    DoublePipeEq, // ||=
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
    DoubleLt, // <<
    DoubleLtEq, // <<=
    LtEq, // <=
    Gt, // >
    DoubleGt, // >>
    DoubleGtEq, // >>=
    GtEq, // >=
    Arrow, // ->
    FatArrow, // =>
    Dollar, // $
    Backslash, // \
    DoubleAmpersand, // &&
}

impl TokenContent for TokenPunct {
    fn try_read(cursor: &mut SourceCursor) -> TokenResult<Self> {
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
                    if cursor.next() == '=' {
                        cursor.advance();
                        Some(Self::DoubleAmpersandEq)
                    } else {
                        Some(Self::DoubleAmpersand)
                    }
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
                    if cursor.next() == '=' {
                        cursor.advance();
                        Some(Self::DoublePipeEq)
                    } else {
                        Some(Self::DoublePipe)
                    }
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
                        Some(Self::DoubleLtEq)
                    } else {
                        Some(Self::DoubleLt)
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
                        Some(Self::DoubleGtEq)
                    } else {
                        Some(Self::DoubleGt)
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

        Ok(result)
    }
}

#[cfg(test)]
mod test {
    use crate::{ file::SourceCursor, tokens::{ punct::TokenPunct, TokenContent } };

    #[test]
    fn test_single_puncts() {
        let mut cursor = SourceCursor::new("+ - * / % ^ & | ~ ? $ . \\ < > ! ; : ,");

        assert_eq!(TokenPunct::try_read(&mut cursor).unwrap(), Some(TokenPunct::Plus));
        cursor.advance();
        assert_eq!(TokenPunct::try_read(&mut cursor).unwrap(), Some(TokenPunct::Minus));
        cursor.advance();
        assert_eq!(TokenPunct::try_read(&mut cursor).unwrap(), Some(TokenPunct::Star));
        cursor.advance();
        assert_eq!(TokenPunct::try_read(&mut cursor).unwrap(), Some(TokenPunct::Slash));
        cursor.advance();
        assert_eq!(TokenPunct::try_read(&mut cursor).unwrap(), Some(TokenPunct::Percent));
        cursor.advance();
        assert_eq!(TokenPunct::try_read(&mut cursor).unwrap(), Some(TokenPunct::Caret));
        cursor.advance();
        assert_eq!(TokenPunct::try_read(&mut cursor).unwrap(), Some(TokenPunct::Ampersand));
        cursor.advance();
        assert_eq!(TokenPunct::try_read(&mut cursor).unwrap(), Some(TokenPunct::Pipe));
        cursor.advance();
        assert_eq!(TokenPunct::try_read(&mut cursor).unwrap(), Some(TokenPunct::Tilde));
        cursor.advance();
        assert_eq!(TokenPunct::try_read(&mut cursor).unwrap(), Some(TokenPunct::Question));
        cursor.advance();
        assert_eq!(TokenPunct::try_read(&mut cursor).unwrap(), Some(TokenPunct::Dollar));
        cursor.advance();
        assert_eq!(TokenPunct::try_read(&mut cursor).unwrap(), Some(TokenPunct::Dot));
        cursor.advance();
        assert_eq!(TokenPunct::try_read(&mut cursor).unwrap(), Some(TokenPunct::Backslash));
        cursor.advance();
        assert_eq!(TokenPunct::try_read(&mut cursor).unwrap(), Some(TokenPunct::Lt));
        cursor.advance();
        assert_eq!(TokenPunct::try_read(&mut cursor).unwrap(), Some(TokenPunct::Gt));
        cursor.advance();
        assert_eq!(TokenPunct::try_read(&mut cursor).unwrap(), Some(TokenPunct::Exclamation));
        cursor.advance();
        assert_eq!(TokenPunct::try_read(&mut cursor).unwrap(), Some(TokenPunct::SemiColon));
        cursor.advance();
        assert_eq!(TokenPunct::try_read(&mut cursor).unwrap(), Some(TokenPunct::Colon));
        cursor.advance();
        assert_eq!(TokenPunct::try_read(&mut cursor).unwrap(), Some(TokenPunct::Comma));
    }

    #[test]
    fn test_complex() {
        let mut cursor = SourceCursor::new(
            "+= -= *= /= ^= |= <= <<= || && == >>= != => -> &&= ||="
        );

        assert_eq!(TokenPunct::try_read(&mut cursor).unwrap(), Some(TokenPunct::PlusEq));
        cursor.advance();
        assert_eq!(TokenPunct::try_read(&mut cursor).unwrap(), Some(TokenPunct::MinusEq));
        cursor.advance();
        assert_eq!(TokenPunct::try_read(&mut cursor).unwrap(), Some(TokenPunct::StarEq));
        cursor.advance();
        assert_eq!(TokenPunct::try_read(&mut cursor).unwrap(), Some(TokenPunct::SlashEq));
        cursor.advance();
        assert_eq!(TokenPunct::try_read(&mut cursor).unwrap(), Some(TokenPunct::CaretEq));
        cursor.advance();
        assert_eq!(TokenPunct::try_read(&mut cursor).unwrap(), Some(TokenPunct::PipeEq));
        cursor.advance();
        assert_eq!(TokenPunct::try_read(&mut cursor).unwrap(), Some(TokenPunct::LtEq));
        cursor.advance();
        assert_eq!(TokenPunct::try_read(&mut cursor).unwrap(), Some(TokenPunct::DoubleLtEq));
        cursor.advance();
        assert_eq!(TokenPunct::try_read(&mut cursor).unwrap(), Some(TokenPunct::DoublePipe));
        cursor.advance();
        assert_eq!(TokenPunct::try_read(&mut cursor).unwrap(), Some(TokenPunct::DoubleAmpersand));
        cursor.advance();
        assert_eq!(TokenPunct::try_read(&mut cursor).unwrap(), Some(TokenPunct::DoubleEq));
        cursor.advance();
        assert_eq!(TokenPunct::try_read(&mut cursor).unwrap(), Some(TokenPunct::DoubleGtEq));
        cursor.advance();
        assert_eq!(TokenPunct::try_read(&mut cursor).unwrap(), Some(TokenPunct::NotEq));
        cursor.advance();
        assert_eq!(TokenPunct::try_read(&mut cursor).unwrap(), Some(TokenPunct::FatArrow));
        cursor.advance();
        assert_eq!(TokenPunct::try_read(&mut cursor).unwrap(), Some(TokenPunct::Arrow));

        cursor.advance();
        assert_eq!(TokenPunct::try_read(&mut cursor).unwrap(), Some(TokenPunct::DoubleAmpersandEq));
        cursor.advance();
        assert_eq!(TokenPunct::try_read(&mut cursor).unwrap(), Some(TokenPunct::DoublePipeEq));
    }
}
