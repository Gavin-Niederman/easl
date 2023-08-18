pub mod ast;

use miette::{Diagnostic, SourceSpan};
// use palette::{FromColor, Xyza};
use pest::{
    iterators::{Pair, Pairs},
    Parser,
};
use pest_derive::Parser;

use thiserror::Error;

use crate::{
    parser::ast::{BinaryOperator, Expression, Primary, UnaryOperator},
    utils::pest_span_to_range,
};

use self::ast::{IdentifierMap, Statement, Type, Spanned};

#[derive(Parser)]
#[grammar = "parser/easl.pest"]
pub struct EaslParser;

pub fn parse(source: &str) -> Result<(Vec<Statement>, IdentifierMap), ParserError> {
    let mut ident_map = IdentifierMap::new();
    Ok((
        build_ast(
            EaslParser::parse(Rule::file, source)?,
            source,
            &mut ident_map,
        )?,
        ident_map,
    ))
}

fn build_ast(
    mut pairs: Pairs<'_, Rule>,
    source: &str,
    ident_map: &mut IdentifierMap,
) -> Result<Vec<Statement>, ParserError> {
    let mut ast = Vec::new();

    let Some(file) = pairs.next() else {
        return Ok(ast);
    };
    let statements = file.into_inner();

    for statement in statements {
        ast.push(build_statement(statement, source, ident_map)?);
    }

    Ok(ast)
}

fn build_statement(
    statement: Pair<'_, Rule>,
    source: &str,
    ident_map: &mut IdentifierMap,
) -> Result<Statement, ParserError> {
    let mut inner = statement.clone().into_inner();
    match statement.as_rule() {
        Rule::statement => build_statement(inner.next().unwrap(), source, ident_map),
        Rule::assignment => {
            let ident = ident_map
                .create_identifier(inner.next().unwrap().to_string())
                .or(Err(ParserError::OverridenIdentifier {
                    source_code: source.to_string(),
                    second_assignment: pest_span_to_range(statement.as_span()).into(),
                }))?;
            let expr = build_expression(inner.next().unwrap(), source, ident_map)?;

            Ok(Statement::Assignment { ident, expr })
        }
        Rule::type_ascription => {
            let ident = inner.next().unwrap();
            let ident = match ident_map.create_identifier(ident.as_str().to_string()) {
                Ok(ident) | Err(ident) => ident
            };

            let type_ = build_type(inner.next().unwrap(), source)?;
            Ok(Statement::TypeAscription { ident, type_ })
        }
        Rule::include => Ok(Statement::Include {
            source: inner.next().unwrap().to_string(),
        }),
        Rule::EOI => Ok(Statement::EOI),
        _ => {
            return Err(ParserError::internal_grammar_error(
                source,
                statement.as_span(),
            ))
        }
    }
}

fn build_expression(
    expression: Pair<'_, Rule>,
    source: &str,
    ident_map: &mut IdentifierMap,
) -> Result<Spanned<Expression>, ParserError> {
    let mut inner = expression.clone().into_inner();
    macro_rules! build_next {
        () => {
            build_expression(inner.next().unwrap(), source, ident_map)?
        };
    }
    macro_rules! unless_1_inner {
        ($block:block) => {
            if inner.len() == 1 {
                return Ok(build_next!());
            } else $block
        };
    }

    Ok(Spanned::new(
        pest_span_to_range(expression.as_span()),
        match expression.as_rule() {
        Rule::expression => return Ok(build_next!()),
        Rule::r#if => unless_1_inner!({
            let cond = Box::new(build_next!());
            let then = Box::new(build_next!());
            let else_ = Box::new(build_next!());
            Expression::If { cond, then, else_ }
        }),
        Rule::comparison => unless_1_inner!({
            let lhs = Box::new(build_next!());
            let operator = match inner.next().unwrap().as_rule() {
                Rule::equivalent => BinaryOperator::Equivalent,
                Rule::not_equivalent => BinaryOperator::NotEquivalent,
                Rule::less_than => BinaryOperator::LessThan,
                Rule::less_than_or_eq => BinaryOperator::LessThanOrEqual,
                Rule::greater_than => BinaryOperator::GreaterThan,
                Rule::greater_than_or_eq => BinaryOperator::GreaterThanOrEqual,
                _ => {
                    return Err(ParserError::internal_grammar_error(
                        source,
                        expression.as_span(),
                    ))
                }
            };
            let rhs = Box::new(build_next!());
            Expression::Binary { lhs, operator, rhs }
        }),
        Rule::term => unless_1_inner!({
            let lhs = Box::new(build_next!());
            let operator = match inner.next().unwrap().as_rule() {
                Rule::add => BinaryOperator::Add,
                Rule::sub => BinaryOperator::Sub,
                _ => {
                    return Err(ParserError::internal_grammar_error(
                        source,
                        expression.as_span(),
                    ))
                }
            };
            let rhs = Box::new(build_next!());
            Expression::Binary { lhs, operator, rhs }
        }),
        Rule::factor => unless_1_inner!({
            let lhs = Box::new(build_next!());
            let operator = match inner.next().unwrap().as_rule() {
                Rule::mul => BinaryOperator::Mul,
                Rule::div => BinaryOperator::Div,
                _ => {
                    return Err(ParserError::internal_grammar_error(
                        source,
                        expression.as_span(),
                    ))
                }
            };
            let rhs = Box::new(build_next!());
            Expression::Binary { lhs, operator, rhs }
        }),
        Rule::unary => unless_1_inner!({
            let operator = match inner.next().unwrap().as_rule() {
                Rule::not => UnaryOperator::Not,
                Rule::negative => UnaryOperator::Negative,
                _ => {
                    return Err(ParserError::internal_grammar_error(
                        source,
                        expression.as_span(),
                    ))
                }
            };
            let rhs = Box::new(build_next!());
            Expression::Unary { operator, rhs }
        }),
        Rule::function_application => unless_1_inner!({
            let function = Box::new(build_next!());
            let argument = Box::new(build_next!());
            Expression::FunctionApplication { function, argument }
        }),
        Rule::variable => {
            println!("{:?}", expression.as_span());
            let next = inner.next().unwrap();
            match next.as_rule() {
                Rule::ident => {
                    Expression::Variable(ident_map.get_from_name(expression.as_str()).ok_or(
                        ParserError::UnknownIdentifier {
                            source_code: source.to_string(),
                            ident: next.as_str().to_string(),
                            unknown_identifier: pest_span_to_range(expression.as_span()).into(),
                        },
                    )?)
                }
                _ => {
                    return Ok(build_expression(next, source, ident_map)?)
                },
            }
        }
        Rule::primary => return Ok(build_next!()),
        Rule::literal => return Ok(build_next!()),
        Rule::lambda => unless_1_inner!({
            println!("{:?}", inner);
            println!("{:?}", expression.as_span());
            println!("{:?}", expression.as_rule());
            let param = inner.next().unwrap().as_str().to_string();
            let param = match ident_map.create_identifier(param) {
                Ok(param) | Err(param) => param
            };
            let span = pest_span_to_range(expression.as_span()).into();
            let body = Box::new(build_next!());
            Expression::Primary(Spanned {
                inner: Primary::Lambda { param, body },
                span,
            })
        }),
        Rule::int_l => Expression::Primary(Spanned {
            inner: Primary::Int(match inner.next().unwrap().as_rule() {
                Rule::hex_int => i64::from_str_radix(expression.as_str(), 16).unwrap() as f64,
                Rule::binary_int => i64::from_str_radix(expression.as_str(), 2).unwrap() as f64,
                Rule::decimal_int => expression.as_str().parse::<f64>().unwrap(),
                _ => {
                    return Err(ParserError::internal_grammar_error(
                        source,
                        expression.as_span(),
                    ))
                }
            }),
            span: pest_span_to_range(expression.as_span()).into(),
        }),
        Rule::string_l => Expression::Primary(Spanned {
            inner: Primary::String(expression.as_str().to_string()),
            span: pest_span_to_range(expression.as_span()).into(),
        }),
        Rule::bool_l => Expression::Primary(Spanned {
            inner: Primary::Bool(match inner.next().unwrap().as_rule() {
                Rule::r#true => true,
                Rule::r#false => false,
                _ => {
                    return Err(ParserError::internal_grammar_error(
                        source,
                        expression.as_span(),
                    ))
                }
            }),
            span: pest_span_to_range(expression.as_span()).into(),
        }),
        Rule::grouping => return Ok(build_next!()),
        Rule::unit_l => Expression::Primary(Spanned {
            inner: Primary::Unit,
            span: pest_span_to_range(expression.as_span()).into(),
        }),
        _ => {
            return Err(ParserError::internal_grammar_error(
                source,
                expression.as_span(),
            ))
        }
    }))
}

fn build_type(type_: Pair<'_, Rule>, source: &str) -> Result<Type, ParserError> {
    let mut inner = type_.clone().into_inner();
    macro_rules! build_next {
        () => {
            build_type(inner.next().unwrap(), source)?
        };
    }
    Ok(match type_.as_rule() {
        Rule::type_annotation => build_next!(),
        Rule::r#type => build_next!(),
        Rule::base_type => build_next!(),
        Rule::fun_t => {
            let input = Box::new(build_next!());
            let output = Box::new(build_next!());
            Type::Fun { input, output }
        }
        Rule::string_t => Type::String,
        Rule::int_t => Type::Int,
        Rule::color_t => Type::Color,
        Rule::bool_t => Type::Bool,
        Rule::unit_t => Type::Unit,
        Rule::array_t => Type::Array(Box::new(build_next!())),
        _ => return Err(ParserError::internal_grammar_error(source, type_.as_span())),
    })
}

#[derive(Debug, Error, Diagnostic)]
pub enum ParserError {
    #[error(transparent)]
    #[diagnostic(code(easl::parser::pest::pest_error))]
    PestError(#[from] pest::error::Error<Rule>),
    #[error("Internal grammar error")]
    #[diagnostic(
        code(easl::parser::internal_grammar_error),
        help("Your code is fine, ours isn't.\nPlease create a github issue to report this error"),
        url("https://github.com/gavin-niederman/easl/issues/new")
    )]
    InternalGrammarError {
        #[source_code]
        source_code: String,
        #[label("Internal grammar error")]
        at: SourceSpan,
    },
    #[error("Identifier defined multiple times")]
    #[diagnostic(
        code(easl::parser::overriden_identifier),
        help = "Remove one of the definitions"
    )]
    OverridenIdentifier {
        #[source_code]
        source_code: String,
        #[label("Identifier was assigned again here")]
        second_assignment: SourceSpan,
    },
    #[error("Unknown identifier '{ident}'")]
    #[diagnostic(code(easl::parser::unknown_identifier), help = "Was this a typo?")]
    UnknownIdentifier {
        #[source_code]
        source_code: String,
        ident: String,
        #[label("Unknown identifier")]
        unknown_identifier: SourceSpan,
    },
}

impl ParserError {
    fn internal_grammar_error(source_code: &str, span: pest::Span<'_>) -> Self {
        let span = miette::SourceSpan::new(span.start().into(), (span.end() - span.start()).into());
        Self::InternalGrammarError {
            source_code: source_code.to_string(),
            at: span,
        }
    }
}
