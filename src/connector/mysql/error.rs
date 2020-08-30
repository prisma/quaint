use crate::error::{DatabaseConstraint, Error, ErrorKind};
use sqlx::mysql::MySqlDatabaseError;

impl From<MySqlDatabaseError> for Error {
    fn from(e: MySqlDatabaseError) -> Self {
        match e.number() {
            code if code == 1062 => {
                let splitted: Vec<&str> = e.message().split_whitespace().collect();
                let splitted: Vec<&str> = splitted.last().map(|s| s.split('\'').collect()).unwrap();

                let index = splitted[1].split(".").last().unwrap().to_string();

                let mut builder = Error::builder(ErrorKind::UniqueConstraintViolation {
                    constraint: DatabaseConstraint::Index(index),
                });

                builder.set_original_code(format!("{}", code));
                builder.set_original_message(e.message());

                builder.build()
            }

            code if code == 1451 || code == 1452 => {
                let splitted: Vec<&str> = e.message().split_whitespace().collect();
                let splitted: Vec<&str> = splitted[17].split('`').collect();

                let field = splitted[1].to_string();

                let mut builder = Error::builder(ErrorKind::ForeignKeyConstraintViolation {
                    constraint: DatabaseConstraint::Fields(vec![field]),
                });

                builder.set_original_code(format!("{}", code));
                builder.set_original_message(e.message());

                builder.build()
            }

            code if code == 1263 => {
                let splitted: Vec<&str> = e.message().split_whitespace().collect();
                let splitted: Vec<&str> = splitted.last().map(|s| s.split('\'').collect()).unwrap();

                let mut builder = Error::builder(ErrorKind::NullConstraintViolation {
                    constraint: DatabaseConstraint::Index(splitted[1].to_string()),
                });

                builder.set_original_code(format!("{}", code));
                builder.set_original_message(e.message());

                builder.build()
            }

            code if code == 1264 => {
                let mut builder = Error::builder(ErrorKind::ValueOutOfRange {
                    message: e.message().to_string(),
                });

                builder.set_original_code(format!("{}", code));
                builder.set_original_message(e.message());

                builder.build()
            }

            code if code == 1364 || code == 1048 => {
                let splitted: Vec<&str> = e.message().split_whitespace().collect();
                let splitted: Vec<&str> = splitted.get(1).map(|s| s.split('\'').collect()).unwrap();

                let mut builder = Error::builder(ErrorKind::NullConstraintViolation {
                    constraint: DatabaseConstraint::Fields(vec![splitted[1].to_string()]),
                });

                builder.set_original_code(format!("{}", code));
                builder.set_original_message(e.message());

                builder.build()
            }

            code if code == 1049 => {
                let splitted: Vec<&str> = e.message().split_whitespace().collect();
                let splitted: Vec<&str> = splitted.last().map(|s| s.split('\'').collect()).unwrap();
                let db_name: String = splitted[1].into();

                let mut builder = Error::builder(ErrorKind::DatabaseDoesNotExist { db_name });

                builder.set_original_code(format!("{}", code));
                builder.set_original_message(e.message());

                builder.build()
            }

            code if code == 1007 => {
                let splitted: Vec<&str> = e.message().split_whitespace().collect();
                let splitted: Vec<&str> = splitted[3].split('\'').collect();
                let db_name: String = splitted[1].into();

                let mut builder = Error::builder(ErrorKind::DatabaseAlreadyExists { db_name });

                builder.set_original_code(format!("{}", code));
                builder.set_original_message(e.message());

                builder.build()
            }

            code if code == 1044 => {
                let splitted: Vec<&str> = e.message().split_whitespace().collect();
                let splitted: Vec<&str> = splitted.last().map(|s| s.split('\'').collect()).unwrap();
                let db_name: String = splitted[1].into();

                let mut builder = Error::builder(ErrorKind::DatabaseAccessDenied { db_name });

                builder.set_original_code(format!("{}", code));
                builder.set_original_message(e.message());

                builder.build()
            }

            code if code == 1045 => {
                let splitted: Vec<&str> = e.message().split_whitespace().collect();
                let splitted: Vec<&str> = splitted[4].split('@').collect();
                let splitted: Vec<&str> = splitted[0].split('\'').collect();
                let user: String = splitted[1].into();

                let mut builder = Error::builder(ErrorKind::AuthenticationFailed { user });

                builder.set_original_code(format!("{}", code));
                builder.set_original_message(e.message());

                builder.build()
            }
            code if code == 1146 => {
                let splitted: Vec<&str> = e.message().split_whitespace().collect();
                let splitted: Vec<&str> = splitted[1].split('\'').collect();
                let splitted: Vec<&str> = splitted[1].split('.').collect();
                let table = splitted.last().unwrap().to_string();

                let mut builder = Error::builder(ErrorKind::TableDoesNotExist { table });
                builder.set_original_code(format!("{}", code));
                builder.set_original_message(e.message());

                builder.build()
            }

            code if code == 1406 => {
                let splitted: Vec<&str> = e.message().split_whitespace().collect();
                let column_name = splitted[6];

                let mut builder = Error::builder(ErrorKind::LengthMismatch {
                    column: Some(column_name.to_owned()),
                });

                builder.set_original_code(format!("{}", code));
                builder.set_original_message(e.message());

                builder.build()
            }

            code => {
                let message = e.message().to_string();
                let kind = ErrorKind::QueryError(e.into());

                let mut builder = Error::builder(kind);
                builder.set_original_code(format!("{}", code));
                builder.set_original_message(message);

                builder.build()
            }
        }
    }
}
