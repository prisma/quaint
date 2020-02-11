use crate::ast::{ConditionTree, Expression};

/// `AND`, `OR` and `NOT` conjuctive implementations.
pub trait Conjuctive<'a> {
    /// Builds an `AND` condition having `self` as the left leaf and `other` as the right.
    ///
    /// ```rust
    /// # use quaint::{ast::*, visitor::{Visitor, Sqlite}};
    /// assert_eq!(
    ///     "foo".equals("bar").and("wtf".less_than(3)),
    ///     ConditionTree::And(vec![
    ///         Expression::from("foo".equals("bar")),
    ///         Expression::from("wtf".less_than(3))
    ///     ])
    /// )
    /// ```
    fn and<E>(self, other: E) -> ConditionTree<'a>
    where
        E: Into<Expression<'a>>;

    /// Builds an `OR` condition having `self` as the left leaf and `other` as the right.
    ///
    /// ```rust
    /// # use quaint::{ast::*, visitor::{Visitor, Sqlite}};
    /// assert_eq!(
    ///     "foo".equals("bar").or("wtf".less_than(3)),
    ///     ConditionTree::Or(vec![
    ///         Expression::from("foo".equals("bar")),
    ///         Expression::from("wtf".less_than(3))
    ///     ])
    /// )
    /// ```
    fn or<E>(self, other: E) -> ConditionTree<'a>
    where
        E: Into<Expression<'a>>;

    /// Builds a `NOT` condition having `self` as the condition.
    ///
    /// ```rust
    /// # use quaint::{ast::*, visitor::{Visitor, Sqlite}};
    /// assert_eq!(
    ///     "foo".equals("bar").not(),
    ///     ConditionTree::not("foo".equals("bar"))
    /// )
    /// ```
    fn not(self) -> ConditionTree<'a>;
}

impl<'a, T> Conjuctive<'a> for T
where
    T: Into<Expression<'a>>,
{
    #[inline]
    fn and<E>(self, other: E) -> ConditionTree<'a>
    where
        E: Into<Expression<'a>>,
    {
        ConditionTree::And(vec![self.into(), other.into()])
    }

    #[inline]
    fn or<E>(self, other: E) -> ConditionTree<'a>
    where
        E: Into<Expression<'a>>,
    {
        ConditionTree::Or(vec![self.into(), other.into()])
    }

    #[inline]
    fn not(self) -> ConditionTree<'a> {
        ConditionTree::not(self.into())
    }
}
