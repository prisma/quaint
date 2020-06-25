mod config;
mod conversion;
mod error;

use crate::{
    ast::{Insert, Query, Value},
    connector::{bind::Bind, metrics, queryable::*, timeout::timeout, ResultSet, Transaction},
    visitor::{self, Visitor},
};
use async_trait::async_trait;
pub use config::*;
use either::Either;
use futures::lock::Mutex;
use sqlx::{Column as _, Connection, Done, Executor, PgConnection, Statement};
use std::time::Duration;

/// A connector interface for the PostgreSQL database.
#[derive(Debug)]
pub struct PostgreSql {
    connection: Mutex<PgConnection>,
    pg_bouncer: bool,
    socket_timeout: Option<Duration>,
}

impl PostgreSql {
    /// Create a new connection to the database.
    pub async fn new(url: PostgresUrl) -> crate::Result<Self> {
        let config = url.to_config();
        let mut conn = PgConnection::connect_with(&config).await?;

        let schema = url.schema();

        // SET NAMES sets the client text encoding. It needs to be explicitly set for automatic
        // conversion to and from UTF-8 to happen server-side.
        //
        // Relevant docs: https://www.postgresql.org/docs/current/multibyte.html
        let session_variables = format!(
            r##"
            SET search_path = "{schema}";
            SET NAMES 'UTF8';
            "##,
            schema = schema
        );

        conn.execute(session_variables.as_str()).await?;

        Ok(Self {
            connection: Mutex::new(conn),
            socket_timeout: url.socket_timeout(),
            pg_bouncer: url.pg_bouncer(),
        })
    }
}

impl TransactionCapable for PostgreSql {}

#[async_trait]
impl Queryable for PostgreSql {
    async fn query(&self, q: Query<'_>) -> crate::Result<ResultSet> {
        let (sql, params) = visitor::Postgres::build(q)?;
        self.query_raw(sql.as_str(), params).await
    }

    async fn execute(&self, q: Query<'_>) -> crate::Result<u64> {
        let (sql, params) = visitor::Postgres::build(q)?;
        self.execute_raw(sql.as_str(), params).await
    }

    async fn insert(&self, q: Insert<'_>) -> crate::Result<ResultSet> {
        self.query(q.into()).await
    }

    async fn query_raw(&self, sql: &str, params: Vec<Value<'_>>) -> crate::Result<ResultSet> {
        metrics::query_new("postgres.query_raw", sql, params, |params| async move {
            let mut conn = self.connection.lock().await;
            let stmt = timeout(self.socket_timeout, conn.prepare(sql)).await?;
            let columns = stmt.columns().into_iter().map(|c| c.name().to_string()).collect();

            let mut query = stmt.query();

            match stmt.parameters() {
                Some(Either::Left(type_infos)) => {
                    let values = params.into_iter();
                    let infos = type_infos.into_iter().map(Some);

                    for (param, type_info) in values.zip(infos) {
                        query = query.bind_value(param, type_info)?;
                    }
                }
                _ => {
                    for param in params.into_iter() {
                        query = query.bind_value(param, None)?;
                    }
                }
            };

            let rows = timeout(
                self.socket_timeout,
                query.try_map(conversion::map_row).fetch_all(&mut *conn),
            )
            .await?;

            Ok(ResultSet::new(columns, rows))
        })
        .await
    }

    async fn execute_raw(&self, sql: &str, params: Vec<Value<'_>>) -> crate::Result<u64> {
        metrics::query_new("postgres.execute_raw", sql, params, |params| async move {
            let mut conn = self.connection.lock().await;
            let stmt = timeout(self.socket_timeout, conn.prepare(sql)).await?;

            let mut query = stmt.query();

            match stmt.parameters() {
                Some(Either::Left(type_infos)) => {
                    let values = params.into_iter();
                    let infos = type_infos.into_iter().map(Some);

                    for (param, type_info) in values.zip(infos) {
                        query = query.bind_value(param, type_info)?;
                    }
                }
                _ => {
                    for param in params.into_iter() {
                        query = query.bind_value(param, None)?;
                    }
                }
            };

            let done = query.execute(&mut *conn).await?;

            Ok(done.rows_affected())
        })
        .await
    }

    async fn raw_cmd(&self, cmd: &str) -> crate::Result<()> {
        metrics::query("postgres.raw_cmd", cmd, &[], move || async move {
            let mut conn = self.connection.lock().await;
            timeout(self.socket_timeout, conn.execute(cmd)).await?;
            Ok(())
        })
        .await
    }

    async fn version(&self) -> crate::Result<Option<String>> {
        let query = r#"SELECT version()"#;
        let rows = self.query_raw(query, vec![]).await?;

        let version_string = rows
            .get(0)
            .and_then(|row| row.get("version").and_then(|version| version.to_string()));

        Ok(version_string)
    }

    async fn server_reset_query(&self, tx: &Transaction<'_>) -> crate::Result<()> {
        if self.pg_bouncer {
            tx.raw_cmd("DEALLOCATE ALL").await
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_api::postgres::CONN_STR;
    use crate::{connector::Queryable, error::*, single::Quaint};
    use url::Url;

    #[test]
    fn should_parse_socket_url() {
        let url = PostgresUrl::new(Url::parse("postgresql:///dbname?host=/var/run/psql.sock").unwrap()).unwrap();
        assert_eq!("dbname", url.dbname());
        assert_eq!("/var/run/psql.sock", url.host());
    }

    #[test]
    fn should_parse_escaped_url() {
        let url = PostgresUrl::new(Url::parse("postgresql:///dbname?host=%2Fvar%2Frun%2Fpostgresql").unwrap()).unwrap();
        assert_eq!("dbname", url.dbname());
        assert_eq!("/var/run/postgresql", url.host());
    }

    #[test]
    fn should_allow_changing_of_cache_size() {
        let url =
            PostgresUrl::new(Url::parse("postgresql:///localhost:5432/foo?statement_cache_size=420").unwrap()).unwrap();
        assert_eq!(420, url.statement_cache_size());
    }

    #[test]
    fn should_have_default_cache_size() {
        let url = PostgresUrl::new(Url::parse("postgresql:///localhost:5432/foo").unwrap()).unwrap();
        assert_eq!(500, url.statement_cache_size());
    }

    #[test]
    fn should_not_enable_caching_with_pgbouncer() {
        let url = PostgresUrl::new(Url::parse("postgresql:///localhost:5432/foo?pgbouncer=true").unwrap()).unwrap();
        assert_eq!(0, url.statement_cache_size());
    }

    #[test]
    fn should_parse_default_host() {
        let url = PostgresUrl::new(Url::parse("postgresql:///dbname").unwrap()).unwrap();
        assert_eq!("dbname", url.dbname());
        assert_eq!("localhost", url.host());
    }

    #[tokio::test]
    async fn test_custom_search_path() {
        let mut url = Url::parse(&CONN_STR).unwrap();
        url.query_pairs_mut().append_pair("schema", "musti-test");

        let client = Quaint::new(url.as_str()).await.unwrap();

        let result_set = client.query_raw("SHOW search_path", vec![]).await.unwrap();
        let row = result_set.first().unwrap();

        assert_eq!(Some("\"musti-test\""), row[0].as_str());
    }

    #[tokio::test]
    async fn should_map_nonexisting_database_error() {
        let mut url = Url::parse(&CONN_STR).unwrap();
        url.set_path("/this_does_not_exist");

        let res = Quaint::new(url.as_str()).await;

        assert!(res.is_err());

        match res {
            Ok(_) => unreachable!(),
            Err(e) => match e.kind() {
                ErrorKind::DatabaseDoesNotExist { db_name } => {
                    assert_eq!(Some("3D000"), e.original_code());
                    assert_eq!(
                        Some("database \"this_does_not_exist\" does not exist"),
                        e.original_message()
                    );
                    assert_eq!("this_does_not_exist", db_name.as_str())
                }
                kind => panic!("Expected `DatabaseDoesNotExist`, got {:?}", kind),
            },
        }
    }

    #[tokio::test]
    async fn should_map_wrong_credentials_error() {
        let mut url = Url::parse(&CONN_STR).unwrap();
        url.set_username("WRONG").unwrap();

        let res = Quaint::new(url.as_str()).await;
        assert!(res.is_err());

        let err = res.unwrap_err();
        assert!(matches!(err.kind(), ErrorKind::AuthenticationFailed { user } if user == "WRONG"));
    }

    #[tokio::test]
    async fn should_map_tls_errors() {
        let mut url = Url::parse(&CONN_STR).expect("parsing url");
        url.set_query(Some("sslmode=require&sslaccept=strict"));

        let res = Quaint::new(url.as_str()).await;

        assert!(res.is_err());

        match res {
            Ok(_) => unreachable!(),
            Err(e) => match e.kind() {
                ErrorKind::TlsError { .. } => (),
                other => panic!("{:#?}", other),
            },
        }
    }
}
