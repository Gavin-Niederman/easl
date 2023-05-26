#[derive(Debug)]
pub enum Token {
    Ident(String),
    String(String),
    Int(i64),
    Float(f64),
    Color(palette::Xyz),
    ColorAlpha(palette::Xyza),

    Arrow,
    Lambda,

    LeftParen,
    RightParen,

    Equals,
    Plus,
    Minus,
    Slash,
    Star,
    Bang,
    LeftAngleBracket,
    RightAngleBracket,

    If,
    Then,
    Else,
    Let,
    In,

    Eof,
}

#[derive(Debug)]
pub enum ScannerError {
    Test,
}
impl std::error::Error for ScannerError {}

impl std::fmt::Display for ScannerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Test => writeln!(f, "Test"),
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

                // Multiple case tokens
                '-' => {
                    // Prevent indexing past the length of the line
                    if (i + 1) >= chars_vec.len() {
                        tokens.push(Token::Minus);
                    } else {
                        if chars_vec[i + 1] == '>' {
                            tokens.push(Token::Arrow);
                            i += 1;
                        } else {
                            tokens.push(Token::Minus);
                        }
                    }
                }

                _ => {}
            }

            i += 1;
        }
    }
    tokens.push(Token::Eof);

    Ok(tokens)
}