mod config;
mod conversion;
mod error;

use crate::{
    ast::{Insert, Query, Value},
    connector::{bind::Bind, metrics, queryable::*, timeout::timeout, ResultSet},
    error::Error,
    visitor::{self, Visitor},
};
use async_trait::async_trait;
pub use config::*;
use futures::{lock::Mutex, TryStreamExt};
use sqlx::{
    sqlite::{SqliteConnectOptions, SqliteRow},
    Column as _, Connection, Done, Executor, Row as _, SqliteConnection,
};
use std::{collections::HashSet, convert::TryFrom, time::Duration};

/// A connector interface for the SQLite database
pub struct Sqlite {
    pub(crate) connection: Mutex<SqliteConnection>,
    /// This is not a `PathBuf` because we need to `ATTACH` the database to the path, and this can
    /// only be done with UTF-8 paths.
    pub(crate) file_path: String,
    pub(crate) socket_timeout: Option<Duration>,
}

impl Sqlite {
    pub async fn new(file_path: &str) -> crate::Result<Sqlite> {
        let params = SqliteParams::try_from(file_path)?;

        let opts = SqliteConnectOptions::new()
            .statement_cache_capacity(params.statement_cache_size)
            .create_if_missing(true);

        let conn = SqliteConnection::connect_with(&opts).await?;

        let connection = Mutex::new(conn);
        let file_path = params.file_path;
        let socket_timeout = params.socket_timeout;

        Ok(Sqlite {
            connection,
            file_path,
            socket_timeout,
        })
    }

    pub async fn attach_database(&mut self, db_name: &str) -> crate::Result<()> {
        let mut conn = self.connection.lock().await;

        let databases: HashSet<String> = sqlx::query("PRAGMA database_list")
            .try_map(|row: SqliteRow| {
                let name: String = row.try_get(1)?;
                Ok(name)
            })
            .fetch_all(&mut *conn)
            .await?
            .into_iter()
            .collect();

        if !databases.contains(db_name) {
            sqlx::query("ATTACH DATABASE ? AS ?")
                .bind(self.file_path.as_str())
                .bind(db_name)
                .execute(&mut *conn)
                .await?;
        }

        sqlx::query("PRAGMA foreign_keys = ON").execute(&mut *conn).await?;

        Ok(())
    }
}

impl TransactionCapable for Sqlite {}

#[async_trait]
impl Queryable for Sqlite {
    async fn query(&self, q: Query<'_>) -> crate::Result<ResultSet> {
        let (sql, params) = visitor::Sqlite::build(q)?;
        self.query_raw(&sql, params).await
    }

    async fn execute(&self, q: Query<'_>) -> crate::Result<u64> {
        let (sql, params) = visitor::Sqlite::build(q)?;
        self.execute_raw(&sql, params).await
    }

    async fn insert(&self, q: Insert<'_>) -> crate::Result<ResultSet> {
        let (sql, params) = visitor::Sqlite::build(q)?;

        metrics::query_new("sqlite.execute_raw", &sql, params, |params| async {
            let mut query = sqlx::query(&sql);

            for param in params.into_iter() {
                query = query.bind_value(param, None)?;
            }

            let mut conn = self.connection.lock().await;
            let done = timeout(self.socket_timeout, query.execute(&mut *conn)).await?;

            let mut result_set = ResultSet::default();
            result_set.set_last_insert_id(done.last_insert_rowid() as u64);

            Ok(result_set)
        })
        .await
    }

    async fn query_raw(&self, sql: &str, params: Vec<Value<'_>>) -> crate::Result<ResultSet> {
        metrics::query_new("sqlite.query_raw", sql, params, move |params| async move {
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
        metrics::query_new("sqlite.execute_raw", sql, params, |params| async move {
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

    async fn raw_cmd(&self, cmd: &str) -> crate::Result<()> {
        metrics::query_new("sqlite.raw_cmd", cmd, Vec::new(), move |_| async move {
            let mut conn = self.connection.lock().await;
            timeout(self.socket_timeout, conn.execute(cmd)).await?;
            Ok(())
        })
        .await
    }

    async fn version(&self) -> crate::Result<Option<String>> {
        let query = r#"SELECT sqlite_version() version;"#;
        let rows = self.query_raw(query, vec![]).await?;

        let version_string = rows
            .get(0)
            .and_then(|row| row.get("version").and_then(|version| version.to_string()));

        Ok(version_string)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ast::*, connector::Queryable, error::ErrorKind};

    #[test]
    fn sqlite_params_from_str_should_resolve_path_correctly_with_file_scheme() {
        let path = "file:dev.db";
        let params = SqliteParams::try_from(path).unwrap();
        assert_eq!(params.file_path, "dev.db");
    }

    #[test]
    fn sqlite_params_from_str_should_resolve_path_correctly_with_sqlite_scheme() {
        let path = "sqlite:dev.db";
        let params = SqliteParams::try_from(path).unwrap();
        assert_eq!(params.file_path, "dev.db");
    }

    #[test]
    fn sqlite_params_from_str_should_resolve_path_correctly_with_no_scheme() {
        let path = "dev.db";
        let params = SqliteParams::try_from(path).unwrap();
        assert_eq!(params.file_path, "dev.db");
    }

    #[tokio::test(threaded_scheduler)]
    async fn unknown_table_should_give_a_good_error() {
        let conn = Sqlite::new("file:db/test.db").await.unwrap();
        let select = Select::from_table("not_there");

        let err = conn.select(select).await.unwrap_err();

        match err.kind() {
            ErrorKind::TableDoesNotExist { table } => {
                assert_eq!("not_there", table.as_str());
            }
            e => panic!("Expected error TableDoesNotExist, got {:?}", e),
        }
    }
}
