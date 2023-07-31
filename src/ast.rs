pub enum Statement {
    Assignment {
        ident: Ident,
        args: Vec<Node>,
        expr: Node,
        type_: Option<Type>,
    },
    TypeAscription {
        ident: String,
        type_: Type,
    },
    Include {
        source: String
    }
    //TODO: Add more 
}

pub enum Node {
    Literal(Literal),
    Unary {
        operator: UnaryOperator,
        expr: Box<Node>,
    },
    If {
        cond: Box<Node>,
        then: Box<Node>,
        else_: Box<Node>
    },
    FunctionCall {
        ident: Ident,
        args: Vec<Node>,
    },
    Grouping {
        expr: Box<Node>,
    },
    Ident(Ident),
    //TODO: Add more
}

pub enum UnaryOperator {
    Negate,
    Not,
}

pub enum Literal {
    Int(f64),
    Bool(bool),
    String(String),
    Color(palette::Xyza),
}

pub enum Type {
    String,
    Int,
    Color,
    Bool,
    Unit,
    Array(Box<Type>),
    Tuple(Box<Vec<Type>>),
    Fun {
        input: Box<Type>,
        output: Box<Type>,
    },
}

#[repr(transparent)]
pub struct Ident(String);
