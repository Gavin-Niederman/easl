pub mod ast;

use miette::{Diagnostic, SourceSpan};
// use palette::{FromColor, Xyza};
use pest::{
    iterators::{Pair, Pairs},
    Parser,
};
use pest_derive::Parser;

use ast::{ComparisonOperator, Expression, FactorOperator, TermOperator, UnaryOperator};
use thiserror::Error;

use crate::{
    parser::ast::{ExpressionType, Primary, PrimaryType},
    utils::pest_span_to_miette_span,
};

use self::ast::{IdentifierMap, Statement, Type};

#[derive(Parser)]
#[grammar = "parser/easl.pest"]
pub struct EaslParser;

pub fn parse(source: &str) -> Result<Vec<Statement>, ParserError> {
    build_ast(EaslParser::parse(Rule::file, source)?, source)
}

fn build_ast(mut pairs: Pairs<'_, Rule>, source: &str) -> Result<Vec<Statement>, ParserError> {
    let mut ident_map = IdentifierMap::new();
    let mut ast = Vec::new();

    let Some(file) = pairs.next() else {
        return Ok(ast);
    };
    let statements = file.into_inner();

    for statement in statements {
        ast.push(build_statement(statement, source, &mut ident_map)?);
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
            let ident = ident_map.create_identifier(inner.next().unwrap().to_string());
            let expr = build_expression(inner.next().unwrap(), source, ident_map)?;

            Ok(Statement::Assignment { ident, expr })
        }
        Rule::type_ascription => {
            let ident = ident_map.create_identifier(inner.next().unwrap().to_string());
            let type_ = build_type(inner.next().unwrap(), source)?;
            Ok(Statement::TypeAscription { ident, type_ })
        }
        Rule::include => Ok(Statement::Include {
            source: inner.next().unwrap().to_string(),
        }),
        Rule::EOI => Ok(Statement::EOI),
        _ => ParserError::internal_grammar_error(source, statement.as_span())?,
    }
}

fn build_expression(
    expression: Pair<'_, Rule>,
    source: &str,
    ident_map: &mut IdentifierMap,
) -> Result<Expression, ParserError> {
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

    Ok(Expression {
        expression_type: match expression.as_rule() {
            Rule::expression => return Ok(build_next!()),
            Rule::r#if => unless_1_inner!({
                let cond = Box::new(build_next!());
                let then = Box::new(build_next!());
                let else_ = Box::new(build_next!());
                ExpressionType::If { cond, then, else_ }
            }),
            Rule::comparison => unless_1_inner!({
                let lhs = Box::new(build_next!());
                let operator = match inner.next().unwrap().as_rule() {
                    Rule::equivalent => ComparisonOperator::Equivalent,
                    Rule::not_equivalent => ComparisonOperator::NotEquivalent,
                    Rule::less_than => ComparisonOperator::LessThan,
                    Rule::less_than_or_eq => ComparisonOperator::LessThanOrEqual,
                    Rule::greater_than => ComparisonOperator::GreaterThan,
                    Rule::greater_than_or_eq => ComparisonOperator::GreaterThanOrEqual,
                    _ => ParserError::internal_grammar_error(source, expression.as_span())?,
                };
                let rhs = Box::new(build_next!());
                ExpressionType::Comparison { lhs, operator, rhs }
            }),
            Rule::term => unless_1_inner!({
                let lhs = Box::new(build_next!());
                let operator = match inner.next().unwrap().as_rule() {
                    Rule::add => TermOperator::Add,
                    Rule::sub => TermOperator::Sub,
                    _ => ParserError::internal_grammar_error(source, expression.as_span())?,
                };
                let rhs = Box::new(build_next!());
                ExpressionType::Term { lhs, operator, rhs }
            }),
            Rule::factor => unless_1_inner!({
                let lhs = Box::new(build_next!());
                let operator = match inner.next().unwrap().as_rule() {
                    Rule::mul => FactorOperator::Mul,
                    Rule::div => FactorOperator::Div,
                    _ => ParserError::internal_grammar_error(source, expression.as_span())?,
                };
                let rhs = Box::new(build_next!());
                ExpressionType::Factor { lhs, operator, rhs }
            }),
            Rule::unary => unless_1_inner!({
                let operator = match inner.next().unwrap().as_rule() {
                    Rule::not => UnaryOperator::Not,
                    Rule::negate => UnaryOperator::Negate,
                    Rule::negative => UnaryOperator::Negative,
                    _ => ParserError::internal_grammar_error(source, expression.as_span())?,
                };
                let rhs = Box::new(build_next!());
                ExpressionType::Unary { operator, rhs }
            }),
            Rule::function_application => unless_1_inner!({
                let function = Box::new(build_next!());
                let argument = Box::new(build_next!());
                ExpressionType::FunctionApplication { function, argument }
            }),
            Rule::primary => return Ok(build_next!()),
            Rule::literal => return Ok(build_next!()),
            Rule::lambda => unless_1_inner!({
                println!("{:?}", inner);
                println!("{:?}", expression.as_span());
                println!("{:?}", expression.as_rule());
                let param = inner.next().unwrap().to_string();
                let span = pest_span_to_miette_span(expression.as_span(), source);
                let body = Box::new(build_next!());
                span;
                ExpressionType::Primary(Primary {
                    primary_type: PrimaryType::Lambda { param, body },
                    span,
                })
            }),
            Rule::int_l => ExpressionType::Primary(Primary {
                primary_type: PrimaryType::Int(match inner.next().unwrap().as_rule() {
                    Rule::hex_int => i64::from_str_radix(expression.as_str(), 16).unwrap() as f64,
                    Rule::binary_int => i64::from_str_radix(expression.as_str(), 2).unwrap() as f64,
                    Rule::decimal_int => expression.as_str().parse::<f64>().unwrap(),
                    _ => ParserError::internal_grammar_error(source, expression.as_span())?,
                }),
                span: pest_span_to_miette_span(expression.as_span(), source),
            }),
            Rule::string_l => ExpressionType::Primary(Primary {
                primary_type: PrimaryType::String(expression.to_string()),
                span: pest_span_to_miette_span(expression.as_span(), source),
            }),
            Rule::bool_l => ExpressionType::Primary(Primary {
                primary_type: PrimaryType::Bool(match inner.next().unwrap().as_rule() {
                    Rule::r#true => true,
                    Rule::r#false => false,
                    _ => ParserError::internal_grammar_error(source, expression.as_span())?,
                }),
                span: pest_span_to_miette_span(expression.as_span(), source),
            }),
            Rule::ident => {
                ExpressionType::Identifier(ident_map.create_identifier(expression.to_string()))
            }
            Rule::grouping => return Ok(build_next!()),
            Rule::unit_l => ExpressionType::Primary(Primary {
                primary_type: PrimaryType::Unit,
                span: pest_span_to_miette_span(expression.as_span(), source),
            }),
            _ => ParserError::internal_grammar_error(source, expression.as_span())?,
        },
        span: pest_span_to_miette_span(expression.as_span(), source),
    })
}

fn build_type(pair: Pair<'_, Rule>, source: &str) -> Result<Type, ParserError> {
    let mut inner = pair.clone().into_inner();
    macro_rules! build_next {
        () => {
            build_type(inner.next().unwrap(), source)?
        };
    }
    Ok(match pair.as_rule() {
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
        _ => ParserError::internal_grammar_error(source, pair.as_span())?,
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
}

impl ParserError {
    fn internal_grammar_error(source_code: &str, span: pest::Span<'_>) -> Result<!, Self> {
        let span = miette::SourceSpan::new(span.start().into(), (span.end() - span.start()).into());
        Err(Self::InternalGrammarError {
            source_code: source_code.to_string(),
            at: span,
        })
    }
}
