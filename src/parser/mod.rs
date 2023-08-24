use std::{ops::Range, sync::Mutex, rc::Rc};
use ::chumsky::prelude::*;
use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

use crate::SOURCE;

use self::{ast::{IdentifierMap, Statement, Spanned}, chumsky::strip_comments};

pub mod ast;
pub mod chumsky;

pub fn parse(source: &str) -> Result<(Vec<Spanned<Statement>>, Rc<Mutex<IdentifierMap>>), Vec<ParserError>> {
    let ident_map = Rc::new(Mutex::new(IdentifierMap::new()));
    let source = strip_comments().parse(source).unwrap();
    Ok((chumsky::parser(ident_map.clone()).parse(source)?, ident_map))
}

#[derive(Debug, Error, Diagnostic)]
pub enum ParserError {
    #[error("Expected one of: {expected} found {found}")]
    #[diagnostic(code(easl::parser::expected_input_found))]
    ExpectedInputFound {
        #[source_code]
        source_code: String,

        #[label]
        span: SourceSpan,

        expected: String,
        found: char,
    },
    #[error("Expected one of: {expected}")]
    #[diagnostic(code(easl::parser::expected_input))]
    ExpectedInput {
        #[source_code]
        source_code: String,

        #[label]
        span: SourceSpan,

        expected: String,
    },
    #[error("Identifier defined multiple times")]
    #[diagnostic(
        code(easl::parser::overriden_identifier),
        help = "Remove one of the definitions"
    )]
    OverridenIdentifier {
        #[source_code]
        source_code: String,
        #[label("Identifier was assigned again here")]
        second_assignment: SourceSpan,
    },
    #[error("Unknown identifier '{ident}'")]
    #[diagnostic(code(easl::parser::unknown_identifier), help = "Was this a typo?")]
    UnknownIdentifier {
        #[source_code]
        source_code: String,
        ident: String,
        #[label("Unknown identifier")]
        unknown_identifier: SourceSpan,
    },
}

impl ::chumsky::Error<char> for ParserError {
    type Span = Range<usize>;

    type Label = miette::LabeledSpan;

    fn expected_input_found<Iter: IntoIterator<Item = Option<char>>>(
        span: Self::Span,
        expected: Iter,
        found: Option<char>,
    ) -> Self {
        let expected = expected.into_iter().filter_map(|ch| ch).map(|ch| format!(", '{ch}'")).collect::<String>();
        match found {
            Some(found) =>
                Self::ExpectedInputFound { 
                    source_code: SOURCE.clone().unwrap(),
                    span: span.into(),
                    expected,
                    found
                },
            None => 
                Self::ExpectedInput {
                    source_code: SOURCE.clone().unwrap(),
                    span: span.into(),
                    expected
                }

        }
    }

    fn with_label(self, label: Self::Label) -> Self {
        // This seems hard if not impossible to implement so leaving a big fat
        //TODO
        self
    }

    fn merge(self, other: Self) -> Self {
        // For this one im just lazy
        //TODO
        self
    }
}