use std::{collections::VecDeque, io::Read};

use crate::{
    file::{SourceFile, SourceFileError},
    location::SourceLocation,
};

struct Ident(String);

enum Literal {
    Number(String),
    String(String),
}

pub enum Punct {
    Plus,        //+
    Minus,       //-
    Star,        //*
    Slash,       // /
    Percent,     // %
    Caret,       // ^
    Ampersand,   // &
    Pipe,        // |
    PlusEq,      // +=
    MinusEq,     // -=
    StarEq,      // *=
    SlashEq,     // /=
    PercentEq,   // %=
    CaretEq,     // ^=
    AmpersandEq, // &=
    PipeEq,      // |=
    Tilde,       // ~
    Question,    // ?
    Exclamation, // !
    Dot,         // .
    Comma,       // ,
    Colon,       // :
    SemiColon,   // ;
    Eq,          // =
    NotEq,       // !=
    Lt,          // <
    LtEq,        // <=
    Gt,          // >
    GtEq,        // >=
    LBrace,      // {
    RBrace,      // }
    LBracket,    // [
    RBracket,    // ]
    LParen,      // (
    RParen,      // )
    Arrow,       // ->
    FatArrow,    // =>
    Hash,        // #
    Dollar,      // $
    At,          // @
    Underscore,  // _
    Backslash,   // \
    Newline,     // \n
}

pub enum TokenType {
    Ident(Ident),
    Literal(Literal),
    Punct(Punct),
    Space(usize),
    EOF,
}

pub enum TokenError {
    UnexpectedChar(char),
    FileError(SourceFileError),
    InvalidEscape(String),
    UnexpectedEOF,
}

impl From<SourceFileError> for TokenError {
    fn from(err: SourceFileError) -> Self {
        Self::FileError(err)
    }
}

pub type Token = (TokenType, SourceLocation);

pub struct TokenStream {
    tokens: VecDeque<Token>,
}

impl TokenStream {
    pub fn new(mut file: SourceFile) -> Result<Self, TokenError> {
        let mut tokens = VecDeque::new();

        loop {
            let (c, pos) = match file.peek() {
                Ok((c, pos)) => (c, pos),
                Err(SourceFileError::EOF) => break,
                Err(SourceFileError::IoError(e)) => panic!("Error: {:?}", e.to_string()),
            };

            let token_type = match c {
                '0'..='9' => {
                    let (mut number, _) = file.until(|c| !matches!(c, '0'..='9'))?;

                    if let Result::Ok(Some((c, _))) = file.next_if(|c| c == '.') {
                        number.push(c);
                        number.push_str(&file.until(|c| !matches!(c, '0'..='9'))?.0);
                    }

                    TokenType::Literal(Literal::Number(number))
                }
                quote @ ('"' | '\'') => {
                    let (mut string, _) = file.until(|c| c == quote || c == '\\')?;

                    match file.peek()?.0 {
                        append @ ('\\' | '"' | '\'') => {
                            file.next()?;
                            string.push(append)
                        }
                        'n' => {
                            file.next()?;
                            string.push('\n')
                        }
                        'r' => {
                            file.next()?;
                            string.push('\r')
                        }
                        't' => {
                            file.next()?;
                            string.push('\t')
                        }
                        '0' => {
                            file.next()?;
                            string.push('\0')
                        }
                        'x' => {
                            file.next()?;
                            let (hex, _) = file.next_seq(2)?;
                            let char_code = match u8::from_str_radix(&hex, 16) {
                                Ok(code) => code,
                                Err(_) => return Err(TokenError::InvalidEscape(hex)),
                            };
                            string.push(char_code as char);
                        }
                        other @ _ => {
                            return Err(TokenError::InvalidEscape(other.to_string()));
                        }
                    }

                    TokenType::Literal(Literal::String(string))
                }

                '+' => {
                    if file.next_is('=')? {
                        TokenType::Punct(Punct::PlusEq)
                    } else {
                        TokenType::Punct(Punct::Plus)
                    }
                }
                '-' => {
                    if file.next_is('>')? {
                        if file.next_is('>')? {
                            TokenType::Punct(Punct::FatArrow)
                        } else {
                            TokenType::Punct(Punct::Arrow)
                        }
                    } else if file.next_is('=')? {
                        TokenType::Punct(Punct::MinusEq)
                    } else {
                        TokenType::Punct(Punct::Minus)
                    }
                }
                '*' => {
                    if file.next_is('=')? {
                        TokenType::Punct(Punct::StarEq)
                    } else {
                        TokenType::Punct(Punct::Star)
                    }
                }
                '/' => {
                    if file.next_is('=')? {
                        TokenType::Punct(Punct::SlashEq)
                    } else {
                        TokenType::Punct(Punct::Slash)
                    }
                }
                '%' => {
                    if file.next_is('=')? {
                        TokenType::Punct(Punct::PercentEq)
                    } else {
                        TokenType::Punct(Punct::Percent)
                    }
                }
                '^' => {
                    if file.next_is('=')? {
                        TokenType::Punct(Punct::CaretEq)
                    } else {
                        TokenType::Punct(Punct::Caret)
                    }
                }
                '&' => {
                    if file.next_is('=')? {
                        TokenType::Punct(Punct::AmpersandEq)
                    } else {
                        TokenType::Punct(Punct::Ampersand)
                    }
                }
                '|' => {
                    if file.next_is('=')? {
                        TokenType::Punct(Punct::PipeEq)
                    } else {
                        TokenType::Punct(Punct::Pipe)
                    }
                }
                '~' => TokenType::Punct(Punct::Tilde),
                '?' => TokenType::Punct(Punct::Question),
                '!' => {
                    if file.next_is('=')? {
                        TokenType::Punct(Punct::NotEq)
                    } else {
                        TokenType::Punct(Punct::Exclamation)
                    }
                }
                '.' => TokenType::Punct(Punct::Dot),
                ',' => TokenType::Punct(Punct::Comma),
                ':' => TokenType::Punct(Punct::Colon),
                ';' => TokenType::Punct(Punct::SemiColon),
                '=' => TokenType::Punct(Punct::Eq),
                '<' => {
                    if file.next_is('=')? {
                        TokenType::Punct(Punct::LtEq)
                    } else {
                        TokenType::Punct(Punct::Lt)
                    }
                }
                '>' => {
                    if file.next_is('=')? {
                        TokenType::Punct(Punct::GtEq)
                    } else {
                        TokenType::Punct(Punct::Gt)
                    }
                }
                '{' => TokenType::Punct(Punct::LBrace),
                '}' => TokenType::Punct(Punct::RBrace),
                '[' => TokenType::Punct(Punct::LBracket),
                ']' => TokenType::Punct(Punct::RBracket),
                '(' => TokenType::Punct(Punct::LParen),
                ')' => TokenType::Punct(Punct::RParen),
                '#' => TokenType::Punct(Punct::Hash),
                '$' => TokenType::Punct(Punct::Dollar),
                '@' => TokenType::Punct(Punct::At),
                '\\' => TokenType::Punct(Punct::Backslash),

                '\n' => TokenType::Punct(Punct::Newline),

                mut space @ (' ' | '\t') => {
                    let mut len = 0;
                    while matches!(space, ' ' | '\t') {
                        len += if space == ' ' {
                            1 // 1 Space
                        } else {
                            4 //1 Tab counts as 4 spaces
                        };
                        space = file.next()?.0;
                    }

                    TokenType::Space(len)
                }

                '_' | 'a'..='z' | 'A'..='Z' => {
                    let (ident, _) =
                        file.until(|c| !matches!(c, 'a'..='z' | 'A'..='Z' | '0'..='9' | '_'))?;

                    TokenType::Ident(Ident(ident))
                }

                _ => return Err(TokenError::UnexpectedChar(c)),
            };

            tokens.push_back((token_type, pos));
        }

        Ok(Self { tokens })
    }
}
