use crate::error::{Error, ErrorKind};
use std::{convert::TryFrom, path::Path, time::Duration};

const DEFAULT_SCHEMA_NAME: &str = "quaint";

pub struct SqliteParams {
    pub connection_limit: Option<usize>,
    /// This is not a `PathBuf` because we need to `ATTACH` the database to the path, and this can
    /// only be done with UTF-8 paths.
    pub file_path: String,
    pub db_name: String,
    pub socket_timeout: Option<Duration>,
    pub statement_cache_size: usize,
}

type ConnectionParams = (Vec<(String, String)>, Vec<(String, String)>);

impl TryFrom<&str> for SqliteParams {
    type Error = Error;

    fn try_from(path: &str) -> crate::Result<Self> {
        let path = if path.starts_with("file:") {
            path.trim_start_matches("file:")
        } else {
            path.trim_start_matches("sqlite:")
        };

        let path_parts: Vec<&str> = path.split('?').collect();
        let path_str = path_parts[0];
        let path = Path::new(path_str);

        if path.is_dir() {
            Err(Error::builder(ErrorKind::DatabaseUrlIsInvalid(path.to_str().unwrap().to_string())).build())
        } else {
            let official = vec![];
            let mut connection_limit = None;
            let mut db_name = None;
            let mut socket_timeout = None;
            let mut statement_cache_size = 500;

            if path_parts.len() > 1 {
                let (_, unsupported): ConnectionParams = path_parts
                    .last()
                    .unwrap()
                    .split('&')
                    .map(|kv| {
                        let splitted: Vec<&str> = kv.split('=').collect();
                        (String::from(splitted[0]), String::from(splitted[1]))
                    })
                    .collect::<Vec<(String, String)>>()
                    .into_iter()
                    .partition(|(k, _)| official.contains(&k.as_str()));

                for (k, v) in unsupported.into_iter() {
                    match k.as_ref() {
                        "connection_limit" => {
                            let as_int: usize = v
                                .parse()
                                .map_err(|_| Error::builder(ErrorKind::InvalidConnectionArguments).build())?;

                            connection_limit = Some(as_int);
                        }
                        "db_name" => {
                            db_name = Some(v.to_string());
                        }
                        "socket_timeout" => {
                            let as_int = v
                                .parse()
                                .map_err(|_| Error::builder(ErrorKind::InvalidConnectionArguments).build())?;

                            socket_timeout = Some(Duration::from_secs(as_int));
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
                            tracing::trace!(message = "Discarding connection string param", param = k.as_str());
                        }
                    };
                }
            }

            Ok(Self {
                connection_limit,
                file_path: path_str.to_owned(),
                db_name: db_name.unwrap_or_else(|| DEFAULT_SCHEMA_NAME.to_owned()),
                socket_timeout,
                statement_cache_size,
            })
        }
    }
}
