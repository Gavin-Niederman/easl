use crate::parser::ast::{Node, Primary};
use crate::visitor::Visitor;

pub struct EaslInterpereter;
impl Visitor for EaslInterpereter {
    type Input = Node;
    type Output = Primary;

    fn visit_expression(input: Self::Input) -> Self::Output {
        match input {
            Node::If { .. } => Self::visit_if(input),
            Node::FunctionApplication { .. } => Self::visit_function_application(input),
            Node::Comparison { .. } => Self::visit_comparison(input),
            Node::Term { .. } => Self::visit_term(input),
            Node::Factor { .. } => Self::visit_factor(input),
            Node::Unary { .. } => Self::visit_unary(input),
            Node::Primary(..) => Self::visit_primary(input),
        }
    }

    fn visit_if(input: Self::Input) -> Self::Output {
        todo!()
    }

    fn visit_function_application(input: Self::Input) -> Self::Output {
        todo!()
    }

    fn visit_comparison(input: Self::Input) -> Self::Output {
        todo!()
    }

    fn visit_term(input: Self::Input) -> Self::Output {
        todo!()
    }

    fn visit_factor(input: Self::Input) -> Self::Output {
        todo!()
    }

    fn visit_unary(input: Self::Input) -> Self::Output {
        todo!()
    }

    fn visit_primary(input: Self::Input) -> Self::Output {
        todo!()
    }
}
