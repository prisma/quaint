use percent_encoding::percent_decode;
use sqlx::mysql::{MySqlConnectOptions, MySqlSslMode};
use std::{
    borrow::Cow,
    path::{Path, PathBuf},
    time::Duration,
};
use url::Url;

use crate::error::{Error, ErrorKind};

/// Wraps a connection url and exposes the parsing logic used by quaint, including default values.
#[derive(Debug, Clone)]
pub struct MysqlUrl {
    url: Url,
    query_params: MysqlUrlQueryParams,
}

impl MysqlUrl {
    /// Parse `Url` to `MysqlUrl`. Returns error for mistyped connection
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

    /// The percent-decoded database password.
    pub fn password(&self) -> Option<Cow<str>> {
        match self
            .url
            .password()
            .and_then(|pw| percent_decode(pw.as_bytes()).decode_utf8().ok())
        {
            Some(password) => Some(password),
            None => self.url.password().map(|s| s.into()),
        }
    }

    /// Name of the database connected. Defaults to `mysql`.
    pub fn dbname(&self) -> &str {
        match self.url.path_segments() {
            Some(mut segments) => segments.next().unwrap_or("mysql"),
            None => "mysql",
        }
    }

    /// The database host. If `socket` and `host` are not set, defaults to `localhost`.
    pub fn host(&self) -> &str {
        self.url.host_str().unwrap_or("localhost")
    }

    /// If set, connected to the database through a Unix socket.
    pub fn socket(&self) -> &Option<String> {
        &self.query_params.socket
    }

    /// The database port, defaults to `3306`.
    pub fn port(&self) -> u16 {
        self.url.port().unwrap_or(3306)
    }

    /// Timeout for reading from the socket.
    pub fn socket_timeout(&self) -> Option<Duration> {
        self.query_params.socket_timeout
    }

    /// Timeout for connecting to the database.
    pub fn connect_timeout(&self) -> Option<Duration> {
        self.query_params.connect_timeout
    }

    fn parse_query_params(url: &Url) -> Result<MysqlUrlQueryParams, Error> {
        let mut connection_limit = None;
        let mut ssl_mode = MySqlSslMode::default();
        let mut root_cert_path = None;
        let mut socket = None;
        let mut socket_timeout = None;
        let mut connect_timeout = None;
        let mut statement_cache_size = 500;

        for (k, v) in url.query_pairs() {
            match k.as_ref() {
                "connection_limit" => {
                    let as_int: usize = v
                        .parse()
                        .map_err(|_| Error::builder(ErrorKind::InvalidConnectionArguments).build())?;

                    connection_limit = Some(as_int);
                }
                "sslmode" => {
                    match v.as_ref() {
                        "disabled" => ssl_mode = MySqlSslMode::Disabled,
                        "preferred" => ssl_mode = MySqlSslMode::Preferred,
                        "required" => ssl_mode = MySqlSslMode::Required,
                        "verify_ca" => ssl_mode = MySqlSslMode::VerifyCa,
                        "verify_identity" => ssl_mode = MySqlSslMode::VerifyIdentity,
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
                "socket" => {
                    socket = Some(v.replace("(", "").replace(")", ""));
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
                "statement_cache_size" => {
                    statement_cache_size = v
                        .parse()
                        .map_err(|_| Error::builder(ErrorKind::InvalidConnectionArguments).build())?;
                }
                _ => {
                    #[cfg(not(feature = "tracing-log"))]
                    trace!("Discarding connection string param: {}", k);
                    #[cfg(feature = "tracing-log")]
                    tracing::trace!(message = "Discarding connection string param", param = &*k);
                }
            };
        }

        Ok(MysqlUrlQueryParams {
            ssl_mode,
            root_cert_path,
            connection_limit,
            socket,
            connect_timeout,
            socket_timeout,
            statement_cache_size,
        })
    }

    #[cfg(feature = "pooled")]
    pub(crate) fn connection_limit(&self) -> Option<usize> {
        self.query_params.connection_limit
    }

    pub(crate) fn statement_cache_size(&self) -> usize {
        self.query_params.statement_cache_size
    }

    pub(crate) fn to_opts_builder(&self) -> MySqlConnectOptions {
        let mut config = MySqlConnectOptions::new()
            .username(&*self.username())
            .database(self.dbname());

        if let Some(password) = self.password() {
            config = config.password(&*password);
        }

        match self.socket() {
            Some(ref socket) => {
                config = config.socket(socket);
            }
            None => {
                config = config.host(self.host());
                config = config.port(self.port());
            }
        }

        config = config.statement_cache_capacity(self.statement_cache_size());
        config = config.ssl_mode(self.query_params.ssl_mode);

        if let Some(ref path) = self.query_params.root_cert_path {
            config = config.ssl_ca(path);
        }

        config
    }
}

#[derive(Debug, Clone)]
pub(crate) struct MysqlUrlQueryParams {
    ssl_mode: MySqlSslMode,
    root_cert_path: Option<PathBuf>,
    connection_limit: Option<usize>,
    socket: Option<String>,
    socket_timeout: Option<Duration>,
    connect_timeout: Option<Duration>,
    statement_cache_size: usize,
}
