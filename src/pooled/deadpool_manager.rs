use std::ops::Deref;
use std::time::Duration;

use super::{
    Connection, ConnectionCreator, PoolManager, PoolManagerError, PoolManagerResult, PoolState, PooledConnection,
};
use crate::{
    ast,
    connector::{self, Queryable, Transaction},
    error::Error,
};
use async_trait::async_trait;
use deadpool::managed::{self, Manager, Pool, PoolError, RecycleError, RecycleResult, Timeouts};

pub struct DeadpoolManager {
    pub(crate) connection_creator: ConnectionCreator,
}

#[async_trait]
impl PoolManager for Pool<DeadpoolManager> {
    async fn acquire_with_timeout(&self, timeout: Duration) -> PoolManagerResult {
        let timeouts = Timeouts {
            wait: Some(timeout),
            create: None,
            recycle: None,
        };
        match self.timeout_get(&timeouts).await {
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

#[async_trait]
impl Queryable for managed::Object<DeadpoolManager> {
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
impl Manager for DeadpoolManager {
    type Type = Connection;
    type Error = Error;

    async fn create(&self) -> crate::Result<Self::Type> {
        self.connection_creator.connect().await
    }

    async fn recycle(&self, conn: &mut Connection) -> RecycleResult<Error> {
        if !conn.is_healthy() {
            //log::info!(target: "deadpool.postgres", "Connection could not be recycled: {}", e);
            return Err(RecycleError::StaticMessage("Connection not healthy"));
        }

        Ok(())

        // match self.check(conn).await {
        //     Ok(_) => Ok(()),
        //     Err(_) => Err(RecycleError::StaticMessage("Connection check failed")),
        // }
    }
}

impl From<PoolError<Error>> for PoolManagerError {
    fn from(err: PoolError<Error>) -> PoolManagerError {
        match err {
            PoolError::Timeout(_) => PoolManagerError::Timeout,
            PoolError::Backend(e) => PoolManagerError::Other(e),
            err => panic!("Unknown error {:?}", err),
        }
    }
}
