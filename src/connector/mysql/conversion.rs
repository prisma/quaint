use crate::{ast::ParameterizedValue, connector::queryable::TakeRow, error::ErrorKind};
#[cfg(feature = "chrono-0_4")]
use chrono::{DateTime, Datelike, NaiveDate, NaiveDateTime, NaiveTime, Timelike, Utc};
use mysql_async as my;
use mysql_async::Value as MyValue;
use rust_decimal::prelude::ToPrimitive;

pub fn conv_params<'a>(params: &[ParameterizedValue<'a>]) -> my::Params {
    if params.is_empty() {
        // If we don't use explicit 'Empty',
        // mysql crashes with 'internal error: entered unreachable code'
        my::Params::Empty
    } else {
        my::Params::Positional(params.iter().map(|x| x.into()).collect::<Vec<my::Value>>())
    }
}

impl TakeRow for my::Row {
    fn take_result_row<'b>(&'b mut self) -> crate::Result<Vec<ParameterizedValue<'static>>> {
        fn convert(row: &mut my::Row, i: usize) -> crate::Result<ParameterizedValue<'static>> {
            let value = row.take(i).unwrap_or(my::Value::NULL);
            let column = match row.columns_ref().get(i) {
                Some(col) => col,
                None => return Ok(ParameterizedValue::Null),
            };
            let res = match value {
                my::Value::NULL => ParameterizedValue::Null,
                // JSON is returned as bytes.
                #[cfg(feature = "json-1")]
                my::Value::Bytes(b) if column.column_type() == mysql_async::consts::ColumnType::MYSQL_TYPE_JSON => {
                    serde_json::from_slice(&b)
                        .map(|val| ParameterizedValue::Json(val))
                        .map_err(|e| {
                            crate::error::Error::builder(ErrorKind::ConversionError("Unable to convert bytes to JSON"))
                                .build()
                        })?
                }
                // NEWDECIMAL returned as bytes. See https://mariadb.com/kb/en/resultset-row/#decimal-binary-encoding
                my::Value::Bytes(b)
                    if column.column_type() == mysql_async::consts::ColumnType::MYSQL_TYPE_NEWDECIMAL =>
                {
                    ParameterizedValue::Real(
                        String::from_utf8(b)
                            .expect("MySQL NEWDECIMAL as string")
                            .parse()
                            .map_err(|_err| {
                                crate::error::Error::builder(ErrorKind::ConversionError("mysql NEWDECIMAL conversion"))
                                    .build()
                            })?,
                    )
                }
                // https://dev.mysql.com/doc/internals/en/character-set.html
                my::Value::Bytes(b) if column.character_set() == 63 => ParameterizedValue::Bytes(b.into()),
                my::Value::Bytes(s) => ParameterizedValue::Text(String::from_utf8(s)?.into()),
                my::Value::Int(i) => ParameterizedValue::Integer(i),
                // TOOD: This is unsafe
                my::Value::UInt(i) => ParameterizedValue::Integer(i as i64),
                my::Value::Float(f) => ParameterizedValue::from(f),
                my::Value::Double(f) => ParameterizedValue::from(f),
                #[cfg(feature = "chrono-0_4")]
                my::Value::Date(year, month, day, hour, min, sec, micro) => {
                    let time = NaiveTime::from_hms_micro(hour.into(), min.into(), sec.into(), micro);

                    let date = NaiveDate::from_ymd(year.into(), month.into(), day.into());
                    let dt = NaiveDateTime::new(date, time);

                    ParameterizedValue::DateTime(DateTime::<Utc>::from_utc(dt, Utc))
                }
                #[cfg(feature = "chrono-0_4")]
                my::Value::Time(is_neg, days, hours, minutes, seconds, micros) => {
                    if is_neg {
                        return Err(crate::error::Error::builder(ErrorKind::ConversionError(
                            "Failed to convert a negative time",
                        ))
                        .build());
                    }

                    if days != 0 {
                        return Err(crate::error::Error::builder(ErrorKind::ConversionError(
                            "Failed to read a MySQL `time` as duration",
                        ))
                        .build());
                    }

                    let time = NaiveTime::from_hms_micro(hours.into(), minutes.into(), seconds.into(), micros);

                    ParameterizedValue::DateTime(DateTime::<Utc>::from_utc(
                        NaiveDateTime::new(NaiveDate::from_ymd(1970, 1, 1), time),
                        Utc,
                    ))
                }
                #[cfg(not(feature = "chrono-0_4"))]
                typ => panic!(
                    "Value of type {:?} is not supported with the current configuration",
                    typ
                ),
            };

            Ok(res)
        }

        let mut row = Vec::with_capacity(self.len());

        for i in 0..self.len() {
            row.push(convert(self, i)?);
        }

        Ok(row)
    }
}

impl<'a> From<ParameterizedValue<'a>> for MyValue {
    fn from(pv: ParameterizedValue<'a>) -> MyValue {
        match pv {
            ParameterizedValue::Null => MyValue::NULL,
            ParameterizedValue::Integer(i) => MyValue::Int(i),
            ParameterizedValue::Real(f) => MyValue::Double(f.to_f64().expect("Decimal is not a f64.")),
            ParameterizedValue::Text(s) => MyValue::Bytes((&*s).as_bytes().to_vec()),
            ParameterizedValue::Bytes(bytes) => MyValue::Bytes(bytes.into_owned()),
            ParameterizedValue::Enum(s) => MyValue::Bytes((&*s).as_bytes().to_vec()),
            ParameterizedValue::Boolean(b) => MyValue::Int(b as i64),
            ParameterizedValue::Char(c) => MyValue::Bytes(vec![c as u8]),
            #[cfg(feature = "json-1")]
            ParameterizedValue::Json(json) => {
                let s = serde_json::to_string(&json).expect("Cannot convert JSON to String.");

                MyValue::Bytes(s.into_bytes())
            }
            #[cfg(feature = "array")]
            ParameterizedValue::Array(_) => unimplemented!("Arrays are not supported for mysql."),
            #[cfg(feature = "uuid-0_8")]
            ParameterizedValue::Uuid(u) => MyValue::Bytes(u.to_hyphenated().to_string().into_bytes()),
            #[cfg(feature = "chrono-0_4")]
            ParameterizedValue::DateTime(dt) => MyValue::Date(
                dt.year() as u16,
                dt.month() as u8,
                dt.day() as u8,
                dt.hour() as u8,
                dt.minute() as u8,
                dt.second() as u8,
                dt.timestamp_subsec_micros(),
            ),
        }
    }
}
