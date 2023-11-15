use crate::file::Cursor;

pub enum Punct {
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

impl Punct {
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
