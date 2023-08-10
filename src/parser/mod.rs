pub mod ast;

// use palette::{FromColor, Xyza};
use pest::{
    iterators::{Pair, Pairs},
    Parser,
};
use pest_derive::Parser;

use ast::{ComparisonOperator, Expression, FactorOperator, TermOperator, UnaryOperator};

use crate::parser::ast::{Literal, Primary};

use self::ast::{Statement, Type};

#[derive(Parser)]
#[grammar = "parser/easl.pest"]
pub struct EaslParser;

pub fn parse(source: &str) -> Result<Vec<Statement>, pest::error::Error<Rule>> {
    match EaslParser::parse(Rule::file, source) {
        Ok(pairs) => {
            let ast = build_ast(pairs);
            Ok(ast)
        }
        Err(err) => Err(err),
    }
}

fn build_ast(mut pairs: Pairs<'_, Rule>) -> Vec<Statement> {
    let mut ast = Vec::new();

    let Some(file) = pairs.next() else {
        return ast;
    };
    let statements = file.into_inner();

    for statement in statements {
        ast.push(build_statement(statement));
    }

    ast
}

fn build_statement(statement: Pair<'_, Rule>) -> Statement {
    let mut inner = statement.clone().into_inner();
    match statement.as_rule() {
        Rule::statement => build_statement(inner.next().unwrap()),
        Rule::assignment => {
            let ident = inner.next().unwrap().as_str().to_string();
            let expr = build_expression(inner.next().unwrap());

            Statement::Assignment { ident, expr }
        }
        Rule::type_ascription => {
            let ident = inner.next().unwrap().as_str().to_string();
            let type_ = build_type(inner.next().unwrap());
            Statement::TypeAscription { ident, type_ }
        }
        Rule::include => Statement::Include {
            source: inner.next().unwrap().as_str().to_string(),
        },
        Rule::EOI => Statement::EOI,
        _ => unreachable!(),
    }
}

fn build_expression(expression: Pair<'_, Rule>) -> Expression {
    let mut inner = expression.clone().into_inner();
    macro_rules! unless_1_inner {
        ($block:block) => {
            if inner.len() == 1 {
                return build_expression(inner.next().unwrap());
            } else $block
        };
    }

    match expression.as_rule() {
        Rule::expression => build_expression(inner.next().unwrap()),
        Rule::r#if => unless_1_inner!({
            let cond = Box::new(build_expression(inner.next().unwrap()));
            let then = Box::new(build_expression(inner.next().unwrap()));
            let else_ = Box::new(build_expression(inner.next().unwrap()));
            Expression::If { cond, then, else_ }
        }),
        Rule::comparison => unless_1_inner!({
            let lhs = Box::new(build_expression(inner.next().unwrap()));
            let operator = match inner.next().unwrap().as_rule() {
                Rule::equivalent => ComparisonOperator::Equivalent,
                Rule::not_equivalent => ComparisonOperator::NotEquivalent,
                Rule::less_than => ComparisonOperator::LessThan,
                Rule::less_than_or_eq => ComparisonOperator::LessThanOrEqual,
                Rule::greater_than => ComparisonOperator::GreaterThan,
                Rule::greater_than_or_eq => ComparisonOperator::GreaterThanOrEqual,
                _ => unreachable!(),
            };
            let rhs = Box::new(build_expression(inner.next().unwrap()));
            Expression::Comparison { lhs, operator, rhs }
        }),
        Rule::term => unless_1_inner!({
            let lhs = Box::new(build_expression(inner.next().unwrap()));
            let operator = match inner.next().unwrap().as_rule() {
                Rule::add => TermOperator::Add,
                Rule::sub => TermOperator::Sub,
                _ => unreachable!(),
            };
            let rhs = Box::new(build_expression(inner.next().unwrap()));
            Expression::Term { lhs, operator, rhs }
        }),
        Rule::factor => unless_1_inner!({
            let lhs = Box::new(build_expression(inner.next().unwrap()));
            let operator = match inner.next().unwrap().as_rule() {
                Rule::mul => FactorOperator::Mul,
                Rule::div => FactorOperator::Div,
                _ => unreachable!(),
            };
            let rhs = Box::new(build_expression(inner.next().unwrap()));
            Expression::Factor { lhs, operator, rhs }
        }),
        Rule::unary => unless_1_inner!({
            let operator = match inner.next().unwrap().as_rule() {
                Rule::not => UnaryOperator::Not,
                Rule::negate => UnaryOperator::Negate,
                Rule::negative => UnaryOperator::Negative,
                _ => unreachable!(),
            };
            let rhs = Box::new(build_expression(inner.next().unwrap()));
            Expression::Unary { operator, rhs }
        }),
        Rule::function_application => unless_1_inner!({
            let function = Box::new(build_expression(inner.next().unwrap()));
            let argument = Box::new(build_expression(inner.next().unwrap()));
            Expression::FunctionApplication { function, argument }
        }),
        Rule::primary => build_expression(inner.next().unwrap()),
        Rule::literal => build_expression(inner.next().unwrap()),
        Rule::lambda => unless_1_inner!({
            println!("{:?}", inner);
            println!("{:?}", expression.as_span());
            println!("{:?}", expression.as_rule());
            let param = inner.next().unwrap().as_str().to_string();
            let body = Box::new(build_expression(inner.next().unwrap()));
            Expression::Primary(Primary::Lambda { param, body })
        }),
        Rule::int_l => Expression::Primary(Primary::Literal(Literal::Int(
            match inner.next().unwrap().as_rule() {
                Rule::hex_int => i64::from_str_radix(expression.as_str(), 16).unwrap() as f64,
                Rule::binary_int => i64::from_str_radix(expression.as_str(), 2).unwrap() as f64,
                Rule::decimal_int => expression.as_str().parse::<f64>().unwrap(),
                _ => unreachable!(),
            },
        ))),
        Rule::string_l => Expression::Primary(Primary::Literal(Literal::String(
            expression.as_str().to_string(),
        ))),
        Rule::bool_l => Expression::Primary(Primary::Literal(Literal::Bool(
            match inner.next().unwrap().as_rule() {
                Rule::r#true => true,
                Rule::r#false => false,
                _ => unreachable!(),
            },
        ))),
        Rule::ident => Expression::Primary(Primary::Ident(expression.as_str().to_string())),
        Rule::grouping => build_expression(inner.next().unwrap()),
        Rule::unit_l => Expression::Primary(Primary::Literal(Literal::Unit)),
        _ => unreachable!(),
    }
}

fn build_type(pair: Pair<'_, Rule>) -> Type {
    let mut inner = pair.clone().into_inner();
    match pair.as_rule() {
        Rule::type_annotation => build_type(inner.next().unwrap()),
        Rule::r#type => build_type(inner.next().unwrap()),
        Rule::base_type => build_type(inner.next().unwrap()),
        Rule::fun_t => {
            let input = Box::new(build_type(inner.next().unwrap()));
            let output = Box::new(build_type(inner.next().unwrap()));
            Type::Fun { input, output }
        }
        Rule::string_t => Type::String,
        Rule::int_t => Type::Int,
        Rule::color_t => Type::Color,
        Rule::bool_t => Type::Bool,
        Rule::unit_t => Type::Unit,
        Rule::array_t => Type::Array(Box::new(build_type(inner.next().unwrap()))),
        _ => unreachable!(),
    }
}
