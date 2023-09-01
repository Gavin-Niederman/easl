use std::ops::Range;

pub struct Spanned<I, R = usize> {
    pub span: Range<R>,
    pub inner: I,
}


impl<T> Spanned<T> {
    pub fn new(span: Range<usize>, inner: T) -> Self {
        Self { span, inner }
    }

    pub fn add_spans(lhs: Self, rhs: Self) -> Range<usize> {
        let lhs = lhs.span;
        let rhs = rhs.span;
        lhs.start.min(rhs.start)..lhs.end.max(rhs.end)
    }
}

impl<T> Clone for Spanned<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            span: self.span.clone(),
            inner: self.inner.clone(),
        }
    }
}

impl<T> std::fmt::Debug for Spanned<T>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "Spanned {{\n\tinner: {:#?}\n\tspan: {:?}\n}}",
            self.inner, self.span
        )?;
        Ok(())
    }
}

impl<T> PartialEq for Spanned<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}