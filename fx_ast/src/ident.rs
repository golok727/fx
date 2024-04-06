use crate::locator::Span;

#[derive(Debug, Clone)]
pub struct Ident {
    pub name: String,
    pub span: Span,
}

impl Ident {
    pub fn new(name: &'static str, span: Span) -> Self {
        Self {
            name: name.to_string(),
            span,
        }
    }
}
