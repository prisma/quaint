use super::Function;
use crate::ast::Expression;

/// A representation of the `AllOperator` operator in the database.
#[derive(Debug, Clone, PartialEq)]
pub struct AllOperator<'a> {
    pub(crate) expr: Expression<'a>,
}

/// Wraps an expression into the ALL operator.
///
/// ```rust
/// # use quaint::{ast::*, visitor::{Visitor, Postgres}};
/// # fn main() -> Result<(), quaint::error::Error> {
/// let query = Select::from_table("users").so_that(col!("name").equals(array_all(col!("list"))));
/// let (sql, _) = Sqlite::build(query)?;
/// assert_eq!(r#"SELECT * FROM "users" WHERE "name" = ALL("list")"#, sql);
/// # Ok(())
/// # }
/// ```
pub fn all_operator<'a, T>(expr: T) -> Function<'a>
where
    T: Into<Expression<'a>>,
{
    let fun = AllOperator { expr: expr.into() };

    fun.into()
}
