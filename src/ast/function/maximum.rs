use super::Function;
use crate::prelude::Expression;

/// A represention of the `MAX` function in the database.
#[derive(Debug, Clone, PartialEq)]
pub struct Maximum<'a> {
    pub(crate) expr: Box<Expression<'a>>,
}

/// Calculates the maximum value of a numeric column.
///
/// ```rust
/// # use quaint::{ast::*, visitor::{Visitor, Sqlite}};
/// # fn main() -> Result<(), quaint::error::Error> {
/// let query = Select::from_table("users").value(max("age"));
/// let (sql, _) = Sqlite::build(query)?;
/// assert_eq!("SELECT MAX(`age`) FROM `users`", sql);
/// # Ok(())
/// # }
/// ```
pub fn max<'a, T>(expr: T) -> Function<'a>
where
    T: Into<Expression<'a>>,
{
    let func = Maximum {
        expr: Box::new(expr.into()),
    };

    func.into()
}
