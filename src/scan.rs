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

    If,
    Then,
    Else,
    Let,
    In,
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
        for ch in line.chars() {
            match ch {
                '#' => break,
                '\\' => tokens.push(Token::Lambda),
                '(' => tokens.push(Token::LeftParen),
                ')' => tokens.push(Token::RightParen),
                '=' => tokens.push(Token::Equals),
                '+' => tokens.push(Token::Plus),
                '/' => tokens.push(Token::Slash),
                '*' => tokens.push(Token::Star),
                '!' => tokens.push(Token::Bang),
                _ => {}
            }
        }
    }

    Ok(tokens)
}