use crate::ast::Value;
use sqlx::Database;

pub trait Bind<'a, DB>
where
    DB: Database,
{
    fn bind_value(self, value: Value<'a>, type_info: Option<&DB::TypeInfo>) -> crate::Result<Self>
    where
        Self: Sized;
}
