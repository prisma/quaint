use crate::error::{DatabaseConstraint, Error, ErrorKind};
use sqlx::{error::DatabaseError, sqlite::SqliteError};

impl From<SqliteError> for Error {
    fn from(e: SqliteError) -> Self {
        match e.code().map(|c| c.into_owned()) {
            Some(code) if code == "2067" => {
                let splitted: Vec<&str> = e.message().split(": ").collect();

                let field_names: Vec<String> = splitted[1]
                    .split(", ")
                    .map(|s| s.split(".").last().unwrap())
                    .map(|s| s.to_string())
                    .collect();

                let mut builder = Error::builder(ErrorKind::UniqueConstraintViolation {
                    constraint: DatabaseConstraint::Fields(field_names),
                });

                builder.set_original_code(code);
                builder.set_original_message(e.message());

                builder.build()
            }

            Some(code) if code == "1555" => {
                let splitted: Vec<&str> = e.message().split(": ").collect();

                let field_names: Vec<String> = splitted[1]
                    .split(", ")
                    .map(|s| s.split(".").last().unwrap())
                    .map(|s| s.to_string())
                    .collect();

                let mut builder = Error::builder(ErrorKind::UniqueConstraintViolation {
                    constraint: DatabaseConstraint::Fields(field_names),
                });

                builder.set_original_code(code);
                builder.set_original_message(e.message());

                builder.build()
            }

            Some(code) if code == "1299" => {
                let splitted: Vec<&str> = e.message().split(": ").collect();

                let field_names: Vec<String> = splitted[1]
                    .split(", ")
                    .map(|s| s.split(".").last().unwrap())
                    .map(|s| s.to_string())
                    .collect();

                let mut builder = Error::builder(ErrorKind::NullConstraintViolation {
                    constraint: DatabaseConstraint::Fields(field_names),
                });

                builder.set_original_code(code);
                builder.set_original_message(e.message());

                builder.build()
            }

            Some(code) if code == "787" => {
                let mut builder = Error::builder(ErrorKind::ForeignKeyConstraintViolation {
                    constraint: DatabaseConstraint::ForeignKey,
                });

                builder.set_original_code(code);
                builder.set_original_message(e.message());

                builder.build()
            }

            Some(code) if code == "261" || code == "517" => {
                let mut builder = Error::builder(ErrorKind::Timeout("SQLite database is busy".into()));
                builder.set_original_code(code);
                builder.set_original_message(e.message());

                builder.build()
            }

            Some(code) => {
                let message = e.message().to_string();

                if message.starts_with("no such table") {
                    let table = message.split(": ").last().unwrap().into();

                    let mut builder = Error::builder(ErrorKind::TableDoesNotExist { table });
                    builder.set_original_code(code);
                    builder.set_original_message(message);

                    builder.build()
                } else {
                    let mut builder = Error::builder(ErrorKind::QueryError(e.into()));
                    builder.set_original_code(code);
                    builder.set_original_message(message);

                    builder.build()
                }
            }

            None => Error::builder(ErrorKind::QueryError(e.into())).build(),
        }
    }
}
