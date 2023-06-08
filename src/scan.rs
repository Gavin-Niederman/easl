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
    pub span: miette::SourceSpan,
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

    let mut loc_line = 0;

    let lines = source.lines().map(|line| {
        let mut loc_col = 0;

        let words = line.split(' ').map(move |word| {
            let start_offset = loc_col;

            let chars = word.chars().map(move |ch| {
                loc_col += 1;

                (loc_col, ch)
            });

            if let Some((offset, _)) = chars.clone().last() {
                loc_col += offset - start_offset + 1;
            }

            chars
        });

        loc_line += 1;

        (loc_line, words)
    });

    for (line, words) in lines {
        let mut start_offset = miette::SourceOffset::from_location(&source, line, 0);

        'words: for word in words {
            let mut end_col = 0;
            let mut string = String::new();

            for (offset, ch) in word {
                start_offset = miette::SourceOffset::from_location(&source, line, offset);

                match ch {
                    '#' => break 'words,
                    ch => {
                        if let Some(token) = create_short_token(ch, line, offset, &source) {
                            tokens.push(token);
                            string.clear();
                            start_offset =
                                miette::SourceOffset::from_location(&source, line, offset);
                        } else {
                            string.push(ch);
                        }
                    }
                }

                end_col = offset;
            }

            if let Some(token) = create_long_token(
                &string,
                miette::SourceSpan::new(
                    start_offset,
                    miette::SourceOffset::from_location(&source, line, end_col),
                ),
            ) {
                tokens.push(token);
            }
        }
    }

    Ok(Tokens { tokens })
}

fn create_long_token(token: &str, span: miette::SourceSpan) -> Option<Token> {
    if let Some(token_type) = match token {
        "if" => Some(TokenType::If),
        "then" => Some(TokenType::Then),
        "else" => Some(TokenType::Else),
        "let" => Some(TokenType::Let),
        "in" => Some(TokenType::In),
        "True" => Some(TokenType::Bool(true)),
        "False" => Some(TokenType::Bool(false)),
        "" => None,
        other => {
            let ident = Some(TokenType::Ident(other.to_string()));

            if other.contains(".") {
                if let Ok(float) = other.parse() {
                    Some(TokenType::Float(float))
                } else {
                    ident
                }
            } else {
                if let Ok(int) = other.parse() {
                    Some(TokenType::Int(int))
                } else {
                    ident
                }
            }
        }
    } {
        return Some(Token { token_type, span });
    }

    None
}

fn create_short_token(ch: char, loc_line: usize, loc_col: usize, source: &str) -> Option<Token> {
    if let Some(token_type) = match ch {
        '\\' => Some(TokenType::Lambda),
        '(' => Some(TokenType::LeftParen),
        ')' => Some(TokenType::RightParen),
        '=' => Some(TokenType::Equals),
        '+' => Some(TokenType::Plus),
        '-' => Some(TokenType::Dash),
        '/' => Some(TokenType::Slash),
        '*' => Some(TokenType::Star),
        '!' => Some(TokenType::Bang),
        '<' => Some(TokenType::LeftAngleBracket),
        '>' => Some(TokenType::RightAngleBracket),
        ':' => Some(TokenType::Colon),
        '&' => Some(TokenType::Ampersand),
        '|' => Some(TokenType::Bar),
        '^' => Some(TokenType::Caret),
        _ => None,
    } {
        return Some(Token {
            token_type,
            span: miette::SourceSpan::new(
                miette::SourceOffset::from_location(source, loc_line, loc_col),
                miette::SourceOffset::from_location(source, loc_line, loc_col + 1),
            ),
        });
    }

    None
}
