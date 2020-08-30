use crate::error::*;
use std::{collections::HashMap, fmt::Write, time::Duration};
use url::Url;

#[derive(Debug, Clone)]
pub struct MssqlUrl {
    connection_string: String,
    query_params: MssqlQueryParams,
}

#[derive(Debug, Clone)]
pub(crate) struct MssqlQueryParams {
    encrypt: bool,
    port: Option<u16>,
    host: Option<String>,
    user: Option<String>,
    password: Option<String>,
    database: String,
    trust_server_certificate: bool,
    connection_limit: Option<usize>,
    socket_timeout: Option<Duration>,
    connect_timeout: Option<Duration>,
}

impl MssqlUrl {
    pub fn new(jdbc_connection_string: &str) -> crate::Result<Self> {
        let query_params = Self::parse_query_params(jdbc_connection_string)?;
        let connection_string = Self::create_ado_net_string(&query_params)?;

        Ok(Self {
            connection_string,
            query_params,
        })
    }

    fn parse_query_params(jdbc_connection_string: &str) -> crate::Result<MssqlQueryParams> {
        let mut parts = jdbc_connection_string.split(';');

        match parts.next() {
            Some(host_part) => {
                let url = Url::parse(host_part)?;

                let params: crate::Result<HashMap<String, String>> = parts
                    .filter(|kv| kv != &"")
                    .map(|kv| kv.split("="))
                    .map(|mut split| {
                        let key = split
                            .next()
                            .ok_or_else(|| {
                                let kind = ErrorKind::conversion("Malformed connection string key");
                                Error::builder(kind).build()
                            })?
                            .trim();

                        let value = split.next().ok_or_else(|| {
                            let kind = ErrorKind::conversion("Malformed connection string value");
                            Error::builder(kind).build()
                        })?;

                        Ok((key.trim().to_lowercase(), value.trim().to_string()))
                    })
                    .collect();

                let mut params = params?;

                let host = url.host().map(|s| s.to_string());
                let port = url.port();
                let user = params.remove("user");
                let password = params.remove("password");
                let database = params.remove("database").unwrap_or_else(|| String::from("master"));
                let connection_limit = params.remove("connectionlimit").and_then(|param| param.parse().ok());

                let connect_timeout = params
                    .remove("logintimeout")
                    .or_else(|| params.remove("connecttimeout"))
                    .or_else(|| params.remove("connectiontimeout"))
                    .and_then(|param| param.parse::<u64>().ok())
                    .map(|secs| Duration::new(secs, 0));

                let socket_timeout = params
                    .remove("sockettimeout")
                    .and_then(|param| param.parse::<u64>().ok())
                    .map(|secs| Duration::new(secs, 0));

                let encrypt = params
                    .remove("encrypt")
                    .and_then(|param| param.parse().ok())
                    .unwrap_or(false);

                let trust_server_certificate = params
                    .remove("trustservercertificate")
                    .and_then(|param| param.parse().ok())
                    .unwrap_or(false);

                Ok(MssqlQueryParams {
                    encrypt,
                    port,
                    host,
                    user,
                    password,
                    database,
                    trust_server_certificate,
                    connection_limit,
                    socket_timeout,
                    connect_timeout,
                })
            }
            _ => {
                let kind = ErrorKind::conversion("Malformed connection string");
                Err(Error::builder(kind).build())
            }
        }
    }

    fn create_ado_net_string(params: &MssqlQueryParams) -> crate::Result<String> {
        let mut buf = String::new();

        write!(&mut buf, "Server=tcp:{},{}", params.host(), params.port())?;
        write!(&mut buf, ";Encrypt={}", params.encrypt())?;
        write!(&mut buf, ";Intial Catalog={}", params.database())?;

        write!(
            &mut buf,
            ";TrustServerCertificate={}",
            params.trust_server_certificate()
        )?;

        if let Some(user) = params.user() {
            write!(&mut buf, ";User ID={}", user)?;
        };

        if let Some(password) = params.password() {
            write!(&mut buf, ";Password={}", password)?;
        };

        Ok(buf)
    }

    pub fn connection_string(&self) -> &str {
        &self.connection_string
    }

    pub fn connection_limit(&self) -> Option<usize> {
        self.query_params.connection_limit()
    }

    pub fn socket_timeout(&self) -> Option<Duration> {
        self.query_params.socket_timeout()
    }

    pub fn connect_timeout(&self) -> Option<Duration> {
        self.query_params.connect_timeout()
    }

    pub fn dbname(&self) -> &str {
        self.query_params.database()
    }

    pub fn host(&self) -> &str {
        self.query_params.host()
    }

    pub fn username(&self) -> Option<&str> {
        self.query_params.user()
    }

    pub fn port(&self) -> u16 {
        self.query_params.port()
    }
}

impl MssqlQueryParams {
    pub fn encrypt(&self) -> bool {
        self.encrypt
    }

    pub fn port(&self) -> u16 {
        self.port.unwrap_or(1433)
    }

    pub fn host(&self) -> &str {
        self.host.as_ref().map(|s| s.as_str()).unwrap_or("localhost")
    }

    pub fn user(&self) -> Option<&str> {
        self.user.as_ref().map(|s| s.as_str())
    }

    pub fn password(&self) -> Option<&str> {
        self.password.as_ref().map(|s| s.as_str())
    }

    pub fn database(&self) -> &str {
        &self.database
    }

    pub fn trust_server_certificate(&self) -> bool {
        self.trust_server_certificate
    }

    pub fn socket_timeout(&self) -> Option<Duration> {
        self.socket_timeout
    }

    pub fn connect_timeout(&self) -> Option<Duration> {
        self.socket_timeout
    }

    pub fn connection_limit(&self) -> Option<usize> {
        self.connection_limit
    }
}
