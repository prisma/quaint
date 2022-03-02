#[cfg(feature = "mssql")]
use crate::connector::MssqlUrl;
#[cfg(feature = "mysql")]
use crate::connector::MysqlUrl;
#[cfg(feature = "postgresql")]
use crate::connector::PostgresUrl;
use crate::{
    ast,
    connector::{self, Queryable, Transaction, TransactionCapable},
};
use async_trait::async_trait;

pub type Connection = Box<dyn Queryable>;

/// A connection from the pool. Implements
/// [Queryable](connector/trait.Queryable.html).
pub struct PooledConnection {
    pub(crate) inner: Connection,
}

// /// A connection from the pool. Implements
// /// [Queryable](connector/trait.Queryable.html).
// pub struct PooledConnection {
//     pub(crate) inner: MobcPooled<ConnectionCreator>,
// }

impl TransactionCapable for PooledConnection {}

#[async_trait]
impl Queryable for PooledConnection {
    async fn query(&self, q: ast::Query<'_>) -> crate::Result<connector::ResultSet> {
        self.inner.query(q).await
    }

    async fn execute(&self, q: ast::Query<'_>) -> crate::Result<u64> {
        self.inner.execute(q).await
    }

    async fn query_raw(&self, sql: &str, params: &[ast::Value<'_>]) -> crate::Result<connector::ResultSet> {
        self.inner.query_raw(sql, params).await
    }

    async fn execute_raw(&self, sql: &str, params: &[ast::Value<'_>]) -> crate::Result<u64> {
        self.inner.execute_raw(sql, params).await
    }

    async fn raw_cmd(&self, cmd: &str) -> crate::Result<()> {
        self.inner.raw_cmd(cmd).await
    }

    async fn version(&self) -> crate::Result<Option<String>> {
        self.inner.version().await
    }

    async fn server_reset_query(&self, tx: &Transaction<'_>) -> crate::Result<()> {
        self.inner.server_reset_query(tx).await
    }

    fn begin_statement(&self) -> &'static str {
        self.inner.begin_statement()
    }

    fn is_healthy(&self) -> bool {
        self.inner.is_healthy()
    }
}

#[doc(hidden)]
pub enum ConnectionCreator {
    #[cfg(feature = "mysql")]
    Mysql { url: MysqlUrl },

    #[cfg(feature = "postgresql")]
    Postgres { url: PostgresUrl },

    #[cfg(feature = "sqlite")]
    Sqlite { url: String, db_name: String },

    #[cfg(feature = "mssql")]
    Mssql { url: MssqlUrl },
}

impl ConnectionCreator {
    pub async fn connect(&self) -> crate::Result<Connection> {
        let conn = match self {
            #[cfg(feature = "sqlite")]
            ConnectionCreator::Sqlite { url, .. } => {
                use crate::connector::Sqlite;

                let conn = Sqlite::new(url)?;

                Ok(Box::new(conn) as Connection)
            }

            #[cfg(feature = "mysql")]
            ConnectionCreator::Mysql { url } => {
                use crate::connector::Mysql;
                Ok(Box::new(Mysql::new(url.clone()).await?) as Connection)
            }

            #[cfg(feature = "postgresql")]
            ConnectionCreator::Postgres { url } => {
                use crate::connector::PostgreSql;
                Ok(Box::new(PostgreSql::new(url.clone()).await?) as Connection)
            }

            #[cfg(feature = "mssql")]
            ConnectionCreator::Mssql { url } => {
                use crate::connector::Mssql;
                Ok(Box::new(Mssql::new(url.clone()).await?) as Connection)
            }
        };

        conn.iter()
            .for_each(|_| tracing::debug!("Acquired database connection."));

        conn
    }
}
