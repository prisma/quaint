use super::{mobc_manager::MobcManager, ConnectionCreator, Quaint};
use crate::prelude::ConnectionInfo;
use std::{sync::Arc, time::Duration};

use mobc::Pool as MobcPool;

#[derive(PartialEq)]
pub enum PoolLibrary {
    Mobc,
    Deadpool,
}

/// A `Builder` to construct an instance of a [`Quaint`] pool.
///
/// [`Quaint`]: pooled.Quaint
pub struct Builder {
    connection_creator: ConnectionCreator,
    connection_info: ConnectionInfo,
    connection_limit: usize,
    max_idle: Option<u64>,
    max_idle_lifetime: Option<Duration>,
    max_lifetime: Option<Duration>,
    health_check_interval: Option<Duration>,
    test_on_check_out: bool,
    pool_timeout: Option<Duration>,
    pool_library: PoolLibrary,
}

impl Builder {
    pub(crate) fn new(url: &str, connection_creator: ConnectionCreator) -> crate::Result<Self> {
        let connection_limit = num_cpus::get_physical() * 2 + 1;
        let connection_info = ConnectionInfo::from_url(url)?;

        Ok(Self {
            connection_creator,
            connection_info,
            connection_limit,
            max_idle: None,
            max_idle_lifetime: None,
            max_lifetime: None,
            health_check_interval: None,
            test_on_check_out: false,
            pool_timeout: None,
            pool_library: PoolLibrary::Mobc,
        })
    }

    /// The maximum number of connections in the pool.
    ///
    /// - Defaults to two times the number of physical cores plus one.
    pub fn connection_limit(&mut self, connection_limit: usize) {
        self.connection_limit = connection_limit;
    }

    /// The maximum number of idle connections the pool can contain at the same time. If a
    /// connection goes idle (a query returns) and there are already this number of idle connections
    /// in the pool, a connection will be closed immediately. Consider using `max_idle_lifetime` to
    /// close idle connections less aggressively.
    ///
    /// - Defaults to the same value as `connection_limit`.
    pub fn max_idle(&mut self, max_idle: u64) {
        self.max_idle = Some(max_idle);
    }

    /// A timeout for acquiring a connection with the [`check_out`] method. If
    /// not set, the method never times out.
    ///
    /// # Panics
    ///
    /// Panics if `pool_timeout` is zero.
    ///
    /// [`check_out`]: struct.Quaint.html#method.check_out
    pub fn pool_timeout(&mut self, pool_timeout: Duration) {
        assert_ne!(pool_timeout, Duration::from_secs(0), "pool_timeout must be positive");

        self.pool_timeout = Some(pool_timeout);
    }

    /// A time how long a connection can be kept in the pool before
    /// replaced with a new one. The reconnect happens in the next
    /// [`check_out`].
    ///
    /// - Defaults to not set, meaning connections are kept forever.
    ///
    /// # Panics
    ///
    /// Panics if `max_lifetime` is zero.
    ///
    /// [`check_out`]: struct.Quaint.html#method.check_out
    pub fn max_lifetime(&mut self, max_lifetime: Duration) {
        self.max_lifetime = Some(max_lifetime);
    }

    /// A time how long an idling connection can be kept in the pool before
    /// replaced with a new one. The reconnect happens in the next
    /// [`check_out`].
    ///
    /// - Defaults to 300 seconds
    ///
    /// # Panics
    ///
    /// Panics if `max_idle_lifetime` is zero.
    ///
    /// [`check_out`]: struct.Quaint.html#method.check_out
    pub fn max_idle_lifetime(&mut self, max_idle_lifetime: Duration) {
        self.max_idle_lifetime = Some(max_idle_lifetime);
    }

    /// Perform a health check before returning a connection from the
    /// [`check_out`]. If the health check fails, a few reconnects are tried
    /// before returning the error and dropping the broken connection from the
    /// pool.
    ///
    /// - Defaults to `false`, meaning connections are never tested on
    /// `check_out`.
    ///
    /// [`check_out`]: struct.Quaint.html#method.check_out
    pub fn test_on_check_out(&mut self, test_on_check_out: bool) {
        self.test_on_check_out = test_on_check_out;
    }

    /// Sets the interval how often a connection health will be tested when
    /// checking out from the pool. Must be used together with
    /// [`test_on_check_out`] set to `true`, otherwise does nothing.
    ///
    /// - Defaults to not set, meaning a test is performed on every `check_out`.
    ///
    /// # Panics
    ///
    /// Panics if `health_check_interval` is zero.
    ///
    /// [`test_on_check_out`]: #method.test_on_check_out
    pub fn health_check_interval(&mut self, health_check_interval: Duration) {
        self.health_check_interval = Some(health_check_interval);
    }

    /// Sets the connection pool library to use to manage the connections
    pub fn set_pool_library(&mut self, pool_library: PoolLibrary) {
        self.pool_library = pool_library
    }

    /// Consume the builder and create a new instance of a pool.
    pub fn build(self) -> Quaint {
        let connection_info = std::sync::Arc::new(self.connection_info);
        Self::log_start(&connection_info, self.connection_limit);

        let inner = if self.pool_library == PoolLibrary::Mobc {
            let manager = MobcManager {
                connection_creator: self.connection_creator,
            };
            MobcPool::builder()
                .max_open(self.connection_limit as u64)
                .max_idle(self.max_idle.unwrap_or(self.connection_limit as u64))
                .max_idle_lifetime(self.max_idle_lifetime)
                .max_lifetime(self.max_lifetime)
                .get_timeout(None) // we handle timeouts here
                .health_check_interval(self.health_check_interval)
                .test_on_check_out(self.test_on_check_out)
                .build(manager)
        } else {
            todo!("")
        };

        Quaint {
            inner: Arc::new(Box::new(inner)),
            connection_info,
            pool_timeout: self.pool_timeout,
        }
    }

    fn log_start(info: &ConnectionInfo, connection_limit: usize) {
        let family = info.sql_family();
        let pg_bouncer = if info.pg_bouncer() { " in PgBouncer mode" } else { "" };

        tracing::info!(
            "Starting a {} pool with {} connections{}.",
            family,
            connection_limit,
            pg_bouncer
        );
    }
}

#[cfg(test)]
mod tests {
    use crate::pooled::Quaint;

    #[tokio::test]
    #[cfg(feature = "mysql")]
    async fn mysql_default_connection_limit() {
        let conn_string = std::env::var("TEST_MYSQL").expect("TEST_MYSQL connection string not set.");

        let pool = Quaint::builder(&conn_string).unwrap().build();

        assert_eq!(num_cpus::get_physical() * 2 + 1, pool.capacity().await as usize);
    }

    #[tokio::test]
    #[cfg(feature = "mysql")]
    async fn mysql_custom_connection_limit() {
        let conn_string = format!(
            "{}?connection_limit=10",
            std::env::var("TEST_MYSQL").expect("TEST_MYSQL connection string not set.")
        );

        let pool = Quaint::builder(&conn_string).unwrap().build();

        assert_eq!(10, pool.capacity().await as usize);
    }

    #[tokio::test]
    #[cfg(feature = "postgresql")]
    async fn psql_default_connection_limit() {
        let conn_string = std::env::var("TEST_PSQL").expect("TEST_PSQL connection string not set.");

        let pool = Quaint::builder(&conn_string).unwrap().build();

        assert_eq!(num_cpus::get_physical() * 2 + 1, pool.capacity().await as usize);
    }

    #[tokio::test]
    #[cfg(feature = "postgresql")]
    async fn psql_custom_connection_limit() {
        let conn_string = format!(
            "{}?connection_limit=10",
            std::env::var("TEST_PSQL").expect("TEST_PSQL connection string not set.")
        );

        let pool = Quaint::builder(&conn_string).unwrap().build();

        assert_eq!(10, pool.capacity().await as usize);
    }

    #[tokio::test]
    #[cfg(feature = "mssql")]
    async fn mssql_default_connection_limit() {
        let conn_string = std::env::var("TEST_MSSQL").expect("TEST_MSSQL connection string not set.");

        let pool = Quaint::builder(&conn_string).unwrap().build();

        assert_eq!(num_cpus::get_physical() * 2 + 1, pool.capacity().await as usize);
    }

    #[tokio::test]
    #[cfg(feature = "mssql")]
    async fn mssql_custom_connection_limit() {
        let conn_string = format!(
            "{};connectionLimit=10",
            std::env::var("TEST_MSSQL").expect("TEST_MSSQL connection string not set.")
        );

        let pool = Quaint::builder(&conn_string).unwrap().build();

        assert_eq!(10, pool.capacity().await as usize);
    }

    #[tokio::test]
    #[cfg(feature = "sqlite")]
    async fn test_default_connection_limit() {
        let conn_string = format!("file:db/test.db",);
        let pool = Quaint::builder(&conn_string).unwrap().build();

        assert_eq!(num_cpus::get_physical() * 2 + 1, pool.capacity().await as usize);
    }

    #[tokio::test]
    #[cfg(feature = "sqlite")]
    async fn test_custom_connection_limit() {
        let conn_string = format!("file:db/test.db?connection_limit=10",);
        let pool = Quaint::builder(&conn_string).unwrap().build();

        assert_eq!(10, pool.capacity().await as usize);
    }
}
