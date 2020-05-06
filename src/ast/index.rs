use super::{Column, Table};

#[derive(Debug, PartialEq, Clone)]
pub enum IndexDefinition<'a> {
    Single(Column<'a>),
    Compound(Vec<Column<'a>>),
}

impl<'a> IndexDefinition<'a> {
    pub(crate) fn set_table<T>(self, table: T) -> Self
    where
        T: Into<Table<'a>>,
    {
        let table = table.into();

        match self {
            Self::Compound(columns) => {
                let cols = columns.into_iter().map(|c| c.table(table.clone())).collect();

                Self::Compound(cols)
            }
            Self::Single(column) => Self::Single(column.table(table)),
        }
    }
}

impl<'a, T> From<T> for IndexDefinition<'a>
where
    T: Into<Column<'a>>,
{
    fn from(s: T) -> Self {
        Self::Single(s.into())
    }
}

impl<'a, T> From<Vec<T>> for IndexDefinition<'a>
where
    T: Into<Column<'a>>,
{
    fn from(s: Vec<T>) -> Self {
        Self::Compound(s.into_iter().map(|c| c.into()).collect())
    }
}
