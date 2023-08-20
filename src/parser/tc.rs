use miette::Diagnostic;
use rusttyc::{Variant, Arity, TcVar, Partial};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Infer,
    String,
    Int,
    Color,
    Bool,
    Unit,
    Array(Box<Type>),
    Fun { input: Box<Type>, output: Box<Type> },
}

impl Variant for Type {
    type Err = TypeCheckerError;
    fn arity(&self) -> Arity {
        match self {
            Self::Fun { .. } => Arity::Fixed(2),
            Self::Array(_) => Arity::Fixed(1),
            _ => Arity::Fixed(0)
        }
    }

    fn top() -> Self {
        Self::Infer
    }

    fn meet(lhs: Partial<Self>, rhs: Partial<Self>) -> Result<Partial<Self>, Self::Err> {
        let variant = match (lhs.variant, rhs.variant) {
            (Self::Infer, x) | (x, Self::Infer) => Ok(x),
            (Self::String, Self::String) => Ok(Self::String),
            (Self::Int, Self::Int) => Ok(Self::Int),
            (Self::Color, Self::Color) => Ok(Self::Color),
            (Self::Bool, Self::Bool) => Ok(Self::Bool),
            (Self::Unit, Self::Unit) => Ok(Self::Unit),
            (Self::Array(_), Self::Array(_)) => todo!(),
            (Self::Fun { .. }, Self::Fun { .. }) => Err(TypeCheckerError::MetFunctions),
            (lhs, rhs) => Err(TypeCheckerError::IncompatibleTypes { lhs, rhs })
        }?;

        Ok(Partial { variant, least_arity: 0 })
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct TypeVariable(usize);
impl TcVar for TypeVariable {}

#[derive(Debug, Error, Diagnostic)]
pub enum TypeCheckerError {
    #[error("Met functions")]
    #[diagnostic(code(easl::parser::tc::met_functions), help = "All functions have different types no matter what the argument types are.")]
    MetFunctions,
    #[error("Incompatible types")]
    #[diagnostic(code(easl::parser::tc::incompatible_types), help("Tried to operate on {lhs:?} and {rhs:?}"))]
    IncompatibleTypes {
        lhs: Type,
        rhs: Type,
    }
}