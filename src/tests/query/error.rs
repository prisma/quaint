use super::*;
use crate::{
    ast::*,
    connector::Queryable,
    error::{DatabaseConstraint, ErrorKind},
};
use test_macros::test_each_connector;

#[test_each_connector]
async fn table_does_not_exist(api: &mut dyn TestApi) -> crate::Result<()> {
    let select = Select::from_table("not_there");

    let err = api.conn().select(select).await.unwrap_err();

    match err.kind() {
        ErrorKind::TableDoesNotExist { table } => {
            assert_eq!("not_there", table.as_str());
        }
        e => panic!("Expected error TableDoesNotExist, got {:?}", e),
    }

    Ok(())
}

#[test_each_connector]
async fn unique_constraint_violation(api: &mut dyn TestApi) -> crate::Result<()> {
    let table = api.create_table("id1 int, id2 int").await?;
    let index = api.create_index(&table, "id1, id2").await?;

    let insert = Insert::single_into(&table).value("id1", 1).value("id2", 2);
    api.conn().insert(insert.clone().into()).await?;

    let res = api.conn().insert(insert.clone().into()).await;

    assert!(res.is_err());

    let err = res.unwrap_err();

    match &err.kind() {
        ErrorKind::UniqueConstraintViolation { constraint } => match constraint {
            DatabaseConstraint::Index(idx) => assert_eq!(&index, idx),
            DatabaseConstraint::Fields(fields) => {
                let fields = fields.iter().map(|s| s.as_str()).collect::<Vec<_>>();
                assert_eq!(vec!["id1", "id2"], fields)
            }
            DatabaseConstraint::ForeignKey => assert!(false, "Expecting index or field constraints"),
        },
        _ => panic!(err),
    }

    Ok(())
}

#[test_each_connector]
async fn null_constraint_violation(api: &mut dyn TestApi) -> crate::Result<()> {
    let table = api.create_table("id1 int not null, id2 int not null").await?;

    let res = api.conn().insert(Insert::single_into(&table).into()).await;
    let err = res.unwrap_err();

    match err.kind() {
        ErrorKind::NullConstraintViolation { constraint } => {
            assert_eq!(&DatabaseConstraint::Fields(vec![String::from("id1")]), constraint)
        }
        _ => panic!(err),
    }

    let insert = Insert::single_into(&table).value("id1", 50).value("id2", 55);
    api.conn().insert(insert.into()).await?;

    let update = Update::table(&table).set("id2", Value::Integer(None));
    let res = api.conn().update(update).await;

    assert!(res.is_err());

    let err = res.unwrap_err();

    match err.kind() {
        ErrorKind::NullConstraintViolation { constraint } => {
            assert_eq!(&DatabaseConstraint::Fields(vec![String::from("id2")]), constraint);
        }
        _ => panic!(err),
    }

    Ok(())
}

#[test_each_connector(tags("mysql"))]
async fn int_unsigned_negative_value_out_of_range(api: &mut dyn TestApi) -> crate::Result<()> {
    let table = api
        .create_table("id int4 auto_increment primary key, big int4 unsigned")
        .await?;

    // Negative value
    {
        let insert = Insert::multi_into(&table, &["big"]).values((-22,));
        let result = api.conn().insert(insert.into()).await;

        assert!(matches!(result.unwrap_err().kind(), ErrorKind::ValueOutOfRange { .. }));
    }

    // Value too big
    {
        let insert = Insert::multi_into(&table, &["big"]).values((std::i64::MAX,));
        let result = api.conn().insert(insert.into()).await;

        assert!(matches!(result.unwrap_err().kind(), ErrorKind::ValueOutOfRange { .. }));
    }

    Ok(())
}

#[test_each_connector(tags("mysql"))]
async fn bigint_unsigned_positive_value_out_of_range(api: &mut dyn TestApi) -> crate::Result<()> {
    let table = api
        .create_table("id int4 auto_increment primary key, big bigint unsigned")
        .await?;

    let insert = format!(r#"INSERT INTO `{}` (`big`) VALUES (18446744073709551615)"#, table);
    api.conn().execute_raw(&insert, vec![]).await.unwrap();
    let result = api.conn().select(Select::from_table(&table)).await;

    assert!(
        matches!(result.unwrap_err().kind(), ErrorKind::ValueOutOfRange { message } if message == "Unsigned integers larger than 9_223_372_036_854_775_807 are currently not handled.")
    );

    Ok(())
}

#[test_each_connector(tags("mysql", "mssql", "postgres"))]
async fn length_mismatch(api: &mut dyn TestApi) -> crate::Result<()> {
    let table = api.create_table("value varchar(3)").await?;
    let insert = Insert::single_into(&table).value("value", "fooo");

    let result = api.conn().insert(insert.into()).await;
    assert!(result.is_err());

    let err = result.unwrap_err();
    assert!(matches!(err.kind(), ErrorKind::LengthMismatch { .. }));

    Ok(())
}

#[test_each_connector(tags("postgres", "sqlite"))]
async fn foreign_key_constraint_violation(api: &mut dyn TestApi) -> crate::Result<()> {
    let parent = api.create_table("id smallint not null primary key").await?;
    let foreign_key = api.foreign_key(&parent, "id", "parent_id");
    let child = api
        .create_table(&format!("parent_id smallint not null, {}", &foreign_key))
        .await?;

    let insert = Insert::single_into(&child).value("parent_id", 10);
    let result = api.conn().insert(insert.into()).await;

    assert!(result.is_err());

    let err = result.unwrap_err();
    assert!(matches!(err.kind(), ErrorKind::ForeignKeyConstraintViolation { .. }));

    Ok(())
}

/// SQL Server and MySQL do not allow foreign keys in temporary tables, so
/// we'll do them separately.
#[test_each_connector(tags("mssql", "mysql"))]
async fn ms_my_foreign_key_constraint_violation(api: &mut dyn TestApi) -> crate::Result<()> {
    let parent_table = api.get_name();
    let child_table = api.get_name();
    let constraint = api.get_name();

    let create_table = format!("CREATE TABLE {} (id smallint not null primary key)", parent_table);
    api.conn().raw_cmd(&create_table).await?;

    let create_table = format!(
        r#"
        CREATE TABLE {} (
            parent_id smallint not null,
            CONSTRAINT {} FOREIGN KEY (parent_id) REFERENCES {}(id))
        "#,
        &child_table, &constraint, &parent_table
    );

    api.conn().raw_cmd(&create_table).await?;

    let insert = Insert::single_into(&child_table).value("parent_id", 10);
    let result = api.conn().insert(insert.into()).await;

    assert!(result.is_err());

    let err = result.unwrap_err();
    assert!(matches!(err.kind(), ErrorKind::ForeignKeyConstraintViolation { .. }));

    api.conn().raw_cmd(&format!("DROP TABLE {}", &child_table)).await?;
    api.conn().raw_cmd(&format!("DROP TABLE {}", &parent_table)).await?;

    Ok(())
}
