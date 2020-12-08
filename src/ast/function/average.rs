use super::Function;
use crate::prelude::Expression;

/// A representation of the `AVG` function in the database.
#[derive(Debug, Clone, PartialEq)]
pub struct Average<'a> {
    pub(crate) expr: Box<Expression<'a>>,
}

/// Calculates the average value of a numeric column.
///
/// ```rust
/// # use quaint::{ast::*, visitor::{Visitor, Sqlite}};
/// # fn main() -> Result<(), quaint::error::Error> {
/// let query = Select::from_table("users").value(avg("age"));
/// let (sql, _) = Sqlite::build(query)?;
/// assert_eq!("SELECT AVG(`age`) FROM `users`", sql);
/// # Ok(())
/// # }
/// ```
pub fn avg<'a, T>(expr: T) -> Function<'a>
where
    T: Into<Expression<'a>>,
{
    let fun = Average {
        expr: Box::new(expr.into()),
    };
    fun.into()
}
