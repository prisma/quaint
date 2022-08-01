use super::Function;
use crate::ast::Expression;

/// A representation of the `AnyOperator` operator in the database.
#[derive(Debug, Clone, PartialEq)]
pub struct AnyOperator<'a> {
    pub(crate) expr: Expression<'a>,
}

/// Wraps an expression into the ALL operator.
///
/// ```rust
/// # use quaint::{ast::*, visitor::{Visitor, Postgres}};
/// # fn main() -> Result<(), quaint::error::Error> {
/// let query = Select::from_table("users").so_that(col!("name").equals(array_any(col!("list"))));
/// let (sql, _) = Sqlite::build(query)?;
/// assert_eq!(r#"SELECT * FROM "users" WHERE "name" = ANY("list")"#, sql);
/// # Ok(())
/// # }
/// ```
pub fn any_operator<'a, T>(expr: T) -> Function<'a>
where
    T: Into<Expression<'a>>,
{
    let fun = AnyOperator { expr: expr.into() };

    fun.into()
}
