#[derive(Debug)]
pub enum Token {
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

#[derive(Debug)]
pub enum ScannerError {
    UnboundedString,
}
impl std::error::Error for ScannerError {}

impl std::fmt::Display for ScannerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnboundedString => writeln!(f, "String without closing quotes detected!"),
        }
    }
}

pub fn scan(source: String) -> Result<Vec<Token>, ScannerError> {
    let mut tokens = Vec::new();

    for line in source.lines() {
        let chars_vec: Vec<char> = line.chars().collect();
        let mut i = 0;
        while i < chars_vec.len() {
            let ch = chars_vec[i];
            match ch {
                // Single character tokens
                '#' => break,
                '\\' => tokens.push(Token::Lambda),
                '(' => tokens.push(Token::LeftParen),
                ')' => tokens.push(Token::RightParen),
                '=' => tokens.push(Token::Equals),
                '+' => tokens.push(Token::Plus),
                '/' => tokens.push(Token::Slash),
                '*' => tokens.push(Token::Star),
                '!' => tokens.push(Token::Bang),
                '<' => tokens.push(Token::LeftAngleBracket),
                '>' => tokens.push(Token::RightAngleBracket),
                '-' => tokens.push(Token::Dash),
                ':' => tokens.push(Token::Colon),
                '&' => tokens.push(Token::Ampersand),
                '|' => tokens.push(Token::Bar),
                '^' => tokens.push(Token::Caret),

                // Strings
                '"' => {
                    tokens.push(scan_string(&mut i, &chars_vec)?);
                    // i -= 1;
                }

                // All other tokens
                _ => {
                    if let Some(token) = search_for_token(&mut i, &chars_vec) {
                        tokens.push(token);
                        // i -= 1;
                    }
                }
            }

            i += 1;
        }

        tokens.push(Token::NewLine);
    }
    tokens.push(Token::Eof);

    Ok(tokens)
}

fn search_for_token(i: &mut usize, chars: &Vec<char>) -> Option<Token> {
    let mut token = String::new();

    while *i < chars.len() {
        if chars[*i] == ' ' || chars[*i] == ')' || chars[*i] == '(' || chars[*i] == '\n' {
            break;
        } else {
            token.push(chars[*i])
        }
        *i += 1;
    }

    if token.contains(".") {
        if let Ok(num) = token.parse() {
            return Some(Token::Float(num));
        }
    } else {
        if let Ok(num) = token.parse() {
            return Some(Token::Int(num));
        }
    }

    let token = match token.as_str() {
        "" => None,
        "if" => Some(Token::If),
        "then" => Some(Token::Then),
        "else" => Some(Token::Else),
        "let" => Some(Token::Let),
        "in" => Some(Token::In),
        "True" => Some(Token::Bool(true)),
        "False" => Some(Token::Bool(false)),

        ident => Some(Token::Ident(String::from(ident))),
    };

    token
}

fn scan_string(i: &mut usize, chars: &Vec<char>) -> Result<Token, ScannerError> {
    *i += 1;

    let mut string = String::new();
    while *i < chars.len() {
        if *i + 1 >= chars.len() {
            return Err(ScannerError::UnboundedString);
        }

        if chars[*i] == '"' {
            break;
        } else {
            string.push(chars[*i]);
        }

        *i += 1;
    }

    Ok(Token::String(string))
}
