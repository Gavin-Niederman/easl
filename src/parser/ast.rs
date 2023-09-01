use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    ops::Range,
};

use crate::{tc::variant::Type, utils::Spanned};

#[derive(Debug, Clone)]
pub enum Statement {
    //TODO: Syntax for patern matching and parameters
    Assignment {
        ident: Identifier,
        // args: Vec<Node>,
        expr: Spanned<Expression>,
        type_ascription: Option<Box<Statement>>
    },
    TypeAscription {
        ident: Identifier,
        type_: Type,
    },
}

//TODO: Add more
#[derive(Debug, Clone)]
pub enum Expression {
    If {
        cond: Box<Spanned<Expression>>,
        then: Box<Spanned<Expression>>,
        else_: Box<Spanned<Expression>>,
    },
    Binary {
        operator: BinaryOperator,
        lhs: Box<Spanned<Expression>>,
        rhs: Box<Spanned<Expression>>,
    },
    Unary {
        operator: UnaryOperator,
        rhs: Box<Spanned<Expression>>,
    },
    FunctionApplication {
        function: Box<Spanned<Expression>>,
        argument: Box<Spanned<Expression>>,
    },
    Variable(Identifier),
    Primary(Spanned<Primary>),
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub enum UnaryOperator {
    Not,
    Negative,
}


#[derive(Debug, Clone)]
pub enum Primary {
    Lambda {
        param: Identifier,
        body: Box<Spanned<Expression>>,
    },
    String(String),
    Int(f64),
    Bool(bool),
    Color(palette::Xyza<palette::white_point::D65, f64>),
    Grouping(Box<Spanned<Expression>>),
    Unit,
}
impl Primary {
    pub fn is_same_type(primary_1: &Primary, primary_2: &Primary) -> bool {
        match (primary_1, primary_2) {
            (Primary::Bool(_), Primary::Bool(_)) => true,
            (Primary::Color(_), Primary::Color(_)) => true,
            (Primary::Int(_), Primary::Int(_)) => true,
            (Primary::String(_), Primary::String(_)) => true,
            (Primary::Lambda { .. }, Primary::Lambda { .. }) => false,
            (Primary::Unit, Primary::Unit) => true,
            _ => false,
        }
    }
}
impl PartialEq for Primary {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Primary::Bool(l), Primary::Bool(r)) => l == r,
            (Primary::Color(l), Primary::Color(r)) => l == r,
            (Primary::Int(l), Primary::Int(r)) => l == r,
            (Primary::String(l), Primary::String(r)) => l == r,
            (Primary::Unit, Primary::Unit) => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Identifier {
    pub handle: u64,
}
pub struct IdentifierMap {
    pub map: std::collections::HashMap<u64, String>,
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
                (hasher.finish(), name)
            })
            .collect(),
        }
    }
    pub fn create_identifier(&mut self, name: String) -> Result<Identifier, Identifier> {
        let mut hasher = DefaultHasher::new();
        name.hash(&mut hasher);
        let handle = hasher.finish();
        if self.map.get(&handle).is_some() {
            return Err(Identifier { handle });
        }
        self.map.insert(handle, name);
        Ok(Identifier { handle })
    }
    pub fn get(&self, identifier: &Identifier) -> Option<&String> {
        self.map.get(&identifier.handle)
    }
    pub fn get_from_name(&self, name: &str) -> Option<Identifier> {
        self.map
            .iter()
            .find(|(_, value)| value == &name)
            .map(|(key, _)| Identifier { handle: *key })
    }
}

pub trait UnwrapSameTypes<T> {
    fn always_ok(self) -> T;
}
impl<T> UnwrapSameTypes<T> for Result<T, T> {
    fn always_ok(self) -> T {
        match self {
            Ok(inner) | Err(inner) => inner
        }
    }
}