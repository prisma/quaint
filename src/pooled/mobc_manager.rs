use async_trait::async_trait;

use crate::{
    connector::{self, ConnectionInfo},
    error::{Error, ErrorKind},
};
use mobc::{Manager, Pool};
use std::{sync::Arc, time::Duration};

use super::{
    Connection, ConnectionCreator, PoolManager, PoolManagerError, PoolManagerResult, PoolState, PooledConnection,
};

use crate::{
    ast,
    connector::{Queryable, Transaction, TransactionCapable},
};

pub struct MobcManager {
    pub(crate) connection_creator: ConnectionCreator,
}

#[async_trait]
impl PoolManager for Pool<MobcManager> {
    async fn acquire_with_timeout(&self, timeout: Duration) -> PoolManagerResult {
        match self.get_timeout(timeout).await {
            Ok(conn) => {
                let pooled = PooledConnection { inner: Box::new(conn) };
                Ok(pooled)
            }
            Err(e) => Err(e.into()),
        }
    }
    async fn acquire(&self) -> PoolManagerResult {
        match self.get().await {
            Ok(conn) => {
                let pooled = PooledConnection { inner: Box::new(conn) };
                Ok(pooled)
            }
            Err(e) => Err(e.into()),
        }
    }
    async fn state(&self) -> PoolState {
        let state = self.state().await;

        PoolState {
            max_open: state.max_open,
            in_use: state.in_use,
        }
    }
}

use std::ops::Deref;

// impl Deref<&dyn Queryable> {
//     type Target = Connection;
//     fn deref(&self) -> &Self::Target {
//         &self.conn.as_ref().unwrap().raw.as_ref().unwrap()
//     }
// }

#[async_trait]
impl Queryable for mobc::Connection<MobcManager> {
    async fn query(&self, q: ast::Query<'_>) -> crate::Result<connector::ResultSet> {
        self.deref().query(q).await
    }

    async fn execute(&self, q: ast::Query<'_>) -> crate::Result<u64> {
        self.deref().execute(q).await
    }

    async fn query_raw(&self, sql: &str, params: &[ast::Value<'_>]) -> crate::Result<connector::ResultSet> {
        self.deref().query_raw(sql, params).await
    }

    async fn execute_raw(&self, sql: &str, params: &[ast::Value<'_>]) -> crate::Result<u64> {
        self.deref().execute_raw(sql, params).await
    }

    async fn raw_cmd(&self, cmd: &str) -> crate::Result<()> {
        self.deref().raw_cmd(cmd).await
    }

    async fn version(&self) -> crate::Result<Option<String>> {
        self.deref().version().await
    }

    async fn server_reset_query(&self, tx: &Transaction<'_>) -> crate::Result<()> {
        self.deref().server_reset_query(tx).await
    }

    fn begin_statement(&self) -> &'static str {
        self.deref().begin_statement()
    }

    fn is_healthy(&self) -> bool {
        self.deref().is_healthy()
    }
}

#[async_trait]
impl Manager for MobcManager {
    type Connection = Connection;
    type Error = Error;

    async fn connect(&self) -> crate::Result<Self::Connection> {
        self.connection_creator.connect().await
    }

    async fn check(&self, conn: Self::Connection) -> crate::Result<Self::Connection> {
        conn.raw_cmd("SELECT 1").await?;
        Ok(conn)
    }

    fn validate(&self, conn: &mut Self::Connection) -> bool {
        conn.is_healthy()
    }
}

impl From<mobc::Error<Error>> for PoolManagerError {
    fn from(err: mobc::Error<Error>) -> PoolManagerError {
        match err {
            mobc::Error::Timeout => PoolManagerError::Timeout,
            mobc::Error::Inner(e) => PoolManagerError::Other(e),
            e @ mobc::Error::BadConn => {
                todo!("FIX ME")
                // PoolManagerError::BadConn(e)
                // let error = Error::builder(ErrorKind::ConnectionError(Box::new(e))).build();
                // return Err(error);
            }
        }
    }
}
