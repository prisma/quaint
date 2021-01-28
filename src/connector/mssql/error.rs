use crate::error::{DatabaseConstraint, Error, ErrorKind};

impl From<tiberius::error::Error> for Error {
    fn from(e: tiberius::error::Error) -> Error {
        match e {
            tiberius::error::Error::Tls(message) => {
                let message = format!(
                    "The TLS settings didn't allow the connection to be established. Please review your connection string. (error: {})",
                    message
                );

                Error::builder(ErrorKind::TlsError { message }).build()
            }
            tiberius::error::Error::Server(e) if e.code() == 18456 => {
                let user = e.message().split('\'').nth(1).into();
                let kind = ErrorKind::AuthenticationFailed { user };

                let mut builder = Error::builder(kind);

                builder.set_original_code(format!("{}", e.code()));
                builder.set_original_message(e.message().to_string());

                builder.build()
            }
            tiberius::error::Error::Server(e) if e.code() == 4060 => {
                let db_name = e.message().split('"').nth(1).into();
                let kind = ErrorKind::DatabaseDoesNotExist { db_name };
                let mut builder = Error::builder(kind);

                builder.set_original_code(format!("{}", e.code()));
                builder.set_original_message(e.message().to_string());

                builder.build()
            }
            tiberius::error::Error::Server(e) if e.code() == 515 => {
                let constraint = e
                    .message()
                    .split_whitespace()
                    .nth(7)
                    .and_then(|s| s.split('\'').nth(1))
                    .map(|column| vec![column.to_string()])
                    .map(DatabaseConstraint::Fields)
                    .unwrap_or(DatabaseConstraint::CannotParse);

                let kind = ErrorKind::NullConstraintViolation { constraint };
                let mut builder = Error::builder(kind);

                builder.set_original_code(format!("{}", e.code()));
                builder.set_original_message(e.message().to_string());

                builder.build()
            }
            tiberius::error::Error::Server(e) if e.code() == 1801 => {
                let db_name = e.message().split('\'').nth(1).into();
                let kind = ErrorKind::DatabaseAlreadyExists { db_name };

                let mut builder = Error::builder(kind);

                builder.set_original_code(format!("{}", e.code()));
                builder.set_original_message(e.message().to_string());

                builder.build()
            }
            tiberius::error::Error::Server(e) if e.code() == 2627 => {
                let constraint = e
                    .message()
                    .split(". ")
                    .nth(1)
                    .and_then(|s| s.split(' ').last())
                    .and_then(|s| s.split('\'').nth(1))
                    .map(ToString::to_string)
                    .map(DatabaseConstraint::Index)
                    .unwrap_or(DatabaseConstraint::CannotParse);

                let kind = ErrorKind::UniqueConstraintViolation { constraint };
                let mut builder = Error::builder(kind);

                builder.set_original_code(format!("{}", e.code()));
                builder.set_original_message(e.message().to_string());

                builder.build()
            }
            tiberius::error::Error::Server(e) if e.code() == 547 => {
                let constraint = e
                    .message()
                    .split('.')
                    .next()
                    .and_then(|s| s.split_whitespace().last())
                    .and_then(|s| s.split('\"').nth(1))
                    .map(ToString::to_string)
                    .map(DatabaseConstraint::Index)
                    .unwrap_or(DatabaseConstraint::CannotParse);

                let kind = ErrorKind::ForeignKeyConstraintViolation { constraint };
                let mut builder = Error::builder(kind);

                builder.set_original_code(format!("{}", e.code()));
                builder.set_original_message(e.message().to_string());

                builder.build()
            }
            tiberius::error::Error::Server(e) if e.code() == 1505 => {
                let constraint = e
                    .message()
                    .split('\'')
                    .nth(3)
                    .map(ToString::to_string)
                    .map(DatabaseConstraint::Index)
                    .unwrap_or(DatabaseConstraint::CannotParse);

                let kind = ErrorKind::UniqueConstraintViolation { constraint };
                let mut builder = Error::builder(kind);

                builder.set_original_code(format!("{}", e.code()));
                builder.set_original_message(e.message().to_string());

                builder.build()
            }
            tiberius::error::Error::Server(e) if e.code() == 2601 => {
                let constraint = e
                    .message()
                    .split_whitespace()
                    .nth(11)
                    .and_then(|s| s.split('\'').nth(1))
                    .map(ToString::to_string)
                    .map(DatabaseConstraint::Index)
                    .unwrap_or(DatabaseConstraint::CannotParse);

                let kind = ErrorKind::UniqueConstraintViolation { constraint };
                let mut builder = Error::builder(kind);

                builder.set_original_code(format!("{}", e.code()));
                builder.set_original_message(e.message().to_string());

                builder.build()
            }
            tiberius::error::Error::Server(e) if e.code() == 2714 => {
                let db_name = e.message().split('\'').nth(1).into();
                let kind = ErrorKind::DatabaseAlreadyExists { db_name };

                let mut builder = Error::builder(kind);

                builder.set_original_code(format!("{}", e.code()));
                builder.set_original_message(e.message().to_string());

                builder.build()
            }
            tiberius::error::Error::Server(e) if e.code() == 2628 => {
                let column = e.message().split('\'').nth(3).into();
                let kind = ErrorKind::LengthMismatch { column };

                let mut builder = Error::builder(kind);

                builder.set_original_code(format!("{}", e.code()));
                builder.set_original_message(e.message().to_string());

                builder.build()
            }
            tiberius::error::Error::Server(e) if e.code() == 208 => {
                let table = e
                    .message()
                    .split_whitespace()
                    .nth(3)
                    .and_then(|s| s.split('\'').nth(1))
                    .into();

                let kind = ErrorKind::TableDoesNotExist { table };
                let mut builder = Error::builder(kind);

                builder.set_original_code(format!("{}", e.code()));
                builder.set_original_message(e.message().to_string());

                builder.build()
            }
            tiberius::error::Error::Server(e) if e.code() == 207 => {
                let column = e
                    .message()
                    .split_whitespace()
                    .nth(3)
                    .and_then(|s| s.split('\'').nth(1))
                    .into();

                let kind = ErrorKind::ColumnNotFound { column };
                let mut builder = Error::builder(kind);

                builder.set_original_code(format!("{}", e.code()));
                builder.set_original_message(e.message().to_string());

                builder.build()
            }
            tiberius::error::Error::Server(e) => {
                let kind = ErrorKind::QueryError(e.clone().into());

                let mut builder = Error::builder(kind);
                builder.set_original_code(format!("{}", e.code()));
                builder.set_original_message(e.message().to_string());

                builder.build()
            }
            e => Error::builder(ErrorKind::QueryError(e.into())).build(),
        }
    }
}
