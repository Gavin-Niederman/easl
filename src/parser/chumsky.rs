use super::{
    ast::{
        BinaryOperator, Expression, IdentifierMap, Primary, Spanned, Statement, UnaryOperator,
        UnwrapSameTypes,
    },
    ParserError,
};
use chumsky::{prelude::*, text::ident};

pub fn parser(
    ident_map: &mut IdentifierMap,
) -> impl Parser<char, Vec<Spanned<Statement>>, Error = ParserError> {
    let ident = ident()
        .padded()
        .map(|name: String| ident_map.create_identifier(name));

    let expression = recursive(|expression| {
        let hex_int = filter(|ch: &char| ch.is_ascii_hexdigit())
            .padded_by(just("_").repeated())
            .repeated()
            .at_least(1)
            .at_most(8)
            .collect()
            .map(|hex: String| u64::from_str_radix(&hex, 16).unwrap() as f64);
        let binary_int = filter(|ch: &char| *ch == '0' || *ch == '1')
            .padded_by(just("_").repeated())
            .repeated()
            .at_least(1)
            .at_most(64)
            .collect()
            .map(|binary: String| u64::from_str_radix(&binary, 2).unwrap() as f64);
        let decimal_int = filter(|ch: &char| ch.is_ascii_digit())
            .padded_by(just("_").repeated())
            .repeated()
            .at_least(1)
            .collect()
            .map(|decimal: String| decimal.parse().unwrap());

        let int_l = ((just("0x").ignore_then(hex_int))
            .or(just("0b").ignore_then(binary_int))
            .or(decimal_int))
        .map(|num: f64| Primary::Int(num));

        let bool_l = (just("True").map(|_| Primary::Bool(true)))
            .or(just("False").map(|_| Primary::Bool(false)));

        let unit_l = just("()").map(|_| Primary::Unit);

        let string_l = just("\"")
            .ignore_then(none_of("\"").repeated())
            .then_ignore(just("\""))
            .collect()
            .map(|string| Primary::String(string));

        let literal = (int_l.or(bool_l).or(unit_l).or(string_l))
            .padded()
            .map_with_span(|primary, span| Spanned {
                inner: primary,
                span,
            });
        let grouping = expression
            .delimited_by(just("("), just(")"))
            .map_with_span(|expr, span| Spanned {
                inner: Primary::Grouping(expr),
                span,
            });
        let lamda = (just("\\")
            .ignore_then(ident)
            .then_ignore(just("->").padded())
            .then(expression)
            .map_with_span(|(param, body), span| Spanned {
                inner: Primary::Lambda {
                    param: param.always_ok(),
                    body,
                },
                span,
            }))
        .or(literal)
        .or(grouping);

        let primary = lamda.padded().map_with_span(|primary, span| Spanned {
            inner: Expression::Primary(primary),
            span,
        });

        let variable = primary
            .or(ident.map_with_span(|ident, span| Spanned {
                inner: Expression::Variable(ident.always_ok()),
                span,
            }))
            .padded();

        let function_application = ((variable.then(variable.repeated().at_least(1)).foldl(
            |function: Spanned<Expression>, arg: Spanned<Expression>| Spanned {
                inner: Expression::FunctionApplication {
                    function: Box::new(function),
                    argument: Box::new(arg),
                },
                span: Spanned::add_spans(function, arg),
            },
        ))
        .or(variable))
        .padded();

        let unary_operator = (just("!")
            .to(UnaryOperator::Not)
            .or(just("-").to(UnaryOperator::Negative)))
        .padded();
        let unary = unary_operator
            .repeated()
            .then(function_application.map_with_span(|inner, span| Spanned { inner, span }))
            .foldr(|operator, rhs| Spanned { inner: Expression::Unary { operator, rhs }, span: rhs.span })
            .map_with_span(|inner, span| Spanned { inner, span })
            .padded();

        let factor_operator = (just("*")
            .to(BinaryOperator::Mul)
            .or(just("/").to(BinaryOperator::Div))
            .or(just("%").to(BinaryOperator::Remainder)))
        .padded();
        let factor = unary
            .then((factor_operator.then(unary)).repeated())
            .foldl(|lhs, (operator, rhs)| Expression::Binary { operator, lhs, rhs })
            .map_with_span(|inner, span| Spanned { inner, span })
            .padded();

        let term_operator = (just("+")
            .to(BinaryOperator::Add)
            .or(just("-").to(BinaryOperator::Sub)))
        .padded();
        let term = factor
            .then((term_operator.then(factor)).repeated())
            .foldl(|lhs, (operator, rhs)| Expression::Binary { operator, lhs, rhs })
            .map_with_span(|inner, span| Spanned { inner, span })
            .padded();

        let comparison_operator = (just("==")
            .to(BinaryOperator::Equivalent)
            .or(just("!=").to(BinaryOperator::NotEquivalent))
            .or(just(">").to(BinaryOperator::GreaterThan))
            .or(just("<").to(BinaryOperator::LessThan))
            .or(just(">=").to(BinaryOperator::GreaterThanOrEqual))
            .or(just("<=").to(BinaryOperator::LessThanOrEqual)))
        .padded();
        let comparison = term
            .then((comparison_operator.then(term)).repeated())
            .foldl(|lhs, (operator, rhs)| Expression::Binary { operator, lhs, rhs })
            .map_with_span(|inner, span| Spanned { inner, span })
            .padded();

        let if_ = recursive(|if_| {
            let pre = just("if").padded();
            let then = just("then").padded();
            let else_ = just("else").padded();

            pre.ignore_then(if_)
                .then_ignore(then)
                .then(if_)
                .then_ignore(else_)
                .then(if_)
                .map_with_span(|((cond, then), else_), span| {
                    Box::new(Spanned {
                        inner: Expression::If { cond, then, else_ },
                        span,
                    })
                })
        });

        if_
    });
}
