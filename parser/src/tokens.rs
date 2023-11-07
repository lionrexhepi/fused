use core::panic;
use std::{ collections::VecDeque, io::BufReader };

use crate::{ file::{ SourceFile, SourceFileError }, location::SourceLocation };

pub struct Ident(String);

type Result<T> = std::result::Result<T, TokenError>;

pub enum Literal {
    Number(String),
    String(String),
}

impl Literal {
    pub fn parse_number(file: &mut SourceFile) -> Result<Self> {
        let (mut number, _) = file.until(|c| !matches!(c, '0'..='9'))?;

        if let crate::file::Result::Ok(Some((c, _))) = file.next_if(|c| c == '.') {
            number.push(c);
            number.push_str(&file.until(|c| !matches!(c, '0'..='9'))?.0);
        }

        Ok(Self::Number(number))
    }

    pub fn parse_string(file: &mut SourceFile, quote: char) -> Result<Self> {
        let (mut string, _) = file.until(|c| (c == quote || c == '\\'))?;

        match file.peek()?.0 {
            append @ ('\\' | '"' | '\'') => {
                file.next()?;
                string.push(append);
            }
            'n' => {
                file.next()?;
                string.push('\n');
            }
            'r' => {
                file.next()?;
                string.push('\r');
            }
            't' => {
                file.next()?;
                string.push('\t');
            }
            '0' => {
                file.next()?;
                string.push('\0');
            }
            'x' => {
                file.next()?;
                let (hex, _) = file.next_seq(2)?;
                let char_code = match u8::from_str_radix(&hex, 16) {
                    Ok(code) => code,
                    Err(_) => {
                        return Err(TokenError::InvalidEscape(hex));
                    }
                };
                string.push(char_code as char);
            }
            other @ _ => {
                return Err(TokenError::InvalidEscape(other.to_string()));
            }
        }

        Ok(Self::String(string))
    }
}

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
    LBrace, // {
    RBrace, // }
    LBracket, // [
    RBracket, // ]
    LParen, // (
    RParen, // )
    Arrow, // ->
    FatArrow, // =>
    Hash, // #
    Dollar, // $
    At, // @
    Underscore, // _
    Backslash, // \
    Newline,
    DoubleAmpersand, // \n
}

impl Punct {
    pub const fn is_punct_char(c: char) -> bool {
        matches!(
            c,
            '+' |
                '-' |
                '*' |
                '/' |
                '%' |
                '^' |
                '&' |
                '|' |
                '~' |
                '?' |
                '!' |
                '.' |
                ',' |
                ':' |
                ';' |
                '=' |
                '<' |
                '>' |
                '{' |
                '}' |
                '[' |
                ']' |
                '(' |
                ')' |
                '#' |
                '$' |
                '@' |
                '_' |
                '\\'
        )
    }

    pub fn parse(file: &mut SourceFile) -> Result<Self> {
        Ok(match file.next()?.0 {
            '+' => {
                if file.next_is('=')? { Self::PlusEq } else { Self::Plus }
            }
            '-' => {
                if file.next_is('>')? {
                    if file.next_is('>')? { Self::FatArrow } else { Self::Arrow }
                } else if file.next_is('=')? {
                    Self::MinusEq
                } else {
                    Self::Minus
                }
            }
            '*' => {
                if file.next_is('=')? { Self::StarEq } else { Self::Star }
            }
            '/' => {
                if file.next_is('=')? { Self::SlashEq } else { Self::Slash }
            }
            '%' => {
                if file.next_is('=')? { Self::PercentEq } else { Self::Percent }
            }
            '^' => {
                if file.next_is('=')? { Self::CaretEq } else { Self::Caret }
            }
            '&' => {
                if file.next_is('=')? {
                    Self::AmpersandEq
                } else if file.next_is('&')? {
                    Self::DoubleAmpersand
                } else {
                    Self::Ampersand
                }
            }
            '|' => {
                if file.next_is('=')? {
                    Self::PipeEq
                } else if file.next_is('|')? {
                    Self::DoublePipe
                } else {
                    Self::Pipe
                }
            }
            '~' => Self::Tilde,
            '?' => Self::Question,
            '!' => {
                if file.next_is('=')? { Self::NotEq } else { Self::Exclamation }
            }
            '.' => Self::Dot,
            ',' => Self::Comma,
            ':' => Self::Colon,
            ';' => Self::SemiColon,
            '=' => {
                if file.next_is('=')? { Self::DoubleEq } else { Self::Eq }
            }
            '<' => {
                if file.next_is('=')? {
                    Self::LtEq
                } else if file.next_is('<')? {
                    if file.next_is('=')? { Self::LeftShiftEq } else { Self::LeftShift }
                } else {
                    Self::Lt
                }
            }
            '>' => {
                if file.next_is('=')? {
                    Self::GtEq
                } else if file.next_is('>')? {
                    if file.next_is('=')? { Self::RightShiftEq } else { Self::RightShift }
                } else {
                    Self::Gt
                }
            }
            '{' => Self::LBrace,
            '}' => Self::RBrace,
            '[' => Self::LBracket,
            ']' => Self::RBracket,
            '(' => Self::LParen,
            ')' => Self::RParen,
            '#' => Self::Hash,
            '$' => Self::Dollar,
            '@' => Self::At,
            '\\' => Self::Backslash,
            '\n' => Self::Newline,
            _ => panic!("Invalid Punctuation"), //Check the is_punct_char function before calling this
        })
    }
}

struct Group {
    content: TokenStream,
    delim: Punct,
}

impl Group {
    fn parse(file: &mut SourceFile, delim_char: char) -> Result<Self> {
        let mut braces = 0;
        let mut brackets = 0;
        let mut parens = 0;

        match delim_char {
            '{' => {
                braces += 1;
            }
            '[' => {
                brackets += 1;
            }
            '(' => {
                parens += 1;
            }
            _ => panic!("Invalid Delimiter"),
        }

        let mut content = String::new();

        while braces > 0 || brackets > 0 || parens > 0 {
            let (c, _) = file.next()?;

            match c {
                '{' => {
                    braces += 1;
                }
                '}' => {
                    braces -= 1;
                }
                '[' => {
                    brackets += 1;
                }
                ']' => {
                    brackets -= 1;
                }
                '(' => {
                    parens += 1;
                }
                ')' => {
                    parens -= 1;
                }
                _ => {}
            }
        }

        let content_file = SourceFile::new(file.name.clone(), Box::new(content.as_bytes()));

        let content = TokenStream::new(content_file)?;

        Ok(Self {
            content,
            delim: match delim_char {
                '(' => Punct::LParen,

                _ => panic!("Invalid Delimiter"),
            },
        })
    }
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
    pub fn new(mut file: SourceFile<'_>) -> Result<Self> {
        let mut tokens = VecDeque::new();

        loop {
            let (c, pos) = match file.peek() {
                Ok((c, pos)) => (c, pos),
                Err(SourceFileError::EOF) => {
                    break;
                }
                Err(SourceFileError::IoError(e)) => panic!("Error: {:?}", e.to_string()),
            };

            let token_type = match c {
                '0'..='9' => TokenType::Literal(Literal::parse_number(&mut file)?),
                quote @ ('"' | '\'') => {
                    TokenType::Literal(Literal::parse_string(&mut file, quote)?)
                }

                punct if Punct::is_punct_char(punct) => TokenType::Punct(Punct::parse(&mut file)?),

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
                    let (ident, _) = file.until(
                        |c| !matches!(c, 'a'..='z' | 'A'..='Z' | '0'..='9' | '_')
                    )?;

                    TokenType::Ident(Ident(ident))
                }

                _ => {
                    return Err(TokenError::UnexpectedChar(c));
                }
            };

            tokens.push_back((token_type, pos));
        }

        Ok(Self { tokens })
    }
}
