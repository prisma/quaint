#[cfg(feature = "sqlite")]
use std::convert::TryFrom;

use crate::{
    error::{Error, ErrorKind},
    prelude::ConnectionInfo,
};
use std::sync::Arc;
use std::time::Duration;

use async_trait::async_trait;

use super::{Builder, Connection, ConnectionCreator, PooledConnection};

#[derive(Clone)]
pub struct Quaint {
    // pub(crate) inner: Pool<ConnectionCreator>,
    pub(crate) inner: Arc<Box<dyn PoolManager>>,
    pub(crate) connection_info: Arc<ConnectionInfo>,
    pub(crate) pool_timeout: Option<Duration>,
}

pub struct PoolState {
    pub max_open: u64,
    pub in_use: u64,
}

pub(crate) enum PoolManagerError {
    Timeout,
    BadConn(Error),
    Other(Error),
}

pub(crate) type PoolManagerResult = Result<PooledConnection, PoolManagerError>;

#[async_trait]
pub(crate) trait PoolManager: Send + Sync {
    async fn acquire_with_timeout(&self, timeout: Duration) -> PoolManagerResult;
    async fn acquire(&self) -> PoolManagerResult;
    async fn state(&self) -> PoolState;
}

pub enum PoolLibrary {
    Mobc,
    Deadpool,
}

impl Quaint {
    /// Creates a new builder for a Quaint connection pool with the given
    /// connection string. See the [module level documentation] for details.
    ///
    /// [module level documentation]: index.html
    #[tracing::instrument]
    pub fn builder(url_str: &str) -> crate::Result<Builder> {
        match url_str {
            #[cfg(feature = "sqlite")]
            s if s.starts_with("file") || s.starts_with("sqlite") => {
                let params = crate::connector::SqliteParams::try_from(s)?;

                let manager = ConnectionCreator::Sqlite {
                    url: s.to_string(),
                    db_name: params.db_name,
                };

                let mut builder = Builder::new(s, manager)?;

                if let Some(limit) = params.connection_limit {
                    builder.connection_limit(limit);
                }

                if let Some(max_lifetime) = params.max_connection_lifetime {
                    builder.max_lifetime(max_lifetime);
                }

                if let Some(max_idle_lifetime) = params.max_idle_connection_lifetime {
                    builder.max_idle_lifetime(max_idle_lifetime);
                }

                Ok(builder)
            }
            #[cfg(feature = "mysql")]
            s if s.starts_with("mysql") => {
                let url = crate::connector::MysqlUrl::new(url::Url::parse(s)?)?;
                let connection_limit = url.connection_limit();
                let pool_timeout = url.pool_timeout();
                let max_connection_lifetime = url.max_connection_lifetime();
                let max_idle_connection_lifetime = url.max_idle_connection_lifetime();

                let manager = ConnectionCreator::Mysql { url };
                let mut builder = Builder::new(s, manager)?;

                if let Some(limit) = connection_limit {
                    builder.connection_limit(limit);
                }

                if let Some(timeout) = pool_timeout {
                    builder.pool_timeout(timeout);
                }

                if let Some(max_lifetime) = max_connection_lifetime {
                    builder.max_lifetime(max_lifetime);
                }

                if let Some(max_idle_lifetime) = max_idle_connection_lifetime {
                    builder.max_idle_lifetime(max_idle_lifetime);
                }

                Ok(builder)
            }
            #[cfg(feature = "postgresql")]
            s if s.starts_with("postgres") || s.starts_with("postgresql") => {
                let url = crate::connector::PostgresUrl::new(url::Url::parse(s)?)?;
                let connection_limit = url.connection_limit();
                let pool_timeout = url.pool_timeout();
                let max_connection_lifetime = url.max_connection_lifetime();
                let max_idle_connection_lifetime = url.max_idle_connection_lifetime();

                let manager = ConnectionCreator::Postgres { url };
                let mut builder = Builder::new(s, manager)?;

                if let Some(limit) = connection_limit {
                    builder.connection_limit(limit);
                }

                if let Some(timeout) = pool_timeout {
                    builder.pool_timeout(timeout);
                }

                if let Some(max_lifetime) = max_connection_lifetime {
                    builder.max_lifetime(max_lifetime);
                }

                if let Some(max_idle_lifetime) = max_idle_connection_lifetime {
                    builder.max_idle_lifetime(max_idle_lifetime);
                }

                Ok(builder)
            }
            #[cfg(feature = "mssql")]
            s if s.starts_with("jdbc:sqlserver") || s.starts_with("sqlserver") => {
                let url = crate::connector::MssqlUrl::new(s)?;
                let connection_limit = url.connection_limit();
                let pool_timeout = url.pool_timeout();
                let max_connection_lifetime = url.max_connection_lifetime();
                let max_idle_connection_lifetime = url.max_idle_connection_lifetime();

                let manager = ConnectionCreator::Mssql { url };
                let mut builder = Builder::new(s, manager)?;

                if let Some(limit) = connection_limit {
                    builder.connection_limit(limit);
                }

                if let Some(timeout) = pool_timeout {
                    builder.pool_timeout(timeout);
                }

                if let Some(max_lifetime) = max_connection_lifetime {
                    builder.max_lifetime(max_lifetime);
                }

                if let Some(max_idle_lifetime) = max_idle_connection_lifetime {
                    builder.max_idle_lifetime(max_idle_lifetime);
                }

                Ok(builder)
            }
            _ => unimplemented!("Supported url schemes: file or sqlite, mysql, postgres or postgresql."),
        }
    }

    /// The number of connections in the pool.
    pub async fn capacity(&self) -> u32 {
        self.inner.state().await.max_open as u32
    }

    /// Reserve a connection from the pool.
    #[tracing::instrument(name = "fetch_new_connection_from_pool", skip(self))]
    pub async fn check_out(&self) -> crate::Result<PooledConnection> {
        // let res = match self.pool_timeout {
        //     Some(duration) => crate::connector::metrics::check_out(self.inner.acquire_with_timeout(duration)).await,
        //     None => crate::connector::metrics::check_out(self.inner.acquire()).await,
        // };
        let res = match self.pool_timeout {
            Some(duration) => self.inner.acquire_with_timeout(duration).await,
            None => self.inner.acquire().await,
        };

        let inner = match res {
            Ok(conn) => conn,
            Err(PoolManagerError::Timeout) => {
                let state = self.inner.state().await;
                // We can use unwrap here because a pool timeout has to be set to use a connection pool
                let timeout_duration = self.pool_timeout.unwrap();
                return Err(
                    Error::builder(ErrorKind::pool_timeout(state.max_open, state.in_use, timeout_duration)).build(),
                );
            }
            Err(PoolManagerError::Other(e)) => return Err(e),
            Err(PoolManagerError::BadConn(e)) => {
                let error = Error::builder(ErrorKind::ConnectionError(Box::new(e))).build();
                return Err(error);
            }
        };

        Ok(inner)

        // Ok(PooledConnection { inner })
    }

    /// Info about the connection and underlying database.
    pub fn connection_info(&self) -> &ConnectionInfo {
        &self.connection_info
    }
}
