use crate::{
    ast::Value,
    connector::bind::Bind,
    error::{Error, ErrorKind},
};
use chrono::{offset::Utc, DateTime, NaiveDate, NaiveTime};
use rust_decimal::{prelude::FromPrimitive, Decimal};
use sqlx::{
    decode::Decode,
    mysql::{MySqlArguments, MySqlRow, MySqlTypeInfo},
    query::Query,
    MySql, Row, Type, TypeInfo, ValueRef,
};
use std::{borrow::Cow, convert::TryFrom};

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum MyValue<'a> {
    /// 64-bit signed integer.
    Integer(Option<i64>),
    /// A decimal value.
    Real(Option<Decimal>),
    /// String value.
    Text(Option<Cow<'a, str>>),
    /// Bytes value.
    Bytes(Option<Cow<'a, [u8]>>),
    /// Boolean value.
    Boolean(Option<bool>),
    #[cfg(feature = "json-1")]
    /// A JSON value.
    Json(Option<serde_json::Value>),
    #[cfg(feature = "chrono-0_4")]
    /// A datetime value.
    DateTime(Option<chrono::DateTime<chrono::offset::Utc>>),
    #[cfg(feature = "chrono-0_4")]
    /// A date value.
    Date(Option<chrono::NaiveDate>),
    #[cfg(feature = "chrono-0_4")]
    /// A time value.
    Time(Option<chrono::NaiveTime>),
}

impl<'a> Bind<'a, MySql> for Query<'a, MySql, MySqlArguments> {
    fn bind_value(self, value: Value<'a>, _: Option<&MySqlTypeInfo>) -> crate::Result<Self> {
        let query = match MyValue::try_from(value)? {
            MyValue::Integer(i) => self.bind(i),
            MyValue::Real(r) => self.bind(r),
            MyValue::Text(s) => self.bind(s.map(|s| s.into_owned())),
            MyValue::Bytes(b) => self.bind(b.map(|s| s.into_owned())),
            MyValue::Boolean(b) => self.bind(b),
            MyValue::Json(j) => self.bind(j),
            MyValue::DateTime(d) => self.bind(d),
            MyValue::Date(d) => self.bind(d),
            MyValue::Time(t) => self.bind(t),
        };

        Ok(query)
    }
}

impl<'a> TryFrom<Value<'a>> for MyValue<'a> {
    type Error = Error;

    fn try_from(v: Value<'a>) -> crate::Result<Self> {
        match v {
            Value::Integer(i) => Ok(MyValue::Integer(i)),
            Value::Real(r) => Ok(MyValue::Real(r)),
            Value::Text(s) => Ok(MyValue::Text(s)),
            Value::Enum(e) => Ok(MyValue::Text(e)),
            Value::Bytes(b) => Ok(MyValue::Bytes(b)),
            Value::Boolean(b) => Ok(MyValue::Boolean(b)),
            Value::Char(c) => Ok(MyValue::Text(c.map(|c| c.to_string().into()))),
            #[cfg(all(feature = "array", feature = "postgresql"))]
            Value::Array(_) => {
                let msg = "Arrays are not supported in MySQL.";
                let kind = ErrorKind::conversion(msg);

                let mut builder = Error::builder(kind);
                builder.set_original_message(msg);

                Err(builder.build())?
            }
            #[cfg(feature = "json-1")]
            Value::Json(j) => Ok(MyValue::Json(j)),
            #[cfg(feature = "uuid-0_8")]
            Value::Uuid(u) => Ok(MyValue::Text(u.map(|u| u.to_hyphenated().to_string().into()))),
            #[cfg(feature = "chrono-0_4")]
            Value::DateTime(d) => Ok(MyValue::DateTime(d)),
            #[cfg(feature = "chrono-0_4")]
            Value::Date(d) => Ok(MyValue::Date(d)),
            #[cfg(feature = "chrono-0_4")]
            Value::Time(t) => Ok(MyValue::Time(t)),
        }
    }
}

pub fn map_row<'a>(row: MySqlRow) -> Result<Vec<Value<'a>>, sqlx::Error> {
    let mut result = Vec::with_capacity(row.len());

    for i in 0..row.len() {
        let value_ref = row.try_get_raw(i)?;

        let decode_err = |source| sqlx::Error::ColumnDecode {
            index: format!("{}", i),
            source,
        };

        let value = match value_ref.type_info() {
            ti if <i64 as Type<MySql>>::compatible(&ti) => {
                let int_opt = Decode::<MySql>::decode(value_ref).map_err(decode_err)?;

                Value::Integer(int_opt)
            }

            ti if <u64 as Type<MySql>>::compatible(&ti) => {
                let uint_opt: Option<u64> = Decode::<MySql>::decode(value_ref).map_err(decode_err)?;

                Value::Integer(uint_opt.map(|u| u as i64))
            }

            ti if <Decimal as Type<MySql>>::compatible(&ti) => {
                let decimal_opt = Decode::<MySql>::decode(value_ref).map_err(decode_err)?;

                Value::Real(decimal_opt)
            }

            ti if <f32 as Type<MySql>>::compatible(&ti) => {
                let f_opt: Option<f32> = Decode::<MySql>::decode(value_ref).map_err(decode_err)?;

                Value::Real(f_opt.map(|f| Decimal::from_f32(f).unwrap()))
            }

            ti if <f64 as Type<MySql>>::compatible(&ti) => {
                let f_opt: Option<f64> = Decode::<MySql>::decode(value_ref).map_err(decode_err)?;

                Value::Real(f_opt.map(|f| Decimal::from_f64(f).unwrap()))
            }

            ti if <String as Type<MySql>>::compatible(&ti) && ti.name() == "ENUM" => {
                let string_opt: Option<String> = Decode::<MySql>::decode(value_ref).map_err(decode_err)?;

                Value::Enum(string_opt.map(Cow::from))
            }

            ti if <String as Type<MySql>>::compatible(&ti) => {
                let string_opt: Option<String> = Decode::<MySql>::decode(value_ref).map_err(decode_err)?;

                Value::Text(string_opt.map(Cow::from))
            }

            ti if <Vec<u8> as Type<MySql>>::compatible(&ti) => {
                let bytes_opt: Option<Vec<u8>> = Decode::<MySql>::decode(value_ref).map_err(decode_err)?;

                Value::Bytes(bytes_opt.map(Cow::from))
            }

            ti if <bool as Type<MySql>>::compatible(&ti) => {
                let bool_opt = Decode::<MySql>::decode(value_ref).map_err(decode_err)?;

                Value::Boolean(bool_opt)
            }

            #[cfg(feature = "chrono-0_4")]
            ti if <DateTime<Utc> as Type<MySql>>::compatible(&ti) => {
                let dt_opt = Decode::<MySql>::decode(value_ref).map_err(decode_err)?;

                Value::DateTime(dt_opt)
            }

            #[cfg(feature = "chrono-0_4")]
            ti if <NaiveDate as Type<MySql>>::compatible(&ti) => {
                let date_opt = Decode::<MySql>::decode(value_ref).map_err(decode_err)?;

                Value::Date(date_opt)
            }

            #[cfg(feature = "chrono-0_4")]
            ti if <NaiveTime as Type<MySql>>::compatible(&ti) => {
                let time_opt = Decode::<MySql>::decode(value_ref).map_err(decode_err)?;

                Value::Time(time_opt)
            }

            #[cfg(feature = "json-1")]
            ti if <serde_json::Value as Type<MySql>>::compatible(&ti) => {
                let json_opt = Decode::<MySql>::decode(value_ref).map_err(decode_err)?;

                Value::Json(json_opt)
            }

            ti => {
                let msg = format!("Type {} is not yet supported in the MySQL connector.", ti.name());
                let kind = ErrorKind::conversion(msg.clone());

                let mut builder = Error::builder(kind);
                builder.set_original_message(msg);

                let error = sqlx::Error::ColumnDecode {
                    index: format!("{}", i),
                    source: Box::new(builder.build()),
                };

                Err(error)?
            }
        };

        result.push(value);
    }

    Ok(result)
}
