use super::Expression;
use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq)]
pub struct Cast<'a> {
    pub(crate) to: Cow<'a, str>,
    pub(crate) expression: Box<Expression<'a>>,
}

pub trait Castable<'a> {
    fn cast(self, to: impl Into<Cow<'a, str>>) -> Expression<'a>;
}
