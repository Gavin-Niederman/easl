use miette::SourceSpan;
use pest::Span;

pub fn pest_span_to_miette_span(span: Span<'_>, source: &str) -> SourceSpan {
    miette::SourceSpan::new(span.start().into(), (span.end() - span.start()).into())
}
