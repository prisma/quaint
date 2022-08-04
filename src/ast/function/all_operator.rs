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
/// # use quaint::{ast::*, col, visitor::{Visitor, Postgres}};
/// # fn main() -> Result<(), quaint::error::Error> {
/// let query = Select::from_table("users").so_that(col!("name").equals(all_operator(col!("list"))));
/// let (sql, _) = Postgres::build(query)?;
/// assert_eq!(r#"SELECT "users".* FROM "users" WHERE "name" = ALL("list")"#, sql);
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
