use crate::Token;

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

enum Associativity {
    Left,
    Right,
}

#[derive(Debug)]
pub struct BinaryExpr {
    pub operator: BinaryOperator,
    pub left_expr: Expr,
    pub right_expr: Expr,
}

pub fn parse(tokens: Vec<Token>) -> Result(Expr) {
    
}
