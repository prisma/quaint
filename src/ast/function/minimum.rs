use super::Function;
use crate::prelude::Expression;

/// A represention of the `MIN` function in the database.
#[derive(Debug, Clone, PartialEq)]
pub struct Minimum<'a> {
    pub(crate) expr: Box<Expression<'a>>,
}

/// Calculates the minimum value of a numeric column.
///
/// ```rust
/// # use quaint::{ast::*, visitor::{Visitor, Sqlite}};
/// # fn main() -> Result<(), quaint::error::Error> {
/// let query = Select::from_table("users").value(min("age"));
/// let (sql, _) = Sqlite::build(query)?;
/// assert_eq!("SELECT MIN(`age`) FROM `users`", sql);
/// # Ok(())
/// # }
/// ```
pub fn min<'a, T>(expr: T) -> Function<'a>
where
    T: Into<Expression<'a>>,
{
    let fun = Minimum {
        expr: Box::new(expr.into()),
    };
    fun.into()
}
