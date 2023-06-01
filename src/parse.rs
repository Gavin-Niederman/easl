pub enum Expr {
    Grouping(Box<Expr>),
    Unary(Box<UnaryExpr>),
    Binary(Box<BinaryExpr>),
    Literal(Literal),
}

pub enum Literal {
    Int(i64),
    Float(f64),
    Bool(bool),
    String(String),
}

pub enum UnaryOperator {
    Not,
    LogicalNegate,
    BinaryNegate,
}

pub struct UnaryExpr {
    pub operator: UnaryOperator,
    pub expr: Expr,
}

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

pub struct BinaryExpr {
    pub operator: BinaryOperator,
    pub left_expr: Expr,
    pub right_expr: Expr,
}