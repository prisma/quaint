use enumflags2::{bitflags, BitFlags};
use std::borrow::Cow;

#[bitflags]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
enum CastDatabase {
    Postgres = 1 << 0,
    Mysql = 1 << 1,
    Mssql = 1 << 2,
}

/// A typecast for an expression.
///
/// By default, casting is performed on all databases. To restrict this
/// behavior, use the corresponding methods
/// [on_postgres](struct.CastType.html#method.on_postgres),
/// [on_mysql](struct.CastType.html#method.on_mysql) or
/// [on_sql_server](struct.CastType.html#method.on_sql_server).
///
/// Always a no-op on SQLite.
#[derive(Debug, Clone, PartialEq)]
pub struct CastType<'a> {
    kind: CastKind<'a>,
    on_databases: BitFlags<CastDatabase>,
}

impl<'a> CastType<'a> {
    /// A 16-bit integer.
    ///
    /// - PostgreSQL: `int2`
    /// - MySQL: `signed`
    /// - SQL Server: `smallint`
    pub fn int2() -> Self {
        Self {
            kind: CastKind::Int2,
            on_databases: BitFlags::all(),
        }
    }

    /// A 32-bit integer (int)
    ///
    /// - PostgreSQL: `int4`
    /// - MySQL: `signed`
    /// - SQL Server: `int`
    pub fn int4() -> Self {
        Self {
            kind: CastKind::Int4,
            on_databases: BitFlags::all(),
        }
    }

    /// A 64-bit integer (bigint)
    ///
    /// - PostgreSQL: `int8`
    /// - MySQL: `signed`
    /// - SQL Server: `bigint`
    pub fn int8() -> Self {
        Self {
            kind: CastKind::Int8,
            on_databases: BitFlags::all(),
        }
    }

    /// A 32-bit floating point number
    ///
    /// - PostgreSQL: `float4`
    /// - MySQL: `decimal`
    /// - SQL Server: `real`
    pub fn float4() -> Self {
        Self {
            kind: CastKind::Float4,
            on_databases: BitFlags::all(),
        }
    }

    /// A 64-bit floating point number
    ///
    /// - PostgreSQL: `float8`
    /// - MySQL: `decimal`
    /// - SQL Server: `float`
    pub fn float8() -> Self {
        Self {
            kind: CastKind::Float8,
            on_databases: BitFlags::all(),
        }
    }

    /// An arbitrary-precision numeric type
    ///
    /// - PostgreSQL: `numeric`
    /// - MySQL: `decimal`
    /// - SQL Server: `numeric`
    pub fn decimal() -> Self {
        Self {
            kind: CastKind::Decimal,
            on_databases: BitFlags::all(),
        }
    }

    /// True or false (or a bit)
    ///
    /// - PostgreSQL: `boolean`
    /// - MySQL: `unsigned`
    /// - SQL Server: `bit`
    pub fn boolean() -> Self {
        Self {
            kind: CastKind::Boolean,
            on_databases: BitFlags::all(),
        }
    }

    /// A unique identifier
    ///
    /// - PostgreSQL: `uuid`
    /// - MySQL: `char`
    /// - SQL Server: `uniqueidentifier`
    pub fn uuid() -> Self {
        Self {
            kind: CastKind::Uuid,
            on_databases: BitFlags::all(),
        }
    }

    /// Json data
    ///
    /// - PostgreSQL: `json`
    /// - MySQL: `nchar`
    /// - SQL Server: `nvarchar`
    pub fn json() -> Self {
        Self {
            kind: CastKind::Json,
            on_databases: BitFlags::all(),
        }
    }

    /// Jsonb data
    ///
    /// - PostgreSQL: `jsonb`
    /// - MySQL: `nchar`
    /// - SQL Server: `nvarchar`
    pub fn jsonb() -> Self {
        Self {
            kind: CastKind::Jsonb,
            on_databases: BitFlags::all(),
        }
    }

    /// Date value
    ///
    /// - PostgreSQL: `date`
    /// - MySQL: `date`
    /// - SQL Server: `date`
    pub fn date() -> Self {
        Self {
            kind: CastKind::Date,
            on_databases: BitFlags::all(),
        }
    }

    /// Time value
    ///
    /// - PostgreSQL: `time`
    /// - MySQL: `time`
    /// - SQL Server: `time`
    pub fn time() -> Self {
        Self {
            kind: CastKind::Time,
            on_databases: BitFlags::all(),
        }
    }

    /// Datetime value
    ///
    /// - PostgreSQL: `datetime`
    /// - MySQL: `datetime`
    /// - SQL Server: `datetime2`
    pub fn datetime() -> Self {
        Self {
            kind: CastKind::DateTime,
            on_databases: BitFlags::all(),
        }
    }

    /// Byte blob
    ///
    /// - PostgreSQL: `bytea`
    /// - MySQL: `binary`
    /// - SQL Server: `bytes`
    pub fn bytes() -> Self {
        Self {
            kind: CastKind::Bytes,
            on_databases: BitFlags::all(),
        }
    }

    /// Textual data
    ///
    /// - PostgreSQL: `text`
    /// - MySQL: `nchar`
    /// - SQL Server: `nvarchar`
    pub fn text() -> Self {
        Self {
            kind: CastKind::Text,
            on_databases: BitFlags::all(),
        }
    }

    /// Creates a new custom cast type.
    pub fn custom(r#type: impl Into<Cow<'a, str>>) -> Self {
        Self {
            kind: CastKind::Custom(r#type.into()),
            on_databases: BitFlags::all(),
        }
    }

    /// Perform the given cast on PostgreSQL.
    pub fn on_postgres(mut self) -> Self {
        self.maybe_clear_databases();
        self.on_databases.insert(CastDatabase::Postgres);

        self
    }

    /// Perform the given cast on MySQL.
    pub fn on_mysql(mut self) -> Self {
        self.maybe_clear_databases();
        self.on_databases.insert(CastDatabase::Mysql);

        self
    }

    /// Perform the given cast on SQL Server.
    pub fn on_sql_server(mut self) -> Self {
        self.maybe_clear_databases();
        self.on_databases.insert(CastDatabase::Mssql);

        self
    }

    #[cfg(feature = "postgresql")]
    pub(crate) fn postgres_enabled(&self) -> bool {
        self.on_databases.contains(CastDatabase::Postgres)
    }

    #[cfg(feature = "mysql")]
    pub(crate) fn mysql_enabled(&self) -> bool {
        self.on_databases.contains(CastDatabase::Mysql)
    }

    #[cfg(feature = "mssql")]
    pub(crate) fn mssql_enabled(&self) -> bool {
        self.on_databases.contains(CastDatabase::Mssql)
    }

    #[cfg(any(feature = "mssql", feature = "mysql", feature = "mssql"))]
    pub(crate) fn kind(&self) -> &CastKind<'a> {
        &self.kind
    }

    fn maybe_clear_databases(&mut self) {
        if self.on_databases.is_all() {
            self.on_databases.remove(BitFlags::all());
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum CastKind<'a> {
    Int2,
    Int4,
    Int8,
    Float4,
    Float8,
    Decimal,
    Boolean,
    Uuid,
    Json,
    Jsonb,
    Date,
    Time,
    DateTime,
    Bytes,
    Text,
    Custom(Cow<'a, str>),
}

/// An item that can be cast to a different type.
pub trait Castable<'a, T>
where
    T: Sized,
{
    /// Map the result of the underlying item into a different type.
    fn cast_as(self, r#type: CastType<'a>) -> T;
}
