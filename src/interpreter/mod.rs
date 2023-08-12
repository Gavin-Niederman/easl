use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

use crate::parser::ast::{Expression, Primary, Statement, ComparisonOperator, ExpressionType, PrimaryType, BinaryOperator};

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
                _ => Err(InterpreterError::IfConditionWrongType {
                    source_code: source.to_string(),
                    this_if: expression.span,
                    this_condition: expression.span,
                }),
            }
        }
        ExpressionType::Binary { operator, lhs, rhs } => {
            let lhs = interpret_expression(*lhs, source)?;
            let rhs = interpret_expression(*rhs, source)?;

            if Primary::is_same_type(&lhs, &rhs) {
                return Err(InterpreterError::BinaryOperandMismatch {
                    source_code: source.to_string(),
                    this_binary: expression.span,
                    this_lhs: lhs.span,
                    this_rhs: rhs.span,
                })
            }

            Ok(match operator {
                BinaryOperator::Equivalent => {
                    Primary { primary_type: PrimaryType::Bool(lhs == rhs), span: expression.span }
                },
                BinaryOperator::NotEquivalent => {
                    Primary { primary_type: PrimaryType::Bool(lhs != rhs), span: expression.span }
                },
                BinaryOperator::GreaterThan => {
                    let (PrimaryType::Int(lhs), PrimaryType::Int(rhs)) = (lhs.primary_type, rhs.primary_type) else {
                        todo!()
                    };
                    Primary { primary_type: PrimaryType::Bool(lhs > rhs), span: expression.span }
                },
                BinaryOperator::LessThan => {
                    let (PrimaryType::Int(lhs), PrimaryType::Int(rhs)) = (lhs.primary_type, rhs.primary_type) else {
                        todo!()
                    };
                    Primary { primary_type: PrimaryType::Bool(lhs < rhs), span: expression.span }
                },
                BinaryOperator::GreaterThanOrEqual=> {
                    let (PrimaryType::Int(lhs), PrimaryType::Int(rhs)) = (lhs.primary_type, rhs.primary_type) else {
                        todo!()
                    };
                    Primary { primary_type: PrimaryType::Bool(lhs >= rhs), span: expression.span }
                },
                BinaryOperator::LessThanOrEqual=> {
                    let (PrimaryType::Int(lhs), PrimaryType::Int(rhs)) = (lhs.primary_type, rhs.primary_type) else {
                        todo!()
                    };
                    Primary { primary_type: PrimaryType::Bool(lhs <= rhs), span: expression.span }
                },

                BinaryOperator::Add => {
                    
                },
                BinaryOperator::Sub => {

                },

                BinaryOperator::Mul => {

                },
                BinaryOperator::Div => {

                },
                BinaryOperator::Remainder => {

                },
            })
        }
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
    #[error("If condition did not evaluate to a boolean")]
    #[diagnostic(code(easl::interpreter::if_condition_not_bool), help = "Make sure your if condition evaluates to a boolean")]
    IfConditionWrongType {
        #[source_code]
        source_code: String,
        #[label("In this if")]
        this_if: SourceSpan,
        #[label("This condition")]
        this_condition: SourceSpan,
    },
    #[error("Mismatched binary operand types")]
    #[diagnostic(code(easl::interpreter::binary_operand_mismatch), help = "Make sure that both operands are the same type")]
    BinaryOperandMismatch {
        #[source_code]
        source_code: String,
        #[label("In this binary operation")]
        this_binary: SourceSpan,
        #[label("This operand")]
        this_lhs: SourceSpan,
        #[label("This operand")]
        this_rhs: SourceSpan,
    },
    #[error("Attempted to negate non boolean")]
    #[diagnostic(code(easl::interpreter::negated_wrong_type), help = "Make sure that you are actually trying to negate a bool")]
    NegatedWrongType {
        #[source_code]
        source_code: String,
        #[label("In this negate operation")]
        this_op: SourceSpan,
        #[label("This isn't a boolean")]
        this_expr: SourceSpan
    }
}