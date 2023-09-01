use chumsky::prelude::*;

use crate::parser::ParserError;

pub fn comment_pass() -> impl Parser<char, String, Error = ParserError> {
    let comment = just("--")
        .ignore_then(filter(|ch: &char| *ch != '\n').repeated())
        .to('\n');

    (comment.or(filter(|c| *c != '\n')))
        .then_ignore(just('\n'))
        .repeated()
        .then_ignore(end())
        .collect()
}
