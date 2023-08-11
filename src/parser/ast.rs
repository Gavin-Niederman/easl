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
    Identifier(Identifier),
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
pub struct Primary {
    pub primary_type: PrimaryType,
    pub span: miette::SourceSpan,
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
