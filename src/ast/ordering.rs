use crate::ast::{Column, DatabaseValue};

pub type OrderDefinition<'a> = (DatabaseValue<'a>, Option<Order>);

/// A list of definitions for the `ORDER BY` statement
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Ordering<'a>(pub Vec<OrderDefinition<'a>>);

impl<'a> Ordering<'a> {
    #[doc(hidden)]
    pub fn append(mut self, value: OrderDefinition<'a>) -> Self {
        self.0.push(value);
        self
    }

    #[inline]
    pub fn new(values: Vec<OrderDefinition<'a>>) -> Self {
        Self(values)
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

/// The ordering direction
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Order {
    /// Ascending
    Asc,
    /// Descending
    Desc,
}

/// An item that can be used in the `ORDER BY` statement
pub trait Orderable<'a>
where
    Self: Sized,
{
    /// Order by `self` in the given order
    fn order(self, order: Option<Order>) -> OrderDefinition<'a>;

    /// Change the order to `ASC`
    #[inline]
    fn ascend(self) -> OrderDefinition<'a> {
        self.order(Some(Order::Asc))
    }

    /// Change the order to `DESC`
    #[inline]
    fn descend(self) -> OrderDefinition<'a> {
        self.order(Some(Order::Desc))
    }
}

/// Convert the value into an order definition with order item and direction
pub trait IntoOrderDefinition<'a> {
    fn into_order_definition(self) -> OrderDefinition<'a>;
}

impl<'a> IntoOrderDefinition<'a> for &'a str {
    #[inline]
    fn into_order_definition(self) -> OrderDefinition<'a> {
        let column: Column<'a> = self.into();
        (column.into(), None)
    }
}

impl<'a> IntoOrderDefinition<'a> for Column<'a> {
    #[inline]
    fn into_order_definition(self) -> OrderDefinition<'a> {
        (self.into(), None)
    }
}

impl<'a> IntoOrderDefinition<'a> for OrderDefinition<'a> {
    #[inline]
    fn into_order_definition(self) -> OrderDefinition<'a> {
        self
    }
}

impl<'a> Orderable<'a> for Column<'a> {
    #[inline]
    fn order(self, order: Option<Order>) -> OrderDefinition<'a> {
        (self.into(), order)
    }
}

impl<'a> Orderable<'a> for &'a str {
    #[inline]
    fn order(self, order: Option<Order>) -> OrderDefinition<'a> {
        let column: Column<'a> = self.into();
        column.order(order)
    }
}

impl<'a> Orderable<'a> for (&'a str, &'a str) {
    #[inline]
    fn order(self, order: Option<Order>) -> OrderDefinition<'a> {
        let column: Column<'a> = self.into();
        column.order(order)
    }
}
