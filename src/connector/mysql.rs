#![allow(dead_code)]

mod config;
mod conversion;
mod error;

pub use config::*;

use async_trait::async_trait;
use futures::{lock::Mutex, TryStreamExt};
use sqlx::{Column, Connection, Done, Executor, MySqlConnection, Row};
use std::time::Duration;

use crate::{
    ast::{Insert, Query, Value},
    connector::{bind::Bind, metrics, queryable::*, timeout::timeout, ResultSet},
    error::Error,
    visitor::{self, Visitor},
};

/// A connector interface for the MySQL database.
#[derive(Debug)]
pub struct Mysql {
    pub(crate) connection: Mutex<MySqlConnection>,
    pub(crate) url: MysqlUrl,
    socket_timeout: Option<Duration>,
    connect_timeout: Option<Duration>,
}

impl Mysql {
    /// Create a new MySQL connection using `OptsBuilder` from the `mysql` crate.
    pub async fn new(url: MysqlUrl) -> crate::Result<Self> {
        let opts = url.to_opts_builder();
        let conn = MySqlConnection::connect_with(&opts).await?;

        Ok(Self {
            socket_timeout: url.socket_timeout(),
            connect_timeout: url.connect_timeout(),
            connection: Mutex::new(conn),
            url,
        })
    }
}

impl TransactionCapable for Mysql {}

#[async_trait]
impl Queryable for Mysql {
    async fn query(&self, q: Query<'_>) -> crate::Result<ResultSet> {
        let (sql, params) = visitor::Mysql::build(q)?;
        self.query_raw(&sql, params).await
    }

    async fn execute(&self, q: Query<'_>) -> crate::Result<u64> {
        let (sql, params) = visitor::Mysql::build(q)?;
        self.execute_raw(&sql, params).await
    }

    async fn query_raw(&self, sql: &str, params: Vec<Value<'_>>) -> crate::Result<ResultSet> {
        metrics::query_new("mysql.query_raw", sql, params, |params| async move {
            let mut query = sqlx::query(sql);

            for param in params.into_iter() {
                query = query.bind_value(param, None)?;
            }

            let mut conn = self.connection.lock().await;
            let mut columns = Vec::new();
            let mut rows = Vec::new();

            timeout(self.socket_timeout, async {
                let mut stream = query.fetch(&mut *conn);

                while let Some(row) = stream.try_next().await? {
                    if columns.is_empty() {
                        columns = row.columns().iter().map(|c| c.name().to_string()).collect();
                    }

                    rows.push(conversion::map_row(row)?);
                }

                Ok::<(), Error>(())
            })
            .await?;

            Ok(ResultSet::new(columns, rows))
        })
        .await
    }

    async fn execute_raw(&self, sql: &str, params: Vec<Value<'_>>) -> crate::Result<u64> {
        metrics::query_new("mysql.execute_raw", sql, params, |params| async move {
            let mut query = sqlx::query(sql);

            for param in params.into_iter() {
                query = query.bind_value(param, None)?;
            }

            let mut conn = self.connection.lock().await;
            let done = timeout(self.socket_timeout, query.execute(&mut *conn)).await?;

            Ok(done.rows_affected())
        })
        .await
    }

    async fn insert(&self, q: Insert<'_>) -> crate::Result<ResultSet> {
        let (sql, params) = visitor::Mysql::build(q)?;

        metrics::query_new("mysql.execute_raw", &sql, params, |params| async {
            let mut query = sqlx::query(&sql);

            for param in params.into_iter() {
                query = query.bind_value(param, None)?;
            }

            let mut conn = self.connection.lock().await;
            let done = timeout(self.socket_timeout, query.execute(&mut *conn)).await?;

            let mut result_set = ResultSet::default();
            result_set.set_last_insert_id(done.last_insert_id());

            Ok(result_set)
        })
        .await
    }

    async fn raw_cmd(&self, cmd: &str) -> crate::Result<()> {
        metrics::query_new("mysql.raw_cmd", cmd, Vec::new(), move |_| async move {
            let mut conn = self.connection.lock().await;
            timeout(self.socket_timeout, conn.execute(cmd)).await?;
            Ok(())
        })
        .await
    }

    async fn version(&self) -> crate::Result<Option<String>> {
        let query = r#"SELECT @@GLOBAL.version version"#;
        let rows = self.query_raw(query, vec![]).await?;

        let version_string = rows
            .get(0)
            .and_then(|row| row.get("version").and_then(|version| version.to_string()));

        Ok(version_string)
    }
}

#[cfg(test)]
mod tests {
    use super::MysqlUrl;
    use crate::tests::test_api::mysql::CONN_STR;
    use crate::{connector::Queryable, error::*, single::Quaint};
    use url::Url;

    #[test]
    fn should_parse_socket_url() {
        let url = MysqlUrl::new(Url::parse("mysql://root@localhost/dbname?socket=(/tmp/mysql.sock)").unwrap()).unwrap();
        assert_eq!("dbname", url.dbname());
        assert_eq!(&Some(String::from("/tmp/mysql.sock")), url.socket());
    }

    #[tokio::test]
    async fn should_map_nonexisting_database_error() {
        let mut url = Url::parse(&*CONN_STR).unwrap();
        url.set_username("root").unwrap();
        url.set_path("/this_does_not_exist");

        let url = url.as_str().to_string();
        let res = Quaint::new(&url).await;

        assert!(&res.is_err());

        let err = res.unwrap_err();

        match err.kind() {
            ErrorKind::DatabaseDoesNotExist { db_name } => {
                assert_eq!(Some("1049"), err.original_code());
                assert_eq!(Some("Unknown database \'this_does_not_exist\'"), err.original_message());
                assert_eq!("this_does_not_exist", db_name.as_str())
            }
            e => panic!("Expected `DatabaseDoesNotExist`, got {:?}", e),
        }
    }

    #[tokio::test]
    async fn should_map_wrong_credentials_error() {
        let mut url = Url::parse(&CONN_STR).unwrap();
        url.set_username("WRONG").unwrap();

        let conn = Quaint::new(url.as_str()).await.unwrap();
        let res = conn.query_raw("SELECT 1", vec![]).await;
        assert!(res.is_err());

        let err = res.unwrap_err();
        assert!(matches!(err.kind(), ErrorKind::AuthenticationFailed { user } if user == "WRONG"));
    }
}
