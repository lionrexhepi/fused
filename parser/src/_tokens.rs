use core::panic;
use std::{ collections::VecDeque, io::BufReader };

use crate::{ file::{ SourceFile, SourceFileError }, location::SourceLocation };

#[derive(Debug, PartialEq, Eq)]
pub struct Ident(pub String);

type Result<T> = std::result::Result<T, TokenError>;

#[derive(Debug, PartialEq, Eq)]
pub enum Literal {
    Number(String),
    String(String),
}

impl Literal {
    pub fn parse_number(file: &mut SourceFile) -> Result<Self> {
        let (mut number, _) = file.until(|c| !matches!(c, '0'..='9'))?;

        if let Some(c) = file.next_if(|c| c == '.')?.0 {
            number.push(c);
            number.push_str(&file.until(|c| !matches!(c, '0'..='9'))?.0);
        }

        Ok(Self::Number(number))
    }

    pub fn parse_string(file: &mut SourceFile, quote: char) -> Result<Self> {
        file.next()?;
        let mut string = String::new();

        loop {
            string += &file.until(|c| (c == quote || c == '\\'))?.0;

            match file.peek().0 {
                Some('\\') => {
                    if let Some(next) = file.next()?.0 {
                        match next {
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
                    }
                }
                _ => {
                    break;
                }
            }
        }

        Ok(Self::String(string))
    }
}

#[derive(Debug, PartialEq, Eq)]
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
        if let (Some(c), _) = file.next()? {
            Ok(match c {
                '+' => {
                    if file.next_if_eq('=')? { Self::PlusEq } else { Self::Plus }
                }
                '-' => {
                    if file.next_if_eq('>')? {
                        if file.next_if_eq('>')? { Self::FatArrow } else { Self::Arrow }
                    } else if file.next_if_eq('=')? {
                        Self::MinusEq
                    } else {
                        Self::Minus
                    }
                }
                '*' => {
                    if file.next_if_eq('=')? { Self::StarEq } else { Self::Star }
                }
                '/' => {
                    if file.next_if_eq('=')? { Self::SlashEq } else { Self::Slash }
                }
                '%' => {
                    if file.next_if_eq('=')? { Self::PercentEq } else { Self::Percent }
                }
                '^' => {
                    if file.next_if_eq('=')? { Self::CaretEq } else { Self::Caret }
                }
                '&' => {
                    if file.next_if_eq('=')? {
                        Self::AmpersandEq
                    } else if file.next_if_eq('&')? {
                        Self::DoubleAmpersand
                    } else {
                        Self::Ampersand
                    }
                }
                '|' => {
                    if file.next_if_eq('=')? {
                        Self::PipeEq
                    } else if file.next_if_eq('|')? {
                        Self::DoublePipe
                    } else {
                        Self::Pipe
                    }
                }
                '~' => Self::Tilde,
                '?' => Self::Question,
                '!' => {
                    if file.next_if_eq('=')? { Self::NotEq } else { Self::Exclamation }
                }
                '.' => Self::Dot,
                ',' => Self::Comma,
                ':' => Self::Colon,
                ';' => Self::SemiColon,
                '=' => {
                    if file.next_if_eq('=')? { Self::DoubleEq } else { Self::Eq }
                }
                '<' => {
                    if file.next_if_eq('=')? {
                        Self::LtEq
                    } else if file.next_if_eq('<')? {
                        if file.next_if_eq('=')? { Self::LeftShiftEq } else { Self::LeftShift }
                    } else {
                        Self::Lt
                    }
                }
                '>' => {
                    if file.next_if_eq('=')? {
                        Self::GtEq
                    } else if file.next_if_eq('>')? {
                        if file.next_if_eq('=')? { Self::RightShiftEq } else { Self::RightShift }
                    } else {
                        Self::Gt
                    }
                }
                '#' => Self::Hash,
                '$' => Self::Dollar,
                '@' => Self::At,
                '\\' => Self::Backslash,
                '\n' => Self::Newline,
                _ => panic!("Invalid Punctuation"), //Check the is_punct_char function before calling this
            })
        } else {
            return Err(TokenError::UnexpectedEOF);
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum GroupDelim {
    Parens,
    Braces,
    Brackets,
}

impl GroupDelim {
    pub fn is_opening_char(c: char) -> bool {
        matches!(c, '(' | '[' | '{')
    }

    pub fn from_opening(c: char) -> std::result::Result<Self, char> {
        match c {
            '(' => Ok(Self::Parens),
            '[' => Ok(Self::Brackets),
            '{' => Ok(Self::Braces),
            _ => Err(c),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Group {
    content: TokenStream,
    delim: GroupDelim,
}

impl Group {
    fn parse(file: &mut SourceFile, delim: GroupDelim) -> Result<Self> {
        let mut braces = 0;
        let mut brackets = 0;
        let mut parens = 0;

        match delim {
            GroupDelim::Braces => {
                braces += 1;
            }
            GroupDelim::Brackets => {
                brackets += 1;
            }
            GroupDelim::Parens => {
                parens += 1;
            }
        }

        let mut content = String::new();

        let end = loop {
            let (c, pos) = file.next()?;
            if let Some(c) = c {
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
                    other => {
                        content.push(other);
                    }
                }

                if !(braces > 0 || brackets > 0 || parens > 0) {
                    break pos;
                }
            } else {
                break pos;
            }
        };

        if braces < 0 || braces < 0 || parens < 0 {
            return Err(TokenError::UnmatchedDelim(end));
        }

        let content_file = SourceFile::new(file.name.clone(), Box::new(content.as_bytes()))?;

        let content = TokenStream::new(content_file)?;

        Ok(Self {
            content,
            delim,
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum TokenType {
    Ident(Ident),
    Literal(Literal),
    Punct(Punct),
    Space(usize),
    Group(Group),
    EOF,
}

#[derive(Debug)]
pub enum TokenError {
    UnexpectedChar(char),
    FileError(SourceFileError),
    InvalidEscape(String),
    UnmatchedDelim(SourceLocation),
    UnexpectedEOF,
}

impl From<SourceFileError> for TokenError {
    fn from(err: SourceFileError) -> Self {
        Self::FileError(err)
    }
}

pub type Token = (TokenType, SourceLocation);

#[derive(Debug, PartialEq, Eq)]
pub struct TokenStream {
    tokens: VecDeque<Token>,
}

impl TokenStream {
    pub fn new(mut file: SourceFile<'_>) -> Result<Self> {
        let mut tokens = VecDeque::new();

        let eof = loop {
            let (c, pos) = match file.peek() {
                (Some(c), pos) => (c, pos),
                (None, pos) => {
                    break pos;
                }
            };

            let token_type = match c {
                '0'..='9' => TokenType::Literal(Literal::parse_number(&mut file)?),
                quote @ ('"' | '\'') => {
                    TokenType::Literal(Literal::parse_string(&mut file, quote)?)
                }

                delim if GroupDelim::is_opening_char(delim) => {
                    //Unwrap is fine because we just checked that delim is a valid opening char
                    TokenType::Group(Group::parse(&mut file, GroupDelim::from_opening(c).unwrap())?)
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
                        space = file.next()?.0.unwrap_or('\0');
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
            file.next()?;
        };

        Ok(Self { tokens })
    }

    pub fn next(&mut self) -> Option<Token> {
        self.tokens.pop_front()
    }
}
