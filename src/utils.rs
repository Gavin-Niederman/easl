use std::ops::Range;
use pest::Span;

pub fn pest_span_to_range<T: From<usize>>(span: Span<'_>) -> Range<T> {
    span.start().into()..span.end().into()
}
