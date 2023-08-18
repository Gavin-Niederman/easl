use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    ops::Range,
};

pub struct Spanned<T> {
    pub span: Range<usize>,
    pub inner: T,
}

impl<T> Spanned<T> {
    pub fn new(span: Range<usize>, inner: T) -> Self {
        Self { span, inner }
    }
}

impl<T> Clone for Spanned<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            span: self.span.clone(),
            inner: self.inner.clone(),
        }
    }
}

impl<T> std::fmt::Debug for Spanned<T>
where
    T: std::fmt::Debug
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Spanned {{\n\tinner: {:#?}\n\tspan: {:?}\n}}", self.inner, self.span)?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum Statement {
    //TODO: Syntax for patern matching, in place type ascription, and parameters
    Assignment {
        ident: Identifier,
        // args: Vec<Node>,
        expr: Spanned<Expression>,
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
    Primary(Primary),
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
pub struct Primary {
    pub primary_type: PrimaryType,
    pub span: miette::SourceSpan,
}

impl PartialEq for Primary {
    fn eq(&self, other: &Self) -> bool {
        match (self.primary_type.clone(), other.primary_type.clone()) {
            (PrimaryType::Bool(lhs), PrimaryType::Bool(rhs)) => lhs == rhs,
            (PrimaryType::Color(lhs), PrimaryType::Color(rhs)) => lhs == rhs,
            (PrimaryType::Int(lhs), PrimaryType::Int(rhs)) => lhs == rhs,
            (PrimaryType::String(lhs), PrimaryType::String(rhs)) => lhs == rhs,
            (PrimaryType::Unit, PrimaryType::Unit) => true,
            _ => {
                panic!(
                    "Cannot compare {:?} and {:?}",
                    self.primary_type, other.primary_type
                )
            }
        }
    }
}

impl Primary {
    pub fn is_same_type(primary_1: &Primary, primary_2: &Primary) -> bool {
        match (
            primary_1.primary_type.clone(),
            primary_2.primary_type.clone(),
        ) {
            (PrimaryType::Bool(_), PrimaryType::Bool(_)) => true,
            (PrimaryType::Color(_), PrimaryType::Color(_)) => true,
            (PrimaryType::Int(_), PrimaryType::Int(_)) => true,
            (PrimaryType::String(_), PrimaryType::String(_)) => true,
            (PrimaryType::Lambda { .. }, PrimaryType::Lambda { .. }) => false,
            (PrimaryType::Unit, PrimaryType::Unit) => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone)]
pub enum PrimaryType {
    Lambda {
        param: Identifier,
        body: Box<Spanned<Expression>>,
    },
    String(String),
    Int(f64),
    Bool(bool),
    Color(palette::Xyza<palette::white_point::D65, f64>),
    Unit,
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

#[derive(Debug, Clone)]
pub enum Type {
    String,
    Int,
    Color,
    Bool,
    Unit,
    Array(Box<Type>),
    Fun { input: Box<Type>, output: Box<Type> },
}
