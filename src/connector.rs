//! A set of abstractions for database connections.
//!
//! Provides traits for database querying and executing, and for spawning
//! transactions.
//!
//! Connectors for [MySQL](struct.Mysql.html),
//! [PostgreSQL](struct.PostgreSql.html), [SQLite](struct.Sqlite.html) and [SQL
//! Server](struct.Mssql.html) connect to the corresponding databases and
//! implement the [Queryable](trait.Queryable.html) trait for generalized
//! querying interface.

mod connection_info;
pub(crate) mod metrics;
mod queryable;
mod result_set;
mod transaction;
mod type_identifier;

#[cfg(feature = "mssql")]
pub(crate) mod mssql;
#[cfg(feature = "mysql")]
pub(crate) mod mysql;
#[cfg(feature = "postgresql")]
pub(crate) mod postgres;
#[cfg(feature = "sqlite")]
pub(crate) mod sqlite;

#[cfg(any(feature = "mssql", feature = "postgresql", feature = "mysql"))]
use std::time::Duration;

#[cfg(any(feature = "mssql", feature = "postgresql", feature = "mysql"))]
use crate::error::{Error, ErrorKind};

#[cfg(feature = "mysql")]
pub use self::mysql::*;
#[cfg(feature = "postgresql")]
pub use self::postgres::*;
pub use self::result_set::*;
pub use connection_info::*;
#[cfg(any(feature = "mssql", feature = "postgresql", feature = "mysql"))]
use futures::Future;
#[cfg(feature = "mssql")]
pub use mssql::*;
pub use queryable::*;
#[cfg(feature = "sqlite")]
pub use sqlite::*;
pub use transaction::*;
#[cfg(any(feature = "sqlite", feature = "mysql", feature = "postgresql"))]
#[allow(unused_imports)]
pub(crate) use type_identifier::*;

#[cfg(any(feature = "mssql", feature = "postgresql", feature = "mysql"))]
async fn connect_timeout<T, F, E>(duration: Option<Duration>, f: F) -> crate::Result<T>
where
    F: Future<Output = std::result::Result<T, E>>,
    E: Into<Error>,
{
    match duration {
        Some(duration) => match tokio::time::timeout(duration, f).await {
            Ok(Ok(result)) => Ok(result),
            Ok(Err(err)) => Err(err.into()),
            Err(_) => {
                let kind = ErrorKind::connect_timeout(format!("Could not connect in {}s.", duration.as_secs()));
                Err(Error::builder(kind).build())
            }
        },
        None => match f.await {
            Ok(result) => Ok(result),
            Err(err) => Err(err.into()),
        },
    }
}
