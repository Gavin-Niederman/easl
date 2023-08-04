pub mod ast;

// use palette::{FromColor, Xyza};
use pest::{
    iterators::{Pair, Pairs},
    Parser,
};
use pest_derive::Parser;

use ast::{ComparisonOperator, FactorOperator, Node, Statement, TermOperator, UnaryOperator};

use crate::parser::ast::{Literal, Primary};

use self::ast::Type;

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

fn build_statement(pair: Pair<'_, Rule>) -> Statement {
    let mut inner = pair.clone().into_inner();
    match pair.as_rule() {
        Rule::statement => build_statement(inner.next().unwrap()),
        Rule::assignment => {
            let ident = inner.next().unwrap().as_str().to_string();
            let expr = build_node(inner.next().unwrap());

            Statement::Assignment { ident, expr }
        }
        Rule::type_ascription => {
            let mut inner = pair.into_inner();
            let ident = inner.next().unwrap().as_str().to_string();
            let type_ = build_type(inner.next().unwrap());
            Statement::TypeAscription { ident, type_ }
        }
        Rule::include => Statement::Include {
            source: pair.into_inner().next().unwrap().as_str().to_string(),
        },
        Rule::EOI => Statement::EOI,
        _ => {
            unreachable!()
        }
    }
}

fn build_node(pair: Pair<'_, Rule>) -> Node {
    let mut inner = pair.clone().into_inner();
    macro_rules! unless_single_inner {
        ($block:block) => {
            if inner.len() == 1 {
                return build_node(inner.next().unwrap());
            } else $block
        }
    }

    match pair.as_rule() {
        Rule::expression => build_node(inner.next().unwrap()),

        Rule::lambda => unless_single_inner!({
            let param = inner.next().unwrap().as_str().to_string();
            let body = Box::new(build_node(inner.next().unwrap()));
            Node::Lambda { param, body }
        }),

        Rule::r#if => unless_single_inner!({
            let cond = Box::new(build_node(inner.next().unwrap()));
            let then = Box::new(build_node(inner.next().unwrap()));
            let else_ = Box::new(build_node(inner.next().unwrap()));
            Node::If { cond, then, else_ }
        }),
        Rule::function_call => unless_single_inner!({
            let callee = Box::new(build_node(inner.next().unwrap()));
            let param = Box::new(build_node(inner.next().unwrap()));
            Node::FunctionCall { callee, param }
        }),
        Rule::comparison => unless_single_inner!({
            let lhs = Box::new(build_node(inner.next().unwrap()));
            let operator = match inner.next().unwrap().as_rule() {
                Rule::equivalent => ComparisonOperator::Equivalent,
                Rule::not_equivalent => ComparisonOperator::NotEquivalent,
                Rule::greater_than => ComparisonOperator::GreaterThan,
                Rule::less_than => ComparisonOperator::LessThan,
                Rule::greater_than_or_eq => ComparisonOperator::GreaterThanOrEqual,
                Rule::less_than_or_eq => ComparisonOperator::LessThanOrEqual,
                _ => unreachable!(),
            };
            let rhs = Box::new(build_node(inner.next().unwrap()));
            Node::Comparison { operator, lhs, rhs }
        }),
        Rule::term => unless_single_inner!({
            let lhs = Box::new(build_node(inner.next().unwrap()));
            let operator = match inner.next().unwrap().as_rule() {
                Rule::add => TermOperator::Add,
                Rule::sub => TermOperator::Sub,
                _ => unreachable!(),
            };
            let rhs = Box::new(build_node(inner.next().unwrap()));
            Node::Term { operator, lhs, rhs }
        }),
        Rule::factor => unless_single_inner!({
            let lhs = Box::new(build_node(inner.next().unwrap()));
            let operator = match inner.next().unwrap().as_rule() {
                Rule::mul => FactorOperator::Mul,
                Rule::div => FactorOperator::Div,
                _ => unreachable!(),
            };
            let rhs = Box::new(build_node(inner.next().unwrap()));
            Node::Factor { operator, lhs, rhs }
        }),
        Rule::unary => unless_single_inner!({
            let operator = match inner.next().unwrap().as_rule() {
                Rule::not => UnaryOperator::Not,
                Rule::negate => UnaryOperator::Negate,
                Rule::negative => UnaryOperator::Negative,
                _ => unreachable!(),
            };
            let rhs = Box::new(build_node(inner.next().unwrap()));
            Node::Unary { operator, rhs }
        }),
        Rule::primary => build_node(inner.next().unwrap()),
        Rule::grouping => build_node(inner.next().unwrap()),
        Rule::literal => build_node(inner.next().unwrap()),
        Rule::ident => Node::Primary(Primary::Ident(pair.as_str().to_string())),

        //TODO: remove quotes
        Rule::string_l => {
            Node::Primary(Primary::Literal(Literal::String(pair.as_str().to_string())))
        }
        Rule::bool_l => Node::Primary(Primary::Literal(Literal::Bool(
            match inner.next().unwrap().as_rule() {
                Rule::r#true => true,
                Rule::r#false => false,
                _ => unreachable!(),
            },
        ))),
        Rule::int_l => Node::Primary(Primary::Literal(Literal::Int(
            match inner.next().unwrap().as_rule() {
                Rule::hex_int => {
                    i64::from_str_radix(pair.as_str(), 16).unwrap() as f64
                }
                Rule::binary_int => {
                    i64::from_str_radix(pair.as_str(), 2).unwrap() as f64
                }
                Rule::decimal_int => pair.as_str().parse::<f64>().unwrap(),
                _ => unreachable!(),
            },
        ))),
        Rule::unit_l => Node::Primary(Primary::Literal(Literal::Unit)),
        // Rule::color_l => ,
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
