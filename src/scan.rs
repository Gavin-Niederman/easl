use miette::Diagnostic;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Ident(String),
    String(String),
    Int(i64),
    Float(f64),
    Bool(bool),

    Lambda,

    LeftParen,
    RightParen,

    Equals,
    Plus,
    Dash,
    Slash,
    Star,
    Bang,
    LeftAngleBracket,
    RightAngleBracket,
    Colon,
    Ampersand,
    Bar,
    Caret,

    If,
    Then,
    Else,
    Let,
    In,

    NewLine,
    Eof,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub start_offset: usize,
    pub len: usize,
}

impl Token {
    pub fn new(token_type: TokenType, start_offset: usize, len: usize) -> Self {
        Self {
            token_type,
            start_offset,
            len,
        }
    }
}

pub struct Tokens {
    tokens: Vec<Token>,
}

impl Iterator for Tokens {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.tokens.is_empty() {
            None
        } else {
            let token = self.tokens[0].clone();
            self.tokens = self.tokens[1..].to_vec();
            Some(token)
        }
    }
}

#[derive(Error, Debug, Diagnostic)]
pub enum ScannerError {
    #[error("Unbounded string")]
    #[diagnostic(help("Add closing doublequote"))]
    UnboundedString {
        #[source_code]
        src: String,

        #[label]
        err_location: miette::SourceOffset,
    },
}

pub fn scan(source: String) -> Result<Tokens, ScannerError> {
    let mut tokens = Vec::new();

    let mut chars = source.char_indices().peekable();

    // Generate tokens untill EOF
    let mut line = 1;
    let mut col = 1;

    loop {
        if let Some((start_offset, string)) = chars.next() {
            let mut string = String::from(string);
            let mut len = 0;

            // Single character tokens
            let token_type = match string.as_str() {
                "#" => {
                    comment(&mut chars);
                    line += 1;
                    col = 1;
                    continue;
                }
                "\"" => {
                    if let Some(string) = scan_string(&mut chars, start_offset) {
                        tokens.push(string)
                    } else {
                        let err_location = miette::SourceOffset::from_location(&source, line, col);

                        return Err(ScannerError::UnboundedString { src: source, err_location, });
                    }
                    continue;
                }
                "\n" => {
                    line += 1;
                    col = 1;
                    continue;
                },
                " " => {
                    col += 1;
                    continue;
                },
                "\\" => Some(TokenType::Lambda),
                "(" => Some(TokenType::LeftParen),
                ")" => Some(TokenType::RightParen),
                "=" => Some(TokenType::Equals),
                "+" => Some(TokenType::Plus),
                "/" => Some(TokenType::Slash),
                "*" => Some(TokenType::Star),
                "!" => Some(TokenType::Bang),
                "<" => Some(TokenType::LeftAngleBracket),
                ">" => Some(TokenType::RightAngleBracket),
                "-" => Some(TokenType::Dash),
                ":" => Some(TokenType::Colon),
                "&" => Some(TokenType::Ampersand),
                "|" => Some(TokenType::Bar),
                "^" => Some(TokenType::Caret),
                _ => None,
            };

            if let Some(token_type) = token_type {
                tokens.push(Token {
                    token_type,
                    start_offset,
                    len: 1,
                });

                col += 1;
                continue;
            }

            // Multi character tokens
            while chars.peek().is_some() {
                match chars.next().unwrap().1 {
                    '\n' => {
                        line += 1;
                        col = 1;
                        break;
                    },
                    ' ' => {
                        col += 1;
                        break;
                    }
                    '(' | ')' => break,
                    ch => {
                        string.push(ch);
                        len += 1;
                    }
                }

                col += 1;
            }

            let mut token_type = match string.as_str() {
                "if" => Some(TokenType::If),
                "then" => Some(TokenType::Then),
                "else" => Some(TokenType::Else),
                "let" => Some(TokenType::Let),
                "in" => Some(TokenType::In),
                "True" => Some(TokenType::Bool(true)),
                "False" => Some(TokenType::Bool(false)),
                "" | " " => None,

                ident => Some(TokenType::Ident(String::from(ident))),
            };

            // Check for numbers
            if string.contains(".") {
                if let Ok(num) = string.parse() {
                    token_type = Some(TokenType::Float(num));
                }
            } else {
                if let Ok(num) = string.parse() {
                    token_type = Some(TokenType::Int(num));
                }
            }

            // This should always pass becuase of idents, except for a token with only a space or nothing at all.
            if let Some(tokenized) = token_type {
                let token_type = tokenized;
                tokens.push(Token {
                    token_type,
                    start_offset,
                    len,
                })
            }
        // We have reached the end
        } else {
            tokens.push(Token {
                token_type: TokenType::Eof,
                start_offset: 0,
                len: 0,
            });
            break;
        }
    }

    Ok(Tokens { tokens })
}

fn comment(chars: &mut std::iter::Peekable<std::str::CharIndices>) {
    for (_, ch) in chars {
        if ch == '\n' {
            break;
        }
    }
}

fn scan_string(
    chars: &mut std::iter::Peekable<std::str::CharIndices>,
    start_offset: usize,
) -> Option<Token> {
    let mut string = String::new();
    let mut len = 0;
    while chars.peek().is_some() {
        let (_, ch) = chars.next().unwrap();

        match ch {
            '"' => break,
            ch => {
                if chars.peek().is_none() {
                    return None;
                }
                string.push(ch)
            }
        }
        len += 1;
    }

    Some(Token {
        token_type: TokenType::String(string),
        start_offset,
        len,
    })
}
