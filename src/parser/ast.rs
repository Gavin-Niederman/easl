#[derive(Debug)]
pub enum Statement {
    Assignment {
        ident: String,
        args: Vec<Node>,
        expr: Node,
        type_: Option<Type>,
    },
    TypeAscription {
        ident: String,
        type_: Type,
    },
    Include {
        source: String,
    }, //TODO: Add more
}

#[derive(Debug)]
pub enum Node {
    If {
        cond: Box<Node>,
        then: Box<Node>,
        else_: Box<Node>,
    },
    FunctionCall {
        ident: String,
        args: Vec<Node>,
    },
    Comparison {
        operator: ComparisonOperator,
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
    Term {
        operator: TermOperator,
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
    Factor {
        operator: FactorOperator,
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
    Unary {
        operator: UnaryOperator,
        expr: Box<Node>,
    },
    Primary(Primary),
    //TODO: Add more
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
    Literal(Literal),
    Grouping { expr: Box<Node> },
    Ident(String),
}

#[derive(Debug)]
pub enum Literal {
    String(String),
    Int(f64),
    Bool(bool),
    Color(palette::Xyza<palette::white_point::D65, f64>),
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
