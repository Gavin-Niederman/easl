use std::{rc::Rc, sync::Mutex};

use crate::{tc::variant::Type, utils::Spanned};

use super::{
    ast::{
        BinaryOperator, Expression, IdentifierMap, Primary, Statement, UnaryOperator,
        UnwrapSameTypes,
    },
    ParserError,
};
use chumsky::{
    prelude::*,
    text::{ident, Character, TextParser},
};

pub fn strip_comments() -> impl Parser<char, String, Error = Simple<char>> {
    let comment = just("--")
        .padded_by(just(" ").repeated())
        .ignore_then(filter(|ch| *ch != '\n').repeated())
        .ignore_then(just("\n").ignored().or(end()))
        .ignored();

    comment.to('\n').or(any()).repeated().collect()
}

pub fn parser(
    ident_map: Rc<Mutex<IdentifierMap>>,
) -> impl Parser<char, Vec<Spanned<Statement>>, Error = ParserError> {
    let ident = ident::<char, ParserError>()
        .padded_by(filter(|ch: &char| ch.is_whitespace() && *ch != '\n').repeated())
        .map(move |name: String| ident_map.lock().unwrap().create_identifier(name));

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
            .then(
                just(".")
                    .ignore_then(
                        filter(|ch: &char| ch.is_ascii_digit())
                            .padded_by(just("_").repeated())
                            .repeated()
                            .at_least(1)
                            .collect(),
                    )
                    .or_not(),
            )
            .map(
                |(integer, rational): (String, Option<String>)| match rational {
                    Some(rational) => (integer + "." + &rational).parse::<f64>().unwrap(),
                    None => integer.parse::<f64>().unwrap(),
                },
            );

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
            .padded_by(filter(|ch: &char| ch.is_whitespace() && *ch != '\n').repeated())
            .map_with_span(|primary, span| Spanned {
                inner: primary,
                span,
            });
        let grouping = expression
            .clone()
            .delimited_by(just("("), just(")"))
            .map_with_span(|expr, span| Spanned {
                inner: Primary::Grouping(expr),
                span,
            });
        let lamda = (just("\\")
            .ignore_then(ident.clone())
            .then_ignore(
                just("->")
                    .padded_by(filter(|ch: &char| ch.is_whitespace() && *ch != '\n').repeated()),
            )
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

        let primary = lamda
            .padded_by(filter(|ch: &char| ch.is_whitespace() && *ch != '\n').repeated())
            .map_with_span(|primary, span| Spanned {
                inner: Expression::Primary(primary),
                span,
            });

        let variable = primary
            .or(ident.clone().map_with_span(|ident, span| Spanned {
                inner: Expression::Variable(ident.always_ok()),
                span,
            }))
            .padded_by(filter(|ch: &char| ch.is_whitespace() && *ch != '\n').repeated());

        let function_application = ((variable
            .clone()
            .then(variable.clone().repeated().at_least(1))
            .foldl(
                |function: Spanned<Expression>, arg: Spanned<Expression>| Spanned {
                    inner: Expression::FunctionApplication {
                        function: Box::new(function.clone()),
                        argument: Box::new(arg.clone()),
                    },
                    span: Spanned::add_spans(function, arg),
                },
            ))
        .or(variable))
        .padded_by(filter(|ch: &char| ch.is_whitespace() && *ch != '\n').repeated());

        let unary_operator = (just("!")
            .to(UnaryOperator::Not)
            .or(just("-").to(UnaryOperator::Negative)))
        .padded_by(filter(|ch: &char| ch.is_whitespace() && *ch != '\n').repeated());
        let unary = recursive(|unary| {
            (unary_operator
                .then(unary)
                .map_with_span(|(operator, unary), span| {
                    Box::new(Spanned {
                        inner: Expression::Unary {
                            operator,
                            rhs: unary,
                        },
                        span,
                    })
                }))
            .or(function_application.map(|fna| Box::new(fna)))
        });

        let factor_operator = (just("*")
            .to(BinaryOperator::Mul)
            .or(just("/").to(BinaryOperator::Div))
            .or(just("%").to(BinaryOperator::Remainder)))
        .padded_by(filter(|ch: &char| ch.is_whitespace() && *ch != '\n').repeated());
        let factor = recursive(|factor| {
            unary
                .clone()
                .then(factor_operator.clone())
                .then(factor.or(unary.clone()))
                .map_with_span(|((lhs, operator), rhs), span| {
                    Box::new(Spanned {
                        inner: Expression::Binary { operator, lhs, rhs },
                        span,
                    })
                })
                .or(unary)
        });
        let term_operator = (just::<_, _, ParserError>("+")
            .to(BinaryOperator::Add)
            .or(just("-").to(BinaryOperator::Sub)))
        .padded_by(filter(|ch: &char| ch.is_whitespace() && *ch != '\n').repeated());
        let term = recursive(|term| {
            factor
                .clone()
                .then(term_operator)
                .then(term.or(factor.clone()))
                .map_with_span(|((lhs, operator), rhs), span| {
                    Box::new(Spanned {
                        inner: Expression::Binary { operator, lhs, rhs },
                        span,
                    })
                })
                .or(factor.clone())
        });

        let comparison_operator = (just::<_, _, ParserError>("==")
            .to(BinaryOperator::Equivalent)
            .or(just("!=").to(BinaryOperator::NotEquivalent))
            .or(just(">").to(BinaryOperator::GreaterThan))
            .or(just("<").to(BinaryOperator::LessThan))
            .or(just(">=").to(BinaryOperator::GreaterThanOrEqual))
            .or(just("<=").to(BinaryOperator::LessThanOrEqual)))
        .padded_by(filter(|ch: &char| ch.is_whitespace() && *ch != '\n').repeated());
        let comparison = recursive(|comparison| {
            factor
                .then(comparison_operator)
                .then(comparison.or(term.clone()))
                .map_with_span(|((lhs, operator), rhs), span| {
                    Box::new(Spanned {
                        inner: Expression::Binary { operator, lhs, rhs },
                        span,
                    })
                })
                .or(term)
        });

        let if_ = recursive(|if_| {
            let pre = just("if").padded();
            let then = just("then").padded();
            let else_ = just("else").padded();

            pre.ignore_then(if_.clone().or(comparison.clone()))
                .then_ignore(then)
                .then(if_.clone().or(comparison.clone()))
                .then_ignore(else_)
                .then(if_.or(comparison.clone()))
                .map_with_span(|((cond, then), else_), span| {
                    Box::new(Spanned {
                        inner: Expression::If { cond, then, else_ },
                        span,
                    })
                })
                .or(comparison)
        });

        if_
    });

    let type_ = recursive(|type_| {
        let base_type = recursive(|base_type| {
            let string = just::<_, _, ParserError>("String").to(Type::String);
            let int = just::<_, _, ParserError>("Int").to(Type::Int);
            let color = just::<_, _, ParserError>("Color").to(Type::Int);
            let boolean = just::<_, _, ParserError>("Bool").to(Type::Bool);
            let unit = just::<_, _, ParserError>("()").to(Type::Unit);

            let array = base_type
                .clone()
                .padded_by(filter(|ch: &char| ch.is_whitespace() && *ch != '\n').repeated())
                .delimited_by(just("["), just("]"))
                .map(|inner| Type::Array(Box::new(inner)));

            array.or(string).or(int).or(color).or(boolean).or(unit)
        })
        .padded_by(filter(|ch: &char| ch.is_whitespace() && *ch != '\n').repeated());
        let fun = base_type
            .clone()
            .then_ignore(just("->"))
            .then(type_.or(base_type))
            .map(|(lhs, rhs)| Type::Fun {
                input: Box::new(lhs),
                output: Box::new(rhs),
            });

        fun
    })
    .padded_by(filter(|ch: &char| ch.is_whitespace() && *ch != '\n').repeated());

    let type_annotation = just("::")
        .padded_by(filter(|ch: &char| ch.is_whitespace() && *ch != '\n').repeated())
        .ignore_then(type_)
        .padded_by(filter(|ch: &char| ch.is_whitespace() && *ch != '\n').repeated());

    let assignment = ident
        .clone()
        .then_ignore(just("="))
        .then(expression)
        .then(type_annotation.clone().or_not())
        .map_with_span(|((ident, expr), type_), span| Spanned {
            inner: Statement::Assignment {
                ident: ident.clone().always_ok(),
                type_ascription: if let Some(type_) = type_ {
                    Some(Box::new(Statement::TypeAscription {
                        ident: ident.always_ok(),
                        type_,
                    }))
                } else {
                    None
                },
                expr: *expr,
            },
            span,
        });

    let type_ascription = ident
        .then(type_annotation)
        .map_with_span(|(ident, type_), span| Spanned {
            inner: Statement::TypeAscription {
                ident: ident.always_ok(),
                type_,
            },
            span,
        });

    //TODO: includes
    let statement = assignment
        .or(type_ascription)
        .then_ignore(just("\n"))
        .padded();

    let file = statement.repeated().then_ignore(end());

    file
}
