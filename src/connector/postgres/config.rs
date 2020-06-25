use crate::error::{Error, ErrorKind};
use percent_encoding::percent_decode;
use sqlx::postgres::{PgConnectOptions, PgSslMode};
use std::{
    borrow::{Borrow, Cow},
    path::{Path, PathBuf},
    time::Duration,
};
use url::Url;

pub(crate) const DEFAULT_SCHEMA: &str = "public";

/// Wraps a connection url and exposes the parsing logic used by quaint, including default values.
#[derive(Debug, Clone)]
pub struct PostgresUrl {
    url: Url,
    query_params: PostgresUrlQueryParams,
}

impl PostgresUrl {
    /// Parse `Url` to `PostgresUrl`. Returns error for mistyped connection
    /// parameters.
    pub fn new(url: Url) -> Result<Self, Error> {
        let query_params = Self::parse_query_params(&url)?;

        Ok(Self { url, query_params })
    }

    /// The bare `Url` to the database.
    pub fn url(&self) -> &Url {
        &self.url
    }

    /// The percent-decoded database username.
    pub fn username(&self) -> Cow<str> {
        match percent_decode(self.url.username().as_bytes()).decode_utf8() {
            Ok(username) => username,
            Err(_) => {
                #[cfg(not(feature = "tracing-log"))]
                warn!("Couldn't decode username to UTF-8, using the non-decoded version.");
                #[cfg(feature = "tracing-log")]
                tracing::warn!("Couldn't decode username to UTF-8, using the non-decoded version.");

                self.url.username().into()
            }
        }
    }

    /// The database host. Taken first from the `host` query parameter, then
    /// from the `host` part of the URL. For socket connections, the query
    /// parameter must be used.
    ///
    /// If none of them are set, defaults to `localhost`.
    pub fn host(&self) -> &str {
        match (self.query_params.host.as_ref(), self.url.host_str()) {
            (Some(host), _) => host.as_str(),
            (None, Some("")) => "localhost",
            (None, None) => "localhost",
            (None, Some(host)) => host,
        }
    }

    /// Name of the database connected. Defaults to `postgres`.
    pub fn dbname(&self) -> &str {
        match self.url.path_segments() {
            Some(mut segments) => segments.next().unwrap_or("postgres"),
            None => "postgres",
        }
    }

    /// The percent-decoded database password.
    pub fn password(&self) -> Cow<str> {
        match self
            .url
            .password()
            .and_then(|pw| percent_decode(pw.as_bytes()).decode_utf8().ok())
        {
            Some(password) => password,
            None => self.url.password().unwrap_or("").into(),
        }
    }

    /// The database port, defaults to `5432`.
    pub fn port(&self) -> u16 {
        self.url.port().unwrap_or(5432)
    }

    /// The database schema, defaults to `public`.
    pub fn schema(&self) -> &str {
        &self.query_params.schema
    }

    pub(crate) fn connect_timeout(&self) -> Option<Duration> {
        self.query_params.connect_timeout
    }

    pub(crate) fn socket_timeout(&self) -> Option<Duration> {
        self.query_params.socket_timeout
    }

    pub(crate) fn pg_bouncer(&self) -> bool {
        self.query_params.pg_bouncer
    }

    pub(crate) fn statement_cache_size(&self) -> usize {
        if self.query_params.pg_bouncer == true {
            0
        } else {
            self.query_params.statement_cache_size
        }
    }

    fn parse_query_params(url: &Url) -> Result<PostgresUrlQueryParams, Error> {
        let mut connection_limit = None;
        let mut schema = String::from(DEFAULT_SCHEMA);
        let mut ssl_mode = PgSslMode::Prefer;
        let mut root_cert_path = None;
        let mut host = None;
        let mut socket_timeout = None;
        let mut connect_timeout = None;
        let mut pg_bouncer = false;
        let mut statement_cache_size = 500;

        for (k, v) in url.query_pairs() {
            match k.as_ref() {
                "pgbouncer" => {
                    pg_bouncer = v
                        .parse()
                        .map_err(|_| Error::builder(ErrorKind::InvalidConnectionArguments).build())?;
                }
                "sslmode" => {
                    match v.as_ref() {
                        "disable" => ssl_mode = PgSslMode::Disable,
                        "allow" => ssl_mode = PgSslMode::Allow,
                        "prefer" => ssl_mode = PgSslMode::Prefer,
                        "require" => ssl_mode = PgSslMode::Require,
                        "verify_ca" => ssl_mode = PgSslMode::VerifyCa,
                        "verify_full" => ssl_mode = PgSslMode::VerifyFull,
                        _ => {
                            #[cfg(not(feature = "tracing-log"))]
                            debug!("Unsupported ssl mode {}, defaulting to 'prefer'", v);
                            #[cfg(feature = "tracing-log")]
                            tracing::debug!(message = "Unsupported SSL mode, defaulting to `prefer`", mode = &*v);
                        }
                    };
                }
                "sslcert" => {
                    root_cert_path = Some(Path::new(&*v).to_path_buf());
                }
                "statement_cache_size" => {
                    statement_cache_size = v
                        .parse()
                        .map_err(|_| Error::builder(ErrorKind::InvalidConnectionArguments).build())?;
                }
                "schema" => {
                    schema = v.to_string();
                }
                "connection_limit" => {
                    let as_int: usize = v
                        .parse()
                        .map_err(|_| Error::builder(ErrorKind::InvalidConnectionArguments).build())?;
                    connection_limit = Some(as_int);
                }
                "host" => {
                    host = Some(v.to_string());
                }
                "socket_timeout" => {
                    let as_int = v
                        .parse()
                        .map_err(|_| Error::builder(ErrorKind::InvalidConnectionArguments).build())?;
                    socket_timeout = Some(Duration::from_secs(as_int));
                }
                "connect_timeout" => {
                    let as_int = v
                        .parse()
                        .map_err(|_| Error::builder(ErrorKind::InvalidConnectionArguments).build())?;
                    connect_timeout = Some(Duration::from_secs(as_int));
                }
                _ => {
                    #[cfg(not(feature = "tracing-log"))]
                    trace!("Discarding connection string param: {}", k);
                    #[cfg(feature = "tracing-log")]
                    tracing::trace!(message = "Discarding connection string param", param = &*k);
                }
            };
        }

        Ok(PostgresUrlQueryParams {
            connection_limit,
            schema,
            ssl_mode,
            host,
            connect_timeout,
            socket_timeout,
            pg_bouncer,
            statement_cache_size,
            root_cert_path,
        })
    }

    #[cfg(feature = "pooled")]
    pub(crate) fn connection_limit(&self) -> Option<usize> {
        self.query_params.connection_limit
    }

    pub(crate) fn to_config(&self) -> PgConnectOptions {
        let mut opts = PgConnectOptions::new()
            .host(self.host())
            .port(self.port())
            .username(self.username().borrow())
            .password(self.password().borrow())
            .database(self.dbname())
            .statement_cache_capacity(self.statement_cache_size())
            .ssl_mode(self.query_params.ssl_mode);

        if let Some(ref path) = self.query_params.root_cert_path {
            opts = opts.ssl_root_cert(path);
        }

        opts
    }
}

#[derive(Debug, Clone)]
pub(crate) struct PostgresUrlQueryParams {
    ssl_mode: PgSslMode,
    root_cert_path: Option<PathBuf>,
    connection_limit: Option<usize>,
    schema: String,
    pg_bouncer: bool,
    host: Option<String>,
    socket_timeout: Option<Duration>,
    connect_timeout: Option<Duration>,
    statement_cache_size: usize,
}
