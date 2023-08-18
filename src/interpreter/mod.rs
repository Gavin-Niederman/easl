use std::collections::HashMap;

use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

use crate::parser::ast::{
    BinaryOperator, Expression, IdentifierMap, Primary, PrimaryType, Statement,
    UnaryOperator, Identifier, Type, Spanned,
};

pub struct InterpreterState {
    pub ident_map: IdentifierMap,

    pub value_map: HashMap<Identifier, Primary>,
    pub type_map: HashMap<Identifier, Type>,
}

pub fn interpret(
    statements: Vec<Statement>,
    source: &str,
    ident_map: IdentifierMap,
) -> Result<(), InterpreterError> {
    let mut state = InterpreterState { ident_map, value_map: HashMap::new(), type_map: HashMap::new() };
    for statement in statements {
        interpret_statement(statement, source, &mut state)?;
    }
    Ok(())
}

fn execute(state: InterpreterState, position: i64) -> Result<(), InterpreterError> {
    todo!("write the code that actually sets leds");
    Ok(())
}

fn interpret_statement(
    statement: Statement,
    source: &str,
    state: &mut InterpreterState,
) -> Result<(), InterpreterError> {
    match statement {
        Statement::Assignment { ident, expr } => {
            let expr = interpret_expression(expr, source)?;
            state.value_map.insert(ident, expr);
        }
        Statement::TypeAscription { ident, type_ } => {
            state.type_map.insert(ident, type_);
        }
        Statement::Include { .. } => {},
        Statement::EOI => (),
    }

    

    Ok(())
}

fn interpret_expression(expression: Spanned<Expression>, source: &str) -> Result<Primary, InterpreterError> {
    match expression.inner {
        Expression::If { cond, then, else_ } => {
            let cond = interpret_expression(*cond, source)?;
            let then = interpret_expression(*then, source)?;
            let else_ = interpret_expression(*else_, source)?;

            match cond.primary_type {
                PrimaryType::Bool(true) => Ok(then),
                PrimaryType::Bool(false) => Ok(else_),
                _ => Err(InterpreterError::IfConditionWrongType {
                    source_code: source.to_string(),
                    this_if: expression.span.into(),
                    this_condition: cond.span.into(),
                }),
            }
        }
        Expression::Binary { operator, lhs, rhs } => {
            let lhs = interpret_expression(*lhs, source)?;
            let rhs = interpret_expression(*rhs, source)?;

            if Primary::is_same_type(&lhs, &rhs) {
                return Err(InterpreterError::BinaryOperandMismatch {
                    source_code: source.to_string(),
                    this_binary: expression.span.into(),
                    this_lhs: lhs.span,
                    this_rhs: rhs.span,
                });
            }

            Ok(match operator {
                BinaryOperator::Equivalent => Primary {
                    primary_type: PrimaryType::Bool(lhs == rhs),
                    span: expression.span.into(),
                },
                BinaryOperator::NotEquivalent => Primary {
                    primary_type: PrimaryType::Bool(lhs != rhs),
                    span: expression.span.into(),
                },
                BinaryOperator::GreaterThan => {
                    let (PrimaryType::Int(lhs), PrimaryType::Int(rhs)) = (lhs.primary_type, rhs.primary_type) else {
                        todo!()
                    };
                    Primary {
                        primary_type: PrimaryType::Bool(lhs > rhs),
                        span: expression.span.into(),
                    }
                }
                BinaryOperator::LessThan => {
                    let (PrimaryType::Int(lhs), PrimaryType::Int(rhs)) = (lhs.primary_type, rhs.primary_type) else {
                        todo!()
                    };
                    Primary {
                        primary_type: PrimaryType::Bool(lhs < rhs),
                        span: expression.span.into(),
                    }
                }
                BinaryOperator::GreaterThanOrEqual => {
                    let (PrimaryType::Int(lhs), PrimaryType::Int(rhs)) = (lhs.primary_type, rhs.primary_type) else {
                        todo!()
                    };
                    Primary {
                        primary_type: PrimaryType::Bool(lhs >= rhs),
                        span: expression.span.into(),
                    }
                }
                BinaryOperator::LessThanOrEqual => {
                    let (PrimaryType::Int(lhs), PrimaryType::Int(rhs)) = (lhs.primary_type, rhs.primary_type) else {
                        todo!()
                    };
                    Primary {
                        primary_type: PrimaryType::Bool(lhs <= rhs),
                        span: expression.span.into(),
                    }
                }

                BinaryOperator::Add => match (lhs.primary_type, rhs.primary_type) {
                    (PrimaryType::Int(lhs), PrimaryType::Int(rhs)) => Primary {
                        primary_type: PrimaryType::Int(lhs + rhs),
                        span: expression.span.into(),
                    },
                    (PrimaryType::String(lhs), PrimaryType::String(rhs)) => Primary {
                        primary_type: PrimaryType::String(lhs + &rhs),
                        span: expression.span.into(),
                    },
                    _ => todo!(),
                },
                BinaryOperator::Sub => {
                    let (PrimaryType::Int(lhs), PrimaryType::Int(rhs)) = (lhs.primary_type, rhs.primary_type) else {
                        todo!()
                    };
                    Primary {
                        primary_type: PrimaryType::Int(lhs - rhs),
                        span: expression.span.into(),
                    }
                }

                BinaryOperator::Mul => {
                    let (PrimaryType::Int(lhs), PrimaryType::Int(rhs)) = (lhs.primary_type, rhs.primary_type) else {
                        todo!()
                    };
                    Primary {
                        primary_type: PrimaryType::Int(lhs * rhs),
                        span: expression.span.into(),
                    }
                }
                BinaryOperator::Div => {
                    let (PrimaryType::Int(lhs), PrimaryType::Int(rhs)) = (lhs.primary_type, rhs.primary_type) else {
                        todo!()
                    };
                    Primary {
                        primary_type: PrimaryType::Int(lhs / rhs),
                        span: expression.span.into(),
                    }
                }
                BinaryOperator::Remainder => {
                    let (PrimaryType::Int(lhs), PrimaryType::Int(rhs)) = (lhs.primary_type, rhs.primary_type) else {
                        todo!()
                    };
                    Primary {
                        primary_type: PrimaryType::Int(lhs % rhs),
                        span: expression.span.into(),
                    }
                }
            })
        }
        Expression::FunctionApplication { function, argument } => {
            let function = interpret_expression(*function, source)?;
            let argument = interpret_expression(*argument, source)?;

            todo!()
        }
        Expression::Unary { operator, rhs } => {
            let rhs = interpret_expression(*rhs, source)?;

            Ok(match operator {
                UnaryOperator::Negative => match rhs.primary_type {
                    PrimaryType::Int(rhs) => Primary {
                        primary_type: PrimaryType::Int(-rhs),
                        span: expression.span.into(),
                    },
                    _ => todo!(),
                },
                UnaryOperator::Not => match rhs.primary_type {
                    PrimaryType::Bool(rhs) => Primary {
                        primary_type: PrimaryType::Bool(!rhs),
                        span: expression.span.into(),
                    },
                    _ => todo!(),
                },
            })
        }
        Expression::Variable(identifier) => todo!(),
        Expression::Primary(primary) => Ok(primary),
    }
}

#[derive(Debug, Error, Diagnostic)]
pub enum InterpreterError {
    #[error("If condition did not evaluate to a boolean")]
    #[diagnostic(
        code(easl::interpreter::if_condition_not_bool),
        help = "Make sure your if condition evaluates to a boolean"
    )]
    IfConditionWrongType {
        #[source_code]
        source_code: String,
        #[label("In this if")]
        this_if: SourceSpan,
        #[label("This condition")]
        this_condition: SourceSpan,
    },
    #[error("Mismatched binary operand types")]
    #[diagnostic(
        code(easl::interpreter::binary_operand_mismatch),
        help = "Make sure that both operands are the same type"
    )]
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
    #[diagnostic(
        code(easl::interpreter::negated_wrong_type),
        help = "Make sure that you are actually trying to negate a bool"
    )]
    NegatedWrongType {
        #[source_code]
        source_code: String,
        #[label("In this negate operation")]
        this_op: SourceSpan,
        #[label("This isn't a boolean")]
        this_expr: SourceSpan,
    },
}
