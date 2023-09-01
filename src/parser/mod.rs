use std::ops::Range;

use chumsky::Error;
use miette::Diagnostic;
use thiserror::Error;

pub mod passes;

pub fn parse(source: &str) -> () {}

#[derive(Error, Diagnostic, Debug)]
pub enum ParserError {}

impl Error<char> for ParserError {
    type Label = miette::LabeledSpan;
    type Span = (String, Range<usize>);

    fn expected_input_found<Iter: IntoIterator<Item = Option<char>>>(
        span: Self::Span,
        expected: Iter,
        found: Option<char>,
    ) -> Self {
        todo!();
    }

    fn merge(self, other: Self) -> Self {
        todo!()
    }

    fn unclosed_delimiter(
        unclosed_span: Self::Span,
        unclosed: char,
        span: Self::Span,
        expected: char,
        found: Option<char>,
    ) -> Self {
        todo!()
    }

    fn with_label(self, label: Self::Label) -> Self {
        todo!()
    }
}
