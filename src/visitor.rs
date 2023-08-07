pub trait Visitor {
    type Input;
    type Output;

    fn visit_expression(input: Self::Input) -> Self::Output;

    fn visit_if(input: Self::Input) -> Self::Output;
    fn visit_function_application(input: Self::Input) -> Self::Output;
    fn visit_comparison(input: Self::Input) -> Self::Output;
    fn visit_term(input: Self::Input) -> Self::Output;
    fn visit_factor(input: Self::Input) -> Self::Output;
    fn visit_unary(input: Self::Input) -> Self::Output;
    fn visit_primary(input: Self::Input) -> Self::Output;
}
