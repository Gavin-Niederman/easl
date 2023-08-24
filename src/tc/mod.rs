pub mod variant;

use miette::Diagnostic;
use rusttyc::{TcVar, TypeChecker, TcErr};
use thiserror::Error;

use variant::Type;

use rusttyc::TypeTable;
use crate::parser::ast::Statement;

pub fn check(statements: Vec<Statement>) -> Result<TypeTable<Type>, TypeCheckerError> {
    let tc: TypeChecker<Type, TypeVariable> = TypeChecker::new();

    Ok(tc.type_check().map_err(|err| <TcErr<Type> as Into<TypeCheckerError>>::into(err))?)
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct TypeVariable(usize);
impl TcVar for TypeVariable {}

#[derive(Debug, Error, Diagnostic)]
pub enum TypeCheckerError {
    #[error("Type checker error")]
    #[diagnostic(code(easl::tc::rusttyc))]
    RusttycError(TcErr<Type>),
    #[error("Met functions")]
    #[diagnostic(code(easl::tc::met_functions), help = "All functions have different types no matter what the argument types are.")]
    MetFunctions,
    #[error("Incompatible types")]
    #[diagnostic(code(easl::tc::incompatible_types), help("Tried to operate on {lhs:?} and {rhs:?}"))]
    IncompatibleTypes {
        lhs: Type,
        rhs: Type,
    }
}
impl From<TcErr<Type>> for TypeCheckerError {
    fn from(value: TcErr<Type>) -> Self {
        match value {
            TcErr::KeyEquation(_, _, _) => todo!(),
            TcErr::Bound(_, _, _) => todo!(),
            TcErr::ChildAccessOutOfBound(_, _, _) => todo!(),
            TcErr::ArityMismatch { key, variant, inferred_arity, reported_arity } => todo!(),
            TcErr::Construction(_, _, _) => todo!(),
            TcErr::ChildConstruction(_, _, _, _) => todo!(),
            TcErr::CyclicGraph => todo!(),
        }
    }
}