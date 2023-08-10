#[derive(Debug)]
pub enum Statement {
    //TODO: Syntax for patern matching, in place type ascription, and parameters
    Assignment {
        ident: String,
        // args: Vec<Node>,
        expr: Expression,
        // type_: Option<Type>,
    },
    TypeAscription {
        ident: String,
        type_: Type,
    },
    Include {
        source: String,
    },
    EOI,
}

//TODO: Add more
#[derive(Debug)]
pub enum Expression {
    If {
        cond: Box<Expression>,
        then: Box<Expression>,
        else_: Box<Expression>,
    },
    FunctionApplication {
        function: Box<Expression>,
        argument: Box<Expression>,
    },
    Comparison {
        operator: ComparisonOperator,
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    Term {
        operator: TermOperator,
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    Factor {
        operator: FactorOperator,
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    Unary {
        operator: UnaryOperator,
        rhs: Box<Expression>,
    },
    Primary(Primary),
}

#[derive(Debug)]
pub enum ComparisonOperator {
    Equivalent,
    NotEquivalent,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
}

#[derive(Debug)]
pub enum TermOperator {
    Add,
    Sub,
}

#[derive(Debug)]
pub enum FactorOperator {
    Mul,
    Div,
    Remainder,
}

#[derive(Debug)]
pub enum UnaryOperator {
    Negate,
    Not,
    Negative,
}

#[derive(Debug)]
pub enum Primary {
    Lambda {
        param: String,
        body: Box<Expression>,
    },
    Literal(Literal),
    Grouping {
        expr: Box<Expression>,
    },
    Ident(String),
}

#[derive(Debug)]
pub enum Literal {
    String(String),
    Int(f64),
    Bool(bool),
    Color(palette::Xyza<palette::white_point::D65, f64>),
    Unit,
}

#[derive(Debug)]
pub enum Type {
    String,
    Int,
    Color,
    Bool,
    Unit,
    Array(Box<Type>),
    Fun { input: Box<Type>, output: Box<Type> },
}
