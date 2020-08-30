use crate::error::{DatabaseConstraint, Error, ErrorKind};
use sqlx::postgres::PgDatabaseError;

impl From<PgDatabaseError> for Error {
    fn from(e: PgDatabaseError) -> Self {
        match e.code().to_string() {
            code if code == "22001" => {
                let mut builder = Error::builder(ErrorKind::LengthMismatch { column: None });

                builder.set_original_code(code);
                builder.set_original_message(e.message());

                builder.build()
            }
            code if code == "23505" => {
                let detail = e.detail().unwrap();
                let splitted: Vec<&str> = detail.split(")=(").collect();
                let splitted: Vec<&str> = splitted[0].split(" (").collect();

                let field_names = splitted[1].replace("\"", "");
                let field_names: Vec<String> = field_names.split(", ").map(|s| s.to_string()).collect();

                let mut builder = Error::builder(ErrorKind::UniqueConstraintViolation {
                    constraint: DatabaseConstraint::Fields(field_names),
                });

                builder.set_original_code(code);
                builder.set_original_message(detail);

                builder.build()
            }
            code if code == "23502" => {
                let column_name = e
                    .column()
                    .expect("column on null constraint violation error")
                    .to_owned();

                let mut builder = Error::builder(ErrorKind::NullConstraintViolation {
                    constraint: DatabaseConstraint::Fields(vec![column_name]),
                });

                builder.set_original_code(code);
                builder.set_original_message(e.message());

                builder.build()
            }
            code if code == "23503" => {
                let code = code.to_string();

                match e.column() {
                    Some(column) => {
                        let column_name = column.to_owned();

                        let mut builder = Error::builder(ErrorKind::ForeignKeyConstraintViolation {
                            constraint: DatabaseConstraint::Fields(vec![column_name]),
                        });

                        builder.set_original_code(code);
                        builder.set_original_message(e.message());

                        builder.build()
                    }
                    None => {
                        let message = e.message();
                        let mut splitted = message.split_whitespace();
                        let constraint = splitted.nth(10).unwrap().split('"').nth(1).unwrap().to_string();

                        let mut builder = Error::builder(ErrorKind::ForeignKeyConstraintViolation {
                            constraint: DatabaseConstraint::Index(constraint),
                        });

                        builder.set_original_code(code);
                        builder.set_original_message(message);

                        builder.build()
                    }
                }
            }
            code if code == "3D000" => {
                let splitted: Vec<&str> = e.message().split_whitespace().collect();
                let splitted: Vec<&str> = splitted[1].split('"').collect();
                let db_name = splitted[1].into();

                let mut builder = Error::builder(ErrorKind::DatabaseDoesNotExist { db_name });

                builder.set_original_code(code);
                builder.set_original_message(e.message());

                builder.build()
            }
            code if code == "28P01" => {
                let splitted: Vec<&str> = e.message().split_whitespace().collect();
                let splitted: Vec<&str> = splitted.last().unwrap().split('"').collect();
                let user = splitted[1].into();

                let mut builder = Error::builder(ErrorKind::AuthenticationFailed { user });

                builder.set_original_code(code);
                builder.set_original_message(e.message());

                builder.build()
            }
            code if code == "42P01" => {
                let code = code.to_string();
                let message = e.message();

                let splitted: Vec<&str> = message.split_whitespace().collect();
                let splitted: Vec<&str> = splitted[1].split('"').collect();
                let table = splitted[1].into();

                let mut builder = Error::builder(ErrorKind::TableDoesNotExist { table });
                builder.set_original_code(code);
                builder.set_original_message(message);

                builder.build()
            }
            code if code == "42P04" => {
                let splitted: Vec<&str> = e.message().split_whitespace().collect();
                let splitted: Vec<&str> = splitted[1].split('"').collect();
                let db_name = splitted[1].into();

                let mut builder = Error::builder(ErrorKind::DatabaseAlreadyExists { db_name });

                builder.set_original_code(code);
                builder.set_original_message(e.message());

                builder.build()
            }
            code => {
                let message = e.message().to_string();
                let mut builder = Error::builder(ErrorKind::QueryError(e.into()));

                builder.set_original_code(code);
                builder.set_original_message(message);
                builder.build()
            }
        }
    }
}
