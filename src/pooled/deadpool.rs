// use std::time::Duration;

// use deadpool::managed::{Manager, Pool, RecycleError, RecycleResult};

// #[cfg(feature = "mssql")]
// use crate::connector::MssqlUrl;
// #[cfg(feature = "mysql")]
// use crate::connector::MysqlUrl;
// #[cfg(feature = "postgresql")]
// use crate::connector::PostgresUrl;
// use crate::{
//     ast,
//     connector::{self, Queryable, Transaction, TransactionCapable},
//     error::Error,
// };
// use async_trait::async_trait;

// use super::{Connection, PoolManager, PoolState, PooledConnection};

// #[doc(hidden)]
// pub enum DPQuaintManager {
//     #[cfg(feature = "mysql")]
//     Mysql { url: MysqlUrl },

//     #[cfg(feature = "postgresql")]
//     Postgres { url: PostgresUrl },

//     #[cfg(feature = "sqlite")]
//     Sqlite { url: String, db_name: String },

//     #[cfg(feature = "mssql")]
//     Mssql { url: MssqlUrl },
// }

// impl DPQuaintManager {
//     async fn check(&self, conn: &mut Connection) -> crate::Result<()> {
//         conn.raw_cmd("SELECT 1").await?;
//         Ok(())
//     }

//     fn validate(&self, conn: &mut Connection) -> bool {
//         conn.is_healthy()
//     }
// }

// #[async_trait]
// impl Manager for DPQuaintManager {
//     type Type = Connection;
//     type Error = Error;

//     async fn create(&self) -> crate::Result<Self::Type> {
//         let conn = match self {
//             #[cfg(feature = "sqlite")]
//             DPQuaintManager::Sqlite { url, .. } => {
//                 use crate::connector::Sqlite;

//                 let conn = Sqlite::new(url)?;

//                 Ok(Box::new(conn) as Self::Type)
//             }

//             #[cfg(feature = "mysql")]
//             DPQuaintManager::Mysql { url } => {
//                 use crate::connector::Mysql;
//                 Ok(Box::new(Mysql::new(url.clone()).await?) as Self::Type)
//             }

//             #[cfg(feature = "postgresql")]
//             DPQuaintManager::Postgres { url } => {
//                 use crate::connector::PostgreSql;
//                 Ok(Box::new(PostgreSql::new(url.clone()).await?) as Self::Type)
//             }

//             #[cfg(feature = "mssql")]
//             DPQuaintManager::Mssql { url } => {
//                 use crate::connector::Mssql;
//                 Ok(Box::new(Mssql::new(url.clone()).await?) as Self::Type)
//             }
//         };

//         conn.iter()
//             .for_each(|_| tracing::debug!("Acquired database connection."));

//         conn
//     }

//     async fn recycle(&self, conn: &mut Connection) -> RecycleResult<Error> {
//         if !conn.is_healthy() {
//             //log::info!(target: "deadpool.postgres", "Connection could not be recycled: {}", e);
//             return Err(RecycleError::StaticMessage("Connection not healthy"));
//         }
//         match self.check(conn).await {
//             Ok(_) => Ok(()),
//             Err(_) => Err(RecycleError::StaticMessage("Connection check failed")),
//         }
//     }
// }

// type DP = Pool<DPQuaintManager>;

// fn test() {}

// #[async_trait]
// impl PoolManager for Pool<DPQuaintManager> {
//     // async fn get_timeout(&self, timeout: Duration) -> crate::Result<PooledConnection> {
//     //     todo!();
//     // }
//     async fn acquire(&self) -> crate::Result<PooledConnection> {
//         let c = self.get().await.unwrap();

//         let pooled_connection = PooledConnection { inner: c };
//         Ok(pooled_connection)
//     }
//     // async fn state(&self) -> PoolState {
//     //     PoolState { max_open: 0, in_use: 0 }
//     // }
// }
