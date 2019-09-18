use crate::ast::DatabaseValue;

#[derive(Debug, Clone, PartialEq)]
pub struct Cast<'a> {
    pub(crate) expr: Box<DatabaseValue<'a>>,
    pub(crate) tpe: &'a str,
}

/// Convert an expression of one type to an expression of another type.
///
/// ```rust
/// # use prisma_query::{ast::*, visitor::{Visitor, Sqlite}};
/// let query = Select::from_table("users").value(cast("3", "INTEGER"));
/// let (sql, params) = Sqlite::build(query);
/// assert_eq!("SELECT CAST(? AS INTEGER) FROM `users`", sql);
/// assert_eq!(&[ParameterizedValue::Text("3".into())], params.as_slice());
/// ```
#[inline]
pub fn cast<'a, T>(expr: T, tpe: &'a str) -> Cast<'a>
where
    T: Into<DatabaseValue<'a>>,
{
    Cast {
        expr: Box::new(expr.into()),
        tpe,
    }
}
