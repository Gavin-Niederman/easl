use miette::Diagnostic;
use thiserror::Error;

use crate::{TokenType, Tokens, SOURCE};

#[derive(Debug)]
pub enum Expr {
    Grouping(Box<Expr>),
    Unary(Box<UnaryExpr>),
    Binary(Box<BinaryExpr>),
    Literal(Literal),
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Grouping(expr) => write!(f, "({})", expr),
            Self::Binary(binary) => write!(
                f,
                "({} {:#?} {})",
                binary.left_expr, binary.operator, binary.right_expr
            ),
            Self::Unary(unary) => write!(f, "({:#?} {})", unary.operator, unary.expr),
            Self::Literal(literal) => write!(f, "{literal}"),
        }
    }
}

#[derive(Debug)]
pub enum Literal {
    Int(i64),
    Float(f64),
    Bool(bool),
    String(String),
}

impl std::fmt::Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bool(literal) => write!(f, "{literal}"),
            Self::Float(literal) => write!(f, "{literal}"),
            Self::Int(literal) => write!(f, "{literal}"),
            Self::String(literal) => write!(f, "{literal}"),
        }
    }
}

#[derive(Debug)]
pub enum UnaryOperator {
    Not,
    LogicalNegate,
    BinaryNegate,
}

#[derive(Debug)]
pub struct UnaryExpr {
    pub operator: UnaryOperator,
    pub expr: Expr,
}

#[derive(Debug)]
pub enum BinaryOperator {
    Equality,
    GreaterThan,
    GreaterThanEqual,
    LessThan,
    LessThanEqual,

    Equals,

    Arrow,
    TypeDeclaration,

    BinaryAnd,
    LogicalAnd,
    BinaryOr,
    LogicalOr,

    Add,
    Sub,
    Mul,
    Div,
    Xor,
    Mod,
}

#[derive(Debug)]
pub struct BinaryExpr {
    pub operator: BinaryOperator,
    pub left_expr: Expr,
    pub right_expr: Expr,
}

pub fn parse(mut tokens: Tokens) -> Result<Expr, ParserError> {
    equality(&mut tokens)
}

fn equality(tokens: &mut Tokens) -> Result<Expr, ParserError> {
    let left = comparison(tokens)?;

    loop {
        let Ok(next_tokens) = tokens.clone().next_chunk::<2>() else {
            break
        };
        match (
            next_tokens[0].clone().token_type,
            next_tokens[1].clone().token_type,
        ) {
            (TokenType::Bang, TokenType::Equals) => {
                tokens.next_chunk::<2>().unwrap();
            }
            (TokenType::Equals, TokenType::Equals) => {
                tokens.next_chunk::<2>().unwrap();
            }
            _ => {
                break;
            }
        }
    }

    Ok(left)
}

fn comparison(tokens: &mut Tokens) -> Result<Expr, ParserError> {
    let start_offset = tokens.next().unwrap().offset;
    Err(ParserError::Test {
        src: SOURCE.to_string(),
        span: miette::SourceSpan::new(
            start_offset,
            (tokens.next().unwrap().offset.offset() - start_offset.offset()).into(),
        ),
    })
}

#[derive(Error, Diagnostic, Debug)]
pub enum ParserError {
    #[error("Test")]
    #[diagnostic(help = "Test")]
    Test {
        #[source_code]
        src: String,

        #[label]
        span: miette::SourceSpan,
    },
}
