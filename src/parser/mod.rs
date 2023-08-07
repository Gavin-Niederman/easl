pub mod ast;

use std::marker::PhantomData;

use easl_derive::{passthrough_parse_visit, parse_visit};
// use palette::{FromColor, Xyza};
use pest::{
    iterators::{Pair, Pairs},
    Parser,
};
use pest_derive::Parser;

use ast::{ComparisonOperator, FactorOperator, Node, TermOperator, UnaryOperator};

use crate::{parser::ast::{Literal, Primary}, visitor::Visitor};

use self::ast::Type;


#[derive(Parser)]
#[grammar = "parser/easl.pest"]
pub struct EaslParser<'a> {
    _marker: PhantomData<&'a ()>,
}
impl<'a> Visitor for EaslParser<'a> {
    type Input = Pair<'a, Rule>;
    type Output = Node;

    #[parse_visit]
    fn visit_statement(input: Self::Input) -> Self::Output {
        println!("{:?}", input.as_rule());
        match input.as_rule() {
            Rule::statement => Self::visit_statement(inner.next().unwrap()),
            Rule::assignment => Self::visit_assignment(input),
            Rule::type_ascription => Self::visit_type_ascription(input),
            Rule::include => Self::visit_include(input),
            Rule::EOI => Self::visit_eoi(input),
            _ => unreachable!(),
        }
    }

    #[parse_visit]
    fn visit_assignment(input: Self::Input) -> Self::Output {
        let ident = inner.next().unwrap().as_str().to_string();
        let expr = Box::new(Self::visit_expression(inner.next().unwrap()));

        Node::Assignment { ident, expr }
    }

    #[parse_visit]
    fn visit_type_ascription(input: Self::Input) -> Self::Output {
        let ident = inner.next().unwrap().as_str().to_string();
        let type_ = build_type(inner.next().unwrap());
        Node::TypeAscription { ident, type_ }
    }

    #[parse_visit]
    fn visit_include(input: Self::Input) -> Self::Output {
        Node::Include { source: input.into_inner().next().unwrap().as_str().to_string() }
    }

    #[parse_visit]
    fn visit_expression(input: Self::Input) -> Self::Output {
        println!("{:?}", input.as_rule());
        match input.as_rule() {
            Rule::expression => Self::visit_expression(inner.next().unwrap()),
            Rule::lambda => Self::visit_lambda(input),
            Rule::r#if => Self::visit_if(input),
            Rule::function_application => Self::visit_function_application(input),
            Rule::comparison => Self::visit_comparison(input),
            Rule::term => Self::visit_term(input),
            Rule::factor => Self::visit_factor(input),
            Rule::unary => Self::visit_unary(input),
            Rule::primary => Self::visit_primary(input),
            _ => unreachable!(),
        }
    }

    #[passthrough_parse_visit]
    fn visit_lambda(input: Self::Input) -> Self::Output {
        let inner = input.into_inner();
        let mut inner = inner.clone();
        let param = inner.next().unwrap().as_str().to_string();
        let body = Box::new(Self::visit_expression(inner.next().unwrap()));
        Node::Lambda { param, body }
        
    }

    #[passthrough_parse_visit]
    fn visit_if(input: Self::Input) -> Self::Output {
            let cond = Box::new(Self::visit_expression(inner.next().unwrap()));
            let then = Box::new(Self::visit_expression(inner.next().unwrap()));
            let else_ = Box::new(Self::visit_expression(inner.next().unwrap()));
            Node::If { cond, then, else_ }
    }

    #[passthrough_parse_visit]
    fn visit_function_application(input: Self::Input) -> Self::Output {
            let function = Box::new(Self::visit_expression(inner.next().unwrap()));
            let argument = Box::new(Self::visit_expression(inner.next().unwrap()));
            Node::FunctionApplication { function, argument }

    }
    #[passthrough_parse_visit]
    fn visit_comparison(input: Self::Input) -> Self::Output {
            let lhs = Box::new(Self::visit_expression(inner.next().unwrap()));
            let operator = match inner.next().unwrap().as_rule() {
                Rule::equivalent => ComparisonOperator::Equivalent,
                Rule::not_equivalent => ComparisonOperator::NotEquivalent,
                Rule::less_than => ComparisonOperator::LessThan,
                Rule::less_than_or_eq => ComparisonOperator::LessThanOrEqual,
                Rule::greater_than => ComparisonOperator::GreaterThan,
                Rule::greater_than_or_eq => ComparisonOperator::GreaterThanOrEqual,
                _ => unreachable!(),
            };
            let rhs = Box::new(Self::visit_expression(inner.next().unwrap()));
            Node::Comparison { lhs, operator, rhs }
    }

    #[passthrough_parse_visit]
    fn visit_term(input: Self::Input) -> Self::Output {

            let lhs = Box::new(Self::visit_expression(inner.next().unwrap()));
            let operator = match inner.next().unwrap().as_rule() {
                Rule::add => TermOperator::Add,
                Rule::sub => TermOperator::Sub,
                _ => unreachable!(),
            };
            let rhs = Box::new(Self::visit_expression(inner.next().unwrap()));
            Node::Term { lhs, operator, rhs }
    }

    #[passthrough_parse_visit]
    fn visit_factor(input: Self::Input) -> Self::Output {
        let lhs = Box::new(Self::visit_expression(inner.next().unwrap()));
        let operator = match inner.next().unwrap().as_rule() {
            Rule::mul => FactorOperator::Mul,
            Rule::div => FactorOperator::Div,
            _ => unreachable!(),
        };
        let rhs = Box::new(Self::visit_expression(inner.next().unwrap()));
        Node::Factor { lhs, operator, rhs }
    }

    #[passthrough_parse_visit]
    fn visit_unary(input: Self::Input) -> Self::Output {
        let operator = match input.as_rule() {
            Rule::not => UnaryOperator::Not,
            Rule::negate => UnaryOperator::Negate,
            Rule::negative => UnaryOperator::Negative,
            _ => unreachable!(),
        };
        let rhs = Box::new(Self::visit_expression(inner.next().unwrap()));
        Node::Unary { operator, rhs }
    }

    #[parse_visit]
    fn visit_primary(input: Self::Input) -> Self::Output {
        println!("{:?}", input.as_rule());
        match input.as_rule() {
            Rule::primary => Self::visit_primary(inner.next().unwrap()),
            Rule::literal => Self::visit_primary(inner.next().unwrap()),
            Rule::int_l => Node::Primary(Primary::Literal(Literal::Int(
                match inner.next().unwrap().as_rule() {
                    Rule::hex_int => {
                        i64::from_str_radix(input.as_str(), 16).unwrap() as f64
                    }
                    Rule::binary_int => {
                        i64::from_str_radix(input.as_str(), 2).unwrap() as f64
                    }
                    Rule::decimal_int => input.as_str().parse::<f64>().unwrap(),
                    _ => unreachable!(),
                },
            ))),
            Rule::string_l => {
                Node::Primary(Primary::Literal(Literal::String(input.as_str().to_string())))
            },
            Rule::bool_l => Node::Primary(Primary::Literal(Literal::Bool(
                match inner.next().unwrap().as_rule() {
                    Rule::r#true => true,
                    Rule::r#false => false,
                    _ => unreachable!(),
                },
            ))),
            Rule::ident => Node::Primary(Primary::Ident(input.as_str().to_string())),
            Rule::grouping => Self::visit_expression(inner.next().unwrap()),
            Rule::unit_l => Node::Primary(Primary::Literal(Literal::Unit)),
            _ => unreachable!(),
        }
    }

    fn visit_eoi(_input: Self::Input) -> Self::Output {
        Node::EOI
    }
}

pub fn parse(source: &str) -> Result<Vec<Node>, pest::error::Error<Rule>> {
    match EaslParser::parse(Rule::file, source) {
        Ok(pairs) => {
            let ast = build_ast(pairs);
            Ok(ast)
        }
        Err(err) => Err(err),
    }
}

fn build_ast(mut pairs: Pairs<'_, Rule>) -> Vec<Node> {
    let mut ast = Vec::new();

    let Some(file) = pairs.next() else {
        return ast;
    };
    let statements = file.into_inner();

    for statement in statements {
        ast.push(EaslParser::visit_statement(statement));
    }

    ast
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
