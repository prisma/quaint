mod config;
mod conversion;
mod error;

use crate::{
    ast::{Insert, Query, Value},
    connector::{metrics, queryable::*, timeout::timeout, ResultSet, Transaction},
    visitor::{self, Visitor},
};
use async_trait::async_trait;
pub use config::*;
use futures::lock::Mutex;
use std::{convert::TryFrom, time::Duration};
use tiberius::*;

#[async_trait]
impl TransactionCapable for Mssql {
    async fn start_transaction(&self) -> crate::Result<Transaction<'_>> {
        Transaction::new(self, "BEGIN TRAN").await
    }
}

/// A connector interface for the PostgreSQL database.
#[derive(Debug)]
pub struct Mssql {
    #[cfg(feature = "runtime-tokio")]
    client: Mutex<Client<tokio_util::compat::Compat<tokio::net::TcpStream>>>,
    #[cfg(feature = "runtime-async-std")]
    client: Mutex<Client<async_std::net::TcpStream>>,

    url: MssqlUrl,
    socket_timeout: Option<Duration>,
}

impl Mssql {
    #[cfg(feature = "runtime-tokio")]
    pub async fn new(url: MssqlUrl) -> crate::Result<Self> {
        use tokio::net::TcpStream;
        use tokio_util::compat::Tokio02AsyncWriteCompatExt;

        let socket_timeout = url.socket_timeout();
        let config = Config::from_ado_string(&url.connection_string())?;

        let tcp = TcpStream::connect_named(&config).await?;
        let client = Client::connect(config, tcp.compat_write()).await?;

        Ok(Self {
            client: Mutex::new(client),
            url,
            socket_timeout,
        })
    }

    #[cfg(feature = "runtime-async-std")]
    pub async fn new(url: MssqlUrl) -> crate::Result<Self> {
        let socket_timeout = url.socket_timeout();
        let config = Config::from_ado_string(&url.connection_string())?;

        let tcp = async_std::net::TcpStream::connect_named(&config).await?;
        let client = Client::connect(config, tcp).await?;

        Ok(Self {
            client: Mutex::new(client),
            url,
            socket_timeout,
        })
    }
}

#[async_trait]
impl Queryable for Mssql {
    async fn query(&self, q: Query<'_>) -> crate::Result<ResultSet> {
        let (sql, params) = visitor::Mssql::build(q)?;
        self.query_raw(&sql, params).await
    }

    async fn execute(&self, q: Query<'_>) -> crate::Result<u64> {
        let (sql, params) = visitor::Mssql::build(q)?;
        self.execute_raw(&sql, params).await
    }

    async fn query_raw(&self, sql: &str, params: Vec<Value<'_>>) -> crate::Result<ResultSet> {
        metrics::query_new("mssql.query_raw", sql, params, move |params| async move {
            let mut client = self.client.lock().await;
            let params = conversion::conv_params(&params)?;
            let query = client.query(sql, params.as_slice());

            let results = timeout(self.socket_timeout, query).await?;

            let columns = results
                .columns()
                .unwrap_or(&[])
                .iter()
                .map(|c| c.name().to_string())
                .collect();

            let rows = results.into_first_result().await?;

            let mut result = ResultSet::new(columns, Vec::new());

            for row in rows {
                let mut values: Vec<Value<'_>> = Vec::with_capacity(row.len());

                for val in row.into_iter() {
                    values.push(Value::try_from(val)?);
                }

                result.rows.push(values);
            }

            Ok(result)
        })
        .await
    }

    async fn execute_raw(&self, sql: &str, params: Vec<Value<'_>>) -> crate::Result<u64> {
        metrics::query_new("mssql.execute_raw", sql, params, move |params| async move {
            let mut client = self.client.lock().await;
            let params = conversion::conv_params(&params)?;
            let query = client.execute(sql, params.as_slice());

            let changes = timeout(self.socket_timeout, query).await?.total();

            Ok(changes)
        })
        .await
    }

    async fn insert(&self, q: Insert<'_>) -> crate::Result<ResultSet> {
        self.query(q.into()).await
    }

    async fn raw_cmd(&self, cmd: &str) -> crate::Result<()> {
        metrics::query_new("mssql.raw_cmd", cmd, vec![], move |_| async move {
            let mut client = self.client.lock().await;
            timeout(self.socket_timeout, client.simple_query(cmd))
                .await?
                .into_results()
                .await?;

            Ok(())
        })
        .await
    }

    async fn version(&self) -> crate::Result<Option<String>> {
        let query = r#"SELECT @@VERSION AS version"#;
        let rows = self.query_raw(query, vec![]).await?;

        let version_string = rows
            .get(0)
            .and_then(|row| row.get("version").and_then(|version| version.to_string()));

        Ok(version_string)
    }

    fn begin_statement(&self) -> &'static str {
        "BEGIN TRAN"
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::test_api::mssql::CONN_STR;
    use crate::{error::*, single::Quaint};

    #[tokio::test]
    async fn should_map_wrong_credentials_error() {
        let url = CONN_STR.replace("user=SA", "user=WRONG");

        let res = Quaint::new(url.as_str()).await;
        assert!(res.is_err());

        let err = res.unwrap_err();
        assert!(matches!(err.kind(), ErrorKind::AuthenticationFailed { user } if user == "WRONG"));
    }
}
