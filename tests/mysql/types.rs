use names::Generator;
use once_cell::sync::Lazy;
use quaint::{connector::Queryable, single::Quaint};
use std::env;

static CONN_STR: Lazy<String> = Lazy::new(|| env::var("TEST_MYSQL").expect("TEST_MYSQL env var"));

pub struct MySql<'a> {
    names: Generator<'a>,
    conn: Quaint,
}

impl<'a> MySql<'a> {
    pub async fn new() -> quaint::Result<MySql<'a>> {
        let names = Generator::default();
        let conn = Quaint::new(&CONN_STR).await?;

        Ok(Self { names, conn })
    }

    pub async fn create_table(&mut self, r#type: &str) -> quaint::Result<String> {
        let table = self.names.next().unwrap().replace('-', "");

        let create_table = format!(
            r##"
            CREATE TEMPORARY TABLE `{}` (
                `id` int(11) NOT NULL AUTO_INCREMENT,
                `value` {},
                PRIMARY KEY (`id`)
            ) ENGINE=InnoDB DEFAULT CHARSET=latin1
            "##,
            table, r#type,
        );

        self.conn.raw_cmd(&create_table).await?;

        Ok(table)
    }

    pub fn conn(&self) -> &Quaint {
        &self.conn
    }
}

#[macro_export]
macro_rules! test_type {
    ($name:ident($db:ident, $sql_type:literal, $($value:expr),+ $(,)?)) => {
        paste::item! {
            #[test]
            fn [< test_type_ $name >] () -> quaint::Result<()> {
                use quaint::ast::*;
                use quaint::connector::Queryable;
                use tokio::runtime::Builder;

                let mut rt = Builder::new().threaded_scheduler().enable_io().enable_time().build().unwrap();

                rt.block_on(async {
                    let mut setup = $db::new().await?;
                    let table = setup.create_table($sql_type).await?;

                    $(
                        let insert = Insert::single_into(&table).value("value", $value);
                        setup.conn().insert(insert.into()).await?;

                        let select = Select::from_table(&table).column("value").order_by("id".descend());
                        let res = setup.conn().select(select).await?.into_single()?;

                        assert_eq!(Some(&$value), res.at(0));
                    )+

                    Result::<(), quaint::error::Error>::Ok(())
                }).unwrap();

                Ok(())
            }
        }
    }
}

test_type!(tinyint(MySql, "tinyint(4)", Value::integer(10), Value::integer(-1)));
