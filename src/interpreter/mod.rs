use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

use crate::parser::ast::{Expression, Primary, Statement, ComparisonOperator, ExpressionType, PrimaryType};

pub struct InterpreterState {

}

pub fn interpret(ast: Vec<Statement>, source: &str) -> Result<(), InterpreterError> {

    Ok(())
}

pub fn interpret_expression(expression: Expression, source: &str) -> Result<Primary, InterpreterError> {
    match expression.expression_type {
        ExpressionType::If { cond, then, else_ } => {
            let cond = interpret_expression(*cond, source)?;
            let then = interpret_expression(*then, source)?;
            let else_ = interpret_expression(*else_, source)?;

            match cond.primary_type {
                PrimaryType::Bool(true) => Ok(then),
                PrimaryType::Bool(false) => Ok(else_),
                _ => Err(InterpreterError::IfConditionNotBool {
                    source_code: source.to_string(),
                    this_if: expression.span,
                    this_condition: expression.span,
                }),
            }
        }
        ExpressionType::Comparison { operator, lhs, rhs } => {
            let lhs = interpret_expression(*lhs, source)?;
            let rhs = interpret_expression(*rhs, source)?;

            todo!()
        },
        ExpressionType::Term { operator, lhs, rhs } => todo!(),
        ExpressionType::Factor { operator, lhs, rhs } => todo!(),
        ExpressionType::Unary { operator, rhs } => {
            let rhs = interpret_expression(*rhs, source)?;
            Ok(match operator {
                crate::parser::ast::UnaryOperator::Negate => {
                    
                },
                crate::parser::ast::UnaryOperator::Not => {
                    match rhs.primary_type {
                        PrimaryType::Bool(boolean) => { Primary { primary_type: PrimaryType::Bool(!boolean), span: expression.span } }
                    }
                },
                crate::parser::ast::UnaryOperator::Negative => todo!(),
            })
        },
        ExpressionType::FunctionApplication { function, argument } => {
            let function = interpret_expression(*function, source)?;
            let argument = interpret_expression(*argument, source)?;

            todo!()
        },
        ExpressionType::Identifier(identifier) => todo!(),
        ExpressionType::Primary(primary) => Ok(primary),
    }
}

#[derive(Debug, Error, Diagnostic)]
pub enum InterpreterError {
    #[error("If condition must evaluate to a boolean")]
    #[diagnostic(code(easl::interpreter::if_condition_not_bool), help = "Make sure your if condition evaluates to a boolean")]
    IfConditionNotBool {
        #[source_code]
        source_code: String,
        #[label("In this if")]
        this_if: SourceSpan,
        #[label("This condition")]
        this_condition: SourceSpan,
    },
}