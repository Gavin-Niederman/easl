use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

#[derive(Debug)]
pub enum Statement {
    //TODO: Syntax for patern matching, in place type ascription, and parameters
    Assignment {
        ident: Identifier,
        // args: Vec<Node>,
        expr: Expression,
        // type_: Option<Type>,
    },
    TypeAscription {
        ident: Identifier,
        type_: Type,
    },
    Include {
        source: String,
    },
    EOI,
}

#[derive(Debug)]
pub struct Expression {
    pub expression_type: ExpressionType,
    pub span: miette::SourceSpan,
}

//TODO: Add more
#[derive(Debug)]
pub enum ExpressionType {
    If {
        cond: Box<Expression>,
        then: Box<Expression>,
        else_: Box<Expression>,
    },
    FunctionApplication {
        function: Box<Expression>,
        argument: Box<Expression>,
    },
    Binary {
        operator: BinaryOperator,
        lhs: Box<Expression>,
        rhs: Box<Expression>
    },
    // Comparison {
    //     operator: ComparisonOperator,
    //     lhs: Box<Expression>,
    //     rhs: Box<Expression>,
    // },
    // Term {
    //     operator: TermOperator,
    //     lhs: Box<Expression>,
    //     rhs: Box<Expression>,
    // },
    // Factor {
    //     operator: FactorOperator,
    //     lhs: Box<Expression>,
    //     rhs: Box<Expression>,
    // },
    Unary {
        operator: UnaryOperator,
        rhs: Box<Expression>,
    },
    Identifier(Identifier),
    Primary(Primary),
}

#[derive(Debug)]
pub enum BinaryOperator {
    Equivalent,
    NotEquivalent,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,

    Add,
    Sub,

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
pub struct Primary {
    pub primary_type: PrimaryType,
    pub span: miette::SourceSpan,
}

impl PartialEq for Primary {
    fn eq(&self, other: &Self) -> bool {
        match (self.primary_type, other.primary_type) {
            (PrimaryType::Bool(lhs), PrimaryType::Bool(rhs)) => {lhs == rhs}
            (PrimaryType::Color(lhs), PrimaryType::Color(rhs)) => {lhs == rhs}
            (PrimaryType::Int(lhs), PrimaryType::Int(rhs)) => {lhs == rhs}
            (PrimaryType::String(lhs), PrimaryType::String(rhs)) => {lhs == rhs}
            (PrimaryType::Unit, PrimaryType::Unit) => {true}
            _ => {panic!("Cannot compare {:?} and {:?}", self.primary_type, other.primary_type)}
        }
    }
}

impl Primary {
    pub fn is_same_type(primary_1: &Primary, primary_2: &Primary) -> bool {
        match (primary_1.primary_type, primary_2.primary_type) {
            (PrimaryType::Bool(_), PrimaryType::Bool(_)) => {true}
            (PrimaryType::Color(_), PrimaryType::Color(_)) => {true}
            (PrimaryType::Int(_), PrimaryType::Int(_)) => {true}
            (PrimaryType::String(_), PrimaryType::String(_)) => {true}
            (PrimaryType::Lambda { .. }, PrimaryType::Lambda { .. }) => {false}
            (PrimaryType::Unit, PrimaryType::Unit) => {true}
            _ => {false}
        }
    }
}

#[derive(Debug)]
pub enum PrimaryType {
    Lambda {
        param: String,
        body: Box<Expression>,
    },
    String(String),
    Int(f64),
    Bool(bool),
    Color(palette::Xyza<palette::white_point::D65, f64>),
    Unit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Identifier {
    pub handle: u64,
}
pub struct IdentifierMap {
    pub map: std::collections::HashMap<String, u64>,
}
impl IdentifierMap {
    pub fn new() -> Self {
        Self {
            map: vec![
                String::from("hsv"),
                String::from("rgb"),
                String::from("cmy"),
                String::from("xyz"),
                String::from("alpha"),
            ]
            .into_iter()
            .map(|name| {
                let mut hasher = DefaultHasher::new();
                name.hash(&mut hasher);
                (name, hasher.finish())
            })
            .collect(),
        }
    }
    pub fn create_identifier(&mut self, name: String) -> Identifier {
        let mut hasher = DefaultHasher::new();
        name.hash(&mut hasher);
        let handle = hasher.finish();
        self.map.insert(name, handle);
        Identifier { handle }
    }
    pub fn get(&self, identifier: &Identifier) -> Option<&String> {
        self.map
            .iter()
            .find(|(_, handle)| **handle == identifier.handle)
            .map(|(name, _)| name)
    }
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
