pub mod ast;

use palette::FromColor;
use pest::{
    iterators::{Pair, Pairs},
    Parser,
};
use pest_derive::Parser;

use ast::{ComparisonOperator, FactorOperator, Node, Statement, TermOperator, UnaryOperator};

use crate::parser::ast::{Literal, Primary};

use self::ast::Type;

#[derive(Parser)]
#[grammar = "easl.pest"]
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
    let mut statements = Vec::new();

    let Some(file) = pairs.next() else {
        return statements;
    };
    let statement_pairs = file.into_inner();

    for statement in statement_pairs {
        statements.push(build_statement(statement));
        println!("{:#?}", statements.last().unwrap());
    }

    statements
}

fn build_statement(pair: Pair<'_, Rule>) -> Statement {
    match pair.as_rule() {
        Rule::statement => build_statement(pair.into_inner().next().unwrap()),
        Rule::assignment => {
            let mut inner = pair.into_inner();
            let mut inner_expressions: Vec<Node> = inner.clone().filter(|e| e.as_rule() == Rule::expression).map(|e| build_node(e)).collect();
            let ident = inner.next().unwrap().as_str().to_string();
            let expr = inner_expressions.pop().unwrap();
            let args = inner_expressions;
            let type_ = match inner.last() {
                Some(type_) => {
                    Some(build_type(type_))
                },
                None => None,
            };

            Statement::Assignment { ident, args, expr, type_ }
        },
        Rule::type_ascription => {
            let mut inner = pair.into_inner();
            let ident = inner.next().unwrap().as_str().to_string();
            let type_ = build_type(inner.next().unwrap());
            Statement::TypeAscription { ident, type_ }
        },
        Rule::include => {
            Statement::Include { source: pair.into_inner().next().unwrap().as_str().to_string() }
        },
        _ => { unreachable!() }
    }
}

fn build_node(pair: Pair<'_, Rule>) -> Node {
    macro_rules! boxed_next {
        ($ident:ident) => {
            Box::new(build_node($ident.next().unwrap()))
        };
    }

    let mut inner = pair.clone().into_inner();

    match pair.as_rule() {
        Rule::r#if => Node::If {
            cond: boxed_next!(inner),
            then: boxed_next!(inner),
            else_: boxed_next!(inner),
        },
        Rule::function_call => {
            let mut fn_call_inner = pair.into_inner();
            if fn_call_inner.len() == 1 {
                let next = fn_call_inner.next().unwrap();
                match next.as_rule() {
                    Rule::ident => Node::FunctionCall {
                        ident: next.as_str().to_string(),
                        args: vec![],
                    },
                    Rule::comparison => build_node(fn_call_inner.next().unwrap()),
                    _ => {
                        unreachable!()
                    }
                }
            } else {
                Node::FunctionCall {
                    ident: fn_call_inner.next().unwrap().as_str().to_string(),
                    args: fn_call_inner.map(|pair| build_node(pair)).collect(),
                }
            }
        }
        Rule::comparison => {
            if inner.len() == 1 {
                build_node(inner.next().unwrap())
            } else {
                let lhs = boxed_next!(inner);
                let operator = match inner.next().unwrap().into_inner().next().unwrap().as_rule() {
                    Rule::equivalent => ComparisonOperator::Equivalent,
                    Rule::not_equivalent => ComparisonOperator::NotEquivalent,
                    Rule::greater_than => ComparisonOperator::GreaterThan,
                    Rule::less_than => ComparisonOperator::LessThan,
                    Rule::greater_than_or_eq => ComparisonOperator::GreaterThanOrEqual,
                    Rule::less_than_or_eq => ComparisonOperator::LessThanOrEqual,
                    _ => unreachable!(),
                };
                let rhs = boxed_next!(inner);
                Node::Comparison { operator, lhs, rhs }
            }
        }
        Rule::term => {
            if inner.len() == 1 {
                build_node(inner.next().unwrap())
            } else {
                let lhs = boxed_next!(inner);
                let operator = match inner.next().unwrap().into_inner().next().unwrap().as_rule() {
                    Rule::add => TermOperator::Add,
                    Rule::sub => TermOperator::Sub,
                    _ => unreachable!(),
                };
                let rhs = boxed_next!(inner);
                Node::Term { operator, lhs, rhs }
            }
        }
        Rule::factor => {
            if inner.len() == 1 {
                build_node(inner.next().unwrap())
            } else {
                let lhs = boxed_next!(inner);
                let operator = match inner.next().unwrap().into_inner().next().unwrap().as_rule() {
                    Rule::mul => FactorOperator::Mul,
                    Rule::div => FactorOperator::Div,
                    _ => unreachable!(),
                };
                let rhs = boxed_next!(inner);
                Node::Factor { operator, lhs, rhs }
            }
        }
        Rule::unary => {
            if inner.len() == 1 {
                build_node(inner.next().unwrap())
            } else {
                let operator = match inner.next().unwrap().into_inner().next().unwrap().as_rule() {
                    Rule::not => UnaryOperator::Not,
                    Rule::negate => UnaryOperator::Negate,
                    Rule::negative => UnaryOperator::Negative,
                    _ => unreachable!(),
                };
                let expr = boxed_next!(inner);

                Node::Unary { operator, expr }
            }
        }
        Rule::primary => build_node(inner.next().unwrap()),
        Rule::grouping => build_node(inner.next().unwrap()),
        Rule::ident => Node::Primary(Primary::Ident(inner.as_str().to_string())),
        Rule::literal => {
            let literal = inner.next().unwrap();
            match literal.as_rule() {
                Rule::string_l => Node::Primary(Primary::Literal(Literal::String(
                    literal.into_inner().next().unwrap().as_str().to_string(),
                ))),
                Rule::bool_l => Node::Primary(Primary::Literal(Literal::Bool(
                    match inner.next().unwrap().into_inner().next().unwrap().as_rule() {
                        Rule::r#true => true,
                        Rule::r#false => false,
                        _ => unreachable!(),
                    },
                ))),
                Rule::int_l => {
                    let int_type = inner.next().unwrap().into_inner().next().unwrap();
                    Node::Primary(Primary::Literal(Literal::Int(match int_type.as_rule() {
                        Rule::decimal_int => int_type.as_str().parse::<f64>().unwrap(),
                        Rule::hex_int => i64::from_str_radix(int_type.as_str(), 16).unwrap() as f64,
                        Rule::binary_int => {
                            i64::from_str_radix(int_type.as_str(), 2).unwrap() as f64
                        }
                        _ => unreachable!(),
                    })))
                }
                Rule::color_l => {
                    let color = literal.into_inner().next().unwrap().as_str();
                    let color = match color.chars().collect::<Vec<char>>().into_iter().len() {
                        3 => {
                            let mut chars = color.chars();
                            let r = chars.next().unwrap();
                            let g = chars.next().unwrap();
                            let b = chars.next().unwrap();
                            palette::Xyza::from_color(palette::rgb::Srgb::new(
                                u8::from_str_radix(&format!("{}{}", r, r), 16).unwrap() as f32,
                                u8::from_str_radix(&format!("{}{}", g, g), 16).unwrap() as f32,
                                u8::from_str_radix(&format!("{}{}", b, b), 16).unwrap() as f32,
                            ))
                        }
                        6 => {
                            let mut chars = color.chars();
                            palette::Xyza::from_color(palette::rgb::Srgb::new(
                                u8::from_str_radix(
                                    &format!(
                                        "{}",
                                        chars
                                            .next_chunk::<2>()
                                            .unwrap()
                                            .into_iter()
                                            .collect::<String>()
                                    ),
                                    16,
                                )
                                .unwrap() as f32,
                                u8::from_str_radix(
                                    &format!(
                                        "{}",
                                        chars
                                            .next_chunk::<2>()
                                            .unwrap()
                                            .into_iter()
                                            .collect::<String>()
                                    ),
                                    16,
                                )
                                .unwrap() as f32,
                                u8::from_str_radix(
                                    &format!(
                                        "{}",
                                        chars
                                            .next_chunk::<2>()
                                            .unwrap()
                                            .into_iter()
                                            .collect::<String>()
                                    ),
                                    16,
                                )
                                .unwrap() as f32,
                            ))
                        }
                        8 => {
                            let mut chars = color.chars();
                            palette::Xyza::from_color(palette::rgb::Srgba::new(
                                u8::from_str_radix(
                                    &format!(
                                        "{}",
                                        chars
                                            .next_chunk::<2>()
                                            .unwrap()
                                            .into_iter()
                                            .collect::<String>()
                                    ),
                                    16,
                                )
                                .unwrap() as f32,
                                u8::from_str_radix(
                                    &format!(
                                        "{}",
                                        chars
                                            .next_chunk::<2>()
                                            .unwrap()
                                            .into_iter()
                                            .collect::<String>()
                                    ),
                                    16,
                                )
                                .unwrap() as f32,
                                u8::from_str_radix(
                                    &format!(
                                        "{}",
                                        chars
                                            .next_chunk::<2>()
                                            .unwrap()
                                            .into_iter()
                                            .collect::<String>()
                                    ),
                                    16,
                                )
                                .unwrap() as f32,
                                u8::from_str_radix(
                                    &format!(
                                        "{}",
                                        chars
                                            .next_chunk::<2>()
                                            .unwrap()
                                            .into_iter()
                                            .collect::<String>()
                                    ),
                                    16,
                                )
                                .unwrap() as f32,
                            ))
                        }
                        _ => unreachable!(),
                    };
                    Node::Primary(Primary::Literal(Literal::Color(color)))
                },
                _ => unreachable!(),
            }
        },
        _ => unreachable!(),
    }
}

fn build_type(pair: Pair<'_, Rule>) -> Type {
    println!("{:#?}", pair.as_rule());
    let mut type_type = pair.into_inner().next().unwrap();
    println!("{:#?}", type_type);
    match type_type.as_rule() {
        Rule::r#type => {
            build_type(type_type.into_inner().next().unwrap())
        }
        Rule::base_type => {
            build_type(type_type.into_inner().next().unwrap())
        }
        Rule::fun_t => {
            let mut inner = type_type.into_inner();
            let input = Box::new(build_type(inner.next().unwrap()));
            let output = Box::new(build_type(inner.next().unwrap()));
            Type::Fun { input, output }
        }
        Rule::string_t => Type::String,
        Rule::int_t => Type::Int,
        Rule::color_t => Type::Color,
        Rule::bool_t => Type::Bool,
        Rule::unit_t => Type::Unit,
        Rule::array_t => {
            Type::Array(Box::new(
                build_type(type_type.into_inner().next().unwrap())
            ))
        }
        _ => unreachable!(),
    }
}