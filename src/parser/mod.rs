use std::ops::Range;
use ::chumsky::prelude::*;
use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

use self::ast::{IdentifierMap, Statement, Spanned};

pub mod ast;
pub mod tc;
pub mod chumsky;

pub fn parse(source: &str) -> Result<(Vec<Spanned<Statement>>, IdentifierMap), Vec<ParserError>> {
    let mut ident_map = IdentifierMap::new();
    Ok((chumsky::parser(&mut ident_map).parse(source)?, ident_map))
}

#[derive(Debug, Error, Diagnostic)]
pub enum ParserError {
    #[error("Expected one of: {expected} found {found}")]
    #[diagnostic(code(easl::parser::expected_input_found))]
    ExpectedInputFound {
        #[label]
        span: SourceSpan,

        expected: String,
        found: char,
    },
    #[error("Expected one of: {expected}")]
    #[diagnostic(code(easl::parser::expected_input))]
    ExpectedInput {
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
        let expected = expected.into_iter().filter_map(|ch| ch).map(|ch| ", '{ch}'").collect::<String>();
        match found {
            Some(found) =>
                Self::ExpectedInputFound { 
                    span: span.into(),
                    expected,
                    found
                },
            None => 
                Self::ExpectedInput {
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