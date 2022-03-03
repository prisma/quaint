//! # A connection pool to a SQL database.
//!
//! A pool is created through the [`builder`] method, starting from a connection
//! string that allows some of the parameters be delivered by the user.
//!
//! A connection string has the following structure:
//!
//! `connector_type://user:password@host/database?parameters`
//!
//! Connector type can be one of the following:
//!
//! - `sqlite`/`file` opens an SQLite connection.
//! - `mysql` opens a MySQL connection.
//! - `postgres`/`postgresql` opens a PostgreSQL connection.
//!
//! All parameters should be given in the query string format:
//! `?key1=val1&key2=val2`. All parameters are optional.
//!
//! As a special case, Microsoft SQL Server connections use the JDBC URI
//! format:
//!
//! `jdbc:sqlserver://host\instance:port;key1=val1;key2=val2;`
//!
//! ## Common parameters
//!
//! - `connection_limit` defines the maximum number of connections opened to the
//!   database.
//!
//! ## SQLite
//!
//! - `user`/`password` do not do anything and can be emitted.
//! - `host` should point to the database file.
//! - `db_name` parameter should give a name to the database attached for
//!   query namespacing.
//! - `socket_timeout` defined in seconds. Acts as the busy timeout in
//!   SQLite. When set, queries that are waiting for a lock to be released
//!   will return the `Timeout` error after the defined value.
//!
//! ## PostgreSQL
//!
//! - `sslmode` either `disable`, `prefer` or `require`. [Read more](https://docs.rs/tokio-postgres/0.5.0-alpha.1/tokio_postgres/config/enum.SslMode.html)
//! - `sslcert` should point to a PEM certificate file.
//! - `sslidentity` should point to a PKCS12 certificate database.
//! - `sslpassword` the password to open the PKCS12 database.
//! - `sslaccept` either `strict` or `accept_invalid_certs`. If strict, the
//!   certificate needs to be valid and in the CA certificates.
//!   `accept_invalid_certs` accepts any certificate from the server and can
//!   lead to weakened security. Defaults to `accept_invalid_certs`.
//! - `schema` the default search path.
//! - `host` additionally the host can be given as a parameter, typically in
//!   cases when connectiong to the database through a unix socket to
//!   separate the database name from the database path, such as
//!   `postgresql:///dbname?host=/var/run/postgresql`.
//! - `socket_timeout` defined in seconds. If set, a query will return a
//!   `Timeout` error if it fails to resolve before given time.
//! - `connect_timeout` defined in seconds. Connecting to a
//!   database will return a `ConnectTimeout` error if taking more than the
//!   defined value. Defaults to 5 seconds, if set to 0, no timeout.
//! - `pool_timeout` defined in seconds. If all connections are in use, the
//!   database will return a `PoolTimeout` error after waiting for the given time.
//!   If set to zero, no timeout.
//! - `pgbouncer` either `true` or `false`. If set, allows usage with the
//!   pgBouncer connection pool in transaction mode. Additionally a transaction
//!   is required for every query for the mode to work. When starting a new
//!   transaction, a deallocation query `DEALLOCATE ALL` is executed right after
//!   `BEGIN` to avoid possible collisions with statements created in other
//!   sessions.
//! - `statement_cache_size`, number of prepared statements kept cached.
//!   Defaults to 500. If `pgbouncer` mode is enabled, caching is always off.
//! - `options` Specifies command-line options to send to the server at connection start. [Read more](https://www.postgresql.org/docs/current/libpq-connect.html#LIBPQ-CONNECT-OPTIONS)
//!
//! ## MySQL
//!
//! - `sslcert` should point to a PEM certificate file.
//! - `sslidentity` should point to a PKCS12 certificate database.
//! - `sslpassword` the password to open the PKCS12 database.
//! - `sslaccept` either `strict` or `accept_invalid_certs`. If strict, the
//!   certificate needs to be valid and in the CA certificates.
//!   `accept_invalid_certs` accepts any certificate from the server and can
//!   lead to weakened security. Defaults to `strict`.
//! - `socket` needed when connecting to MySQL database through a unix
//!   socket. When set, the host parameter is dismissed.
//! - `socket_timeout` defined in seconds. If set, a query will return a
//!   `Timeout` error if it fails to resolve before given time.
//! - `connect_timeout` defined in seconds. Connecting to a
//!   database will return a `ConnectTimeout` error if taking more than the
//!   defined value. Defaults to 5 seconds, if set to 0, no timeout.
//! - `pool_timeout` defined in seconds. If all connections are in use, the
//!   database will return a `PoolTimeout` error after waiting for the given time.
//!   If set to zero, no timeout.
//! - `statement_cache_size`, number of prepared statements kept cached.
//!   Defaults to 1000. Set to 0 to disable caching.
//!
//! ## Microsoft SQL Server
//!
//! - `encrypt` if set to `true` encrypts all traffic over TLS. If `false`, only
//!   the login details are encrypted. A special value `DANGER_PLAINTEXT` will
//!   disable TLS completely, including sending login credentials as plaintext.
//! - `user` sets the login name.
//! - `password` sets the login password.
//! - `database` sets the database to connect to.
//! - `trustServerCertificate` if set to `true`, accepts any kind of certificate
//!   from the server.
//! - `socketTimeout` defined in seconds. If set, a query will return a
//!   `Timeout` error if it fails to resolve before given time.
//! - `connectTimeout` defined in seconds (default: 5). Connecting to a
//!   database will return a `ConnectTimeout` error if taking more than the
//!   defined value. Defaults to 5 seconds, disabled if set to zero.
//! - `poolTimeout` defined in seconds. If all connections are in use, the
//!   database will return a `Timeout` error after waiting for the given time.
//!   If set to zero, no timeout.
//! - `connectionLimit` defines the maximum number of connections opened to the
//!   database.
//! - `schema` the name of the lookup schema. Only stored to the connection,
//!   must be used in every query to be effective.
//! - `isolationLevel` the transaction isolation level. Possible values:
//!   `READ UNCOMMITTED`, `READ COMMITTED`, `REPEATABLE READ`, `SNAPSHOT`,
//!   `SERIALIZABLE`.
//!
//! To create a new `Quaint` pool connecting to a PostgreSQL database:
//!
//! ``` no_run
//! use quaint::{prelude::*, pooled::Quaint};
//! use std::time::Duration;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), quaint::error::Error> {
//!     let mut builder = Quaint::builder("postgresql://postgres:password@localhost:5432/postgres")?;
//!     builder.connection_limit(5);
//!     builder.max_idle_lifetime(Duration::from_secs(300));
//!     builder.test_on_check_out(true);
//!
//!     let pool = builder.build();
//!     let conn = pool.check_out().await?;
//!     let result = conn.select(Select::default().value(1)).await?;
//!
//!     assert_eq!(
//!         Some(1),
//!         result.into_iter().nth(0).and_then(|row| row[0].as_i64()),
//!     );
//!
//!     Ok(())
//! }
//! ```
//!
//! [`builder`]: struct.Quaint.html#method.builder

// mod deadpool;
// mod manager;

// use builder::Builder;
// pub use manager::*;

mod builder;
mod deadpool_manager;
mod manager;
mod mobc_manager;
mod quaint;
/// The main entry point and an abstraction over database connections and
/// connection handling.
pub use builder::*;
pub use deadpool_manager::*;
pub use manager::*;
pub use mobc_manager::*;
pub use quaint::*;
