use std::{collections::HashMap, sync::Mutex, rc::Rc};

use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

use crate::parser::{
    ast::{
        BinaryOperator, Expression, Identifier, IdentifierMap, Primary, Spanned, Statement,
        UnaryOperator,
    },
    tc::Type,
};

pub struct InterpreterState {
    pub ident_map: Rc<Mutex<IdentifierMap>>,

    pub value_map: HashMap<Identifier, Primary>,
    pub type_map: HashMap<Identifier, Type>,
}

pub fn interpret(
    statements: Vec<Spanned<Statement>>,
    source: &str,
    ident_map: Rc<Mutex<IdentifierMap>>,
) -> Result<(), InterpreterError> {
    let mut state = InterpreterState {
        ident_map,
        value_map: HashMap::new(),
        type_map: HashMap::new(),
    };
    for statement in statements {
        interpret_statement(statement, source, &mut state)?;
    }
    Ok(())
}

fn execute(_state: InterpreterState, _position: i64) -> Result<(), InterpreterError> {
    todo!("write the code that actually sets leds");
    Ok(())
}

fn interpret_statement(
    statement: Spanned<Statement>,
    source: &str,
    state: &mut InterpreterState,
) -> Result<(), InterpreterError> {
    match statement.inner {
        Statement::Assignment { ident, expr } => {
            let expr = interpret_expression(expr, source)?;
            state.value_map.insert(ident, expr.inner);
        }
        Statement::TypeAscription { ident, type_ } => {
            state.type_map.insert(ident, type_);
        }
    }

    Ok(())
}

fn interpret_expression(
    expression: Spanned<Expression>,
    source: &str,
) -> Result<Spanned<Primary>, InterpreterError> {
    match expression.inner {
        Expression::If { cond, then, else_ } => {
            let cond = interpret_expression(*cond, source)?;
            let then = interpret_expression(*then, source)?;
            let else_ = interpret_expression(*else_, source)?;

            match cond.inner {
                Primary::Bool(true) => Ok(then),
                Primary::Bool(false) => Ok(else_),
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

            if Primary::is_same_type(&lhs.inner, &rhs.inner) {
                return Err(InterpreterError::BinaryOperandMismatch {
                    source_code: source.to_string(),
                    this_binary: expression.span.into(),
                    this_lhs: lhs.span.into(),
                    this_rhs: rhs.span.into(),
                });
            }

            Ok(match operator {
                BinaryOperator::Equivalent => Spanned {
                    inner: Primary::Bool(lhs == rhs),
                    span: expression.span,
                },
                BinaryOperator::NotEquivalent => Spanned {
                    inner: Primary::Bool(lhs != rhs),
                    span: expression.span,
                },
                BinaryOperator::GreaterThan => {
                    let (Primary::Int(lhs), Primary::Int(rhs)) = (lhs.inner, rhs.inner) else {
                        todo!()
                    };
                    Spanned {
                        inner: Primary::Bool(lhs > rhs),
                        span: expression.span,
                    }
                }
                BinaryOperator::LessThan => {
                    let (Primary::Int(lhs), Primary::Int(rhs)) = (lhs.inner, rhs.inner) else {
                        todo!()
                    };
                    Spanned {
                        inner: Primary::Bool(lhs < rhs),
                        span: expression.span,
                    }
                }
                BinaryOperator::GreaterThanOrEqual => {
                    let (Primary::Int(lhs), Primary::Int(rhs)) = (lhs.inner, rhs.inner) else {
                        todo!()
                    };
                    Spanned {
                        inner: Primary::Bool(lhs >= rhs),
                        span: expression.span,
                    }
                }
                BinaryOperator::LessThanOrEqual => {
                    let (Primary::Int(lhs), Primary::Int(rhs)) = (lhs.inner, rhs.inner) else {
                        todo!()
                    };
                    Spanned {
                        inner: Primary::Bool(lhs <= rhs),
                        span: expression.span,
                    }
                }

                BinaryOperator::Add => match (lhs.inner, rhs.inner) {
                    (Primary::Int(lhs), Primary::Int(rhs)) => Spanned {
                        inner: Primary::Int(lhs + rhs),
                        span: expression.span,
                    },
                    (Primary::String(lhs), Primary::String(rhs)) => Spanned {
                        inner: Primary::String(lhs + &rhs),
                        span: expression.span,
                    },
                    _ => todo!(),
                },
                BinaryOperator::Sub => {
                    let (Primary::Int(lhs), Primary::Int(rhs)) = (lhs.inner, rhs.inner) else {
                        todo!()
                    };
                    Spanned {
                        inner: Primary::Int(lhs - rhs),
                        span: expression.span,
                    }
                }

                BinaryOperator::Mul => {
                    let (Primary::Int(lhs), Primary::Int(rhs)) = (lhs.inner, rhs.inner) else {
                        todo!()
                    };
                    Spanned {
                        inner: Primary::Int(lhs * rhs),
                        span: expression.span,
                    }
                }
                BinaryOperator::Div => {
                    let (Primary::Int(lhs), Primary::Int(rhs)) = (lhs.inner, rhs.inner) else {
                        todo!()
                    };
                    Spanned {
                        inner: Primary::Int(lhs / rhs),
                        span: expression.span,
                    }
                }
                BinaryOperator::Remainder => {
                    let (Primary::Int(lhs), Primary::Int(rhs)) = (lhs.inner, rhs.inner) else {
                        todo!()
                    };
                    Spanned {
                        inner: Primary::Int(lhs % rhs),
                        span: expression.span,
                    }
                }
            })
        }
        Expression::FunctionApplication { function, argument } => {
            let _function = interpret_expression(*function, source)?;
            let _argument = interpret_expression(*argument, source)?;

            todo!()
        }
        Expression::Unary { operator, rhs } => {
            let rhs = interpret_expression(*rhs, source)?;

            Ok(match operator {
                UnaryOperator::Negative => match rhs.inner {
                    Primary::Int(rhs) => Spanned {
                        inner: Primary::Int(-rhs),
                        span: expression.span,
                    },
                    _ => todo!(),
                },
                UnaryOperator::Not => match rhs.inner {
                    Primary::Bool(rhs) => Spanned {
                        inner: Primary::Bool(!rhs),
                        span: expression.span,
                    },
                    _ => todo!(),
                },
            })
        }
        Expression::Variable(_identifier) => todo!(),
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
