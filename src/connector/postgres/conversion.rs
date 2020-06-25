use crate::{
    ast::Value,
    connector::bind::Bind,
    error::{Error, ErrorKind},
};
use rust_decimal::{
    prelude::{FromPrimitive, ToPrimitive},
    Decimal,
};
#[cfg(feature = "chrono-0_4")]
use sqlx::postgres::types::PgTimeTz;
use sqlx::{
    postgres::{types::PgMoney, PgArguments, PgRow, PgTypeInfo, PgTypeKind},
    query::Query,
    types::Json,
    Column, Postgres, Row, TypeInfo,
};
use std::borrow::Cow;

impl<'a> Bind<'a, Postgres> for Query<'a, Postgres, PgArguments> {
    #[inline]
    fn bind_value(self, value: Value<'a>, type_info: Option<&PgTypeInfo>) -> crate::Result<Self> {
        let query = match (value, type_info.map(|ti| ti.name())) {
            // integers
            (Value::Integer(i), Some("INT2")) => self.bind(i.map(|i| i as i16)),
            (Value::Integer(i), Some("INT4")) => self.bind(i.map(|i| i as i32)),
            (Value::Integer(i), Some("OID")) => self.bind(i.map(|i| i as u32)),
            (Value::Integer(i), Some("TEXT")) => self.bind(i.map(|i| format!("{}", i))),
            (Value::Integer(i), _) => self.bind(i.map(|i| i as i64)),

            // floating and real
            (Value::Real(d), Some("FLOAT4")) => match d {
                Some(decimal) => {
                    let f = decimal.to_f32().ok_or_else(|| {
                        let kind = ErrorKind::conversion("Could not convert `Decimal` into `f32`.");
                        Error::builder(kind).build()
                    })?;

                    self.bind(f)
                }
                None => self.bind(Option::<f32>::None),
            },
            (Value::Real(d), Some("FLOAT8")) => match d {
                Some(decimal) => {
                    let f = decimal.to_f64().ok_or_else(|| {
                        let kind = ErrorKind::conversion("Could not convert `Decimal` into `f32`.");
                        Error::builder(kind).build()
                    })?;

                    self.bind(f)
                }
                None => self.bind(Option::<f64>::None),
            },
            (Value::Real(d), Some("MONEY")) => self.bind(d.map(|r| PgMoney::from_decimal(r, 2))),
            (Value::Real(d), _) => self.bind(d),

            #[cfg(feature = "uuid-0_8")]
            (Value::Text(val), Some("UUID")) => match val {
                Some(cow) => {
                    let id: uuid::Uuid = cow.parse().map_err(|_| {
                        let kind = ErrorKind::conversion(format!(
                            "The given string '{}' could not be converted to UUID.",
                            cow
                        ));
                        Error::builder(kind).build()
                    })?;
                    self.bind(id)
                }
                None => self.bind(Option::<uuid::Uuid>::None),
            },
            // strings
            #[cfg(feature = "ipnetwork")]
            (Value::Text(c), t) if t == Some("INET") || t == Some("CIDR") => match c {
                Some(s) => {
                    let ip: sqlx::types::ipnetwork::IpNetwork = s.parse().map_err(|_| {
                        let msg = format!("Provided IP address ({}) not in the right format.", s);
                        let kind = ErrorKind::conversion(msg);

                        Error::builder(kind).build()
                    })?;

                    self.bind(ip)
                }
                None => self.bind(Option::<sqlx::types::ipnetwork::IpNetwork>::None),
            },
            #[cfg(feature = "bit-vec")]
            (Value::Text(c), t) if t == Some("BIT") || t == Some("VARBIT") => match c {
                Some(s) => {
                    let bits = string_to_bits(&s)?;
                    self.bind(bits)
                }
                None => self.bind(Option::<sqlx::types::ipnetwork::IpNetwork>::None),
            },
            (Value::Text(c), _)
                if type_info
                    .map(|ti| matches!(ti.kind(), PgTypeKind::Enum(_)))
                    .unwrap_or(false) =>
            {
                self.bind(c.map(|c| c.into_owned()))
            }
            (Value::Text(c), _) => self.bind(c.map(|c| c.into_owned())),
            (Value::Enum(c), _) => self.bind(c.map(|c| c.into_owned())),

            (Value::Bytes(c), _) => self.bind(c.map(|c| c.into_owned())),
            (Value::Boolean(b), _) => self.bind(b),
            (Value::Char(c), _) => self.bind(c.map(|c| c as i8)),

            #[cfg(all(feature = "bit-vec", feature = "array"))]
            (Value::Array(ary_opt), t) if t == Some("BIT[]") || t == Some("VARBIT[]") => match ary_opt {
                Some(ary) => {
                    let mut bits = Vec::with_capacity(ary.len());

                    for val in ary.into_iter().map(|v| v.into_string()) {
                        match val {
                            Some(s) => {
                                let bit = string_to_bits(&s)?;
                                bits.push(bit);
                            }
                            None => {
                                let msg = "Non-string parameter when storing a BIT[]";
                                let kind = ErrorKind::conversion(msg);

                                Err(Error::builder(kind).build())?
                            }
                        }
                    }

                    self.bind(bits)
                }
                None => self.bind(Option::<Vec<i16>>::None),
            },

            #[cfg(feature = "array")]
            (Value::Array(ary_opt), Some("INT2[]")) => match ary_opt {
                Some(ary) => {
                    let mut ints = Vec::with_capacity(ary.len());

                    for val in ary.into_iter().map(|v| v.as_i64().map(|i| i as i16)) {
                        match val {
                            Some(int) => {
                                ints.push(int);
                            }
                            None => {
                                let msg = "Non-integer parameter when storing an INT2[]";
                                let kind = ErrorKind::conversion(msg);

                                Err(Error::builder(kind).build())?
                            }
                        }
                    }

                    self.bind(ints)
                }
                None => self.bind(Option::<Vec<i16>>::None),
            },

            #[cfg(feature = "array")]
            (Value::Array(ary_opt), Some("INT4[]")) => match ary_opt {
                Some(ary) => {
                    let mut ints = Vec::with_capacity(ary.len());

                    for val in ary.into_iter().map(|v| v.as_i64().map(|i| i as i32)) {
                        match val {
                            Some(int) => {
                                ints.push(int);
                            }
                            None => {
                                let msg = "Non-integer parameter when storing an INT4[]";
                                let kind = ErrorKind::conversion(msg);

                                Err(Error::builder(kind).build())?
                            }
                        }
                    }

                    self.bind(ints)
                }
                None => self.bind(Option::<Vec<i32>>::None),
            },

            #[cfg(feature = "array")]
            (Value::Array(ary_opt), Some("INT8[]")) => match ary_opt {
                Some(ary) => {
                    let mut ints = Vec::with_capacity(ary.len());

                    for val in ary.into_iter().map(|v| v.as_i64()) {
                        match val {
                            Some(int) => {
                                ints.push(int);
                            }
                            None => {
                                let msg = "Non-integer parameter when storing an INT8[]";
                                let kind = ErrorKind::conversion(msg);

                                Err(Error::builder(kind).build())?
                            }
                        }
                    }

                    self.bind(ints)
                }
                None => self.bind(Option::<Vec<i64>>::None),
            },

            #[cfg(feature = "array")]
            (Value::Array(ary_opt), Some("OID[]")) => match ary_opt {
                Some(ary) => {
                    let mut ints = Vec::with_capacity(ary.len());

                    for val in ary.into_iter().map(|v| v.as_i64().map(|i| i as u32)) {
                        match val {
                            Some(int) => {
                                ints.push(int);
                            }
                            None => {
                                let msg = "Non-integer parameter when storing an OID[]";
                                let kind = ErrorKind::conversion(msg);

                                Err(Error::builder(kind).build())?
                            }
                        }
                    }

                    self.bind(ints)
                }
                None => self.bind(Option::<Vec<u32>>::None),
            },

            #[cfg(feature = "array")]
            (Value::Array(ary_opt), Some("FLOAT4[]")) => match ary_opt {
                Some(ary) => {
                    let mut floats = Vec::with_capacity(ary.len());

                    for val in ary.into_iter().map(|v| v.as_f64().map(|i| i as f32)) {
                        match val {
                            Some(float) => {
                                floats.push(float);
                            }
                            None => {
                                let msg = "Non-float parameter when storing a FLOAT4[]";
                                let kind = ErrorKind::conversion(msg);

                                Err(Error::builder(kind).build())?
                            }
                        }
                    }

                    self.bind(floats)
                }
                None => self.bind(Option::<Vec<f32>>::None),
            },

            #[cfg(feature = "array")]
            (Value::Array(ary_opt), Some("FLOAT8[]")) => match ary_opt {
                Some(ary) => {
                    let mut floats = Vec::with_capacity(ary.len());

                    for val in ary.into_iter().map(|v| v.as_f64()) {
                        match val {
                            Some(float) => {
                                floats.push(float);
                            }
                            None => {
                                let msg = "Non-float parameter when storing a FLOAT8[]";
                                let kind = ErrorKind::conversion(msg);

                                Err(Error::builder(kind).build())?
                            }
                        }
                    }

                    self.bind(floats)
                }
                None => self.bind(Option::<Vec<f64>>::None),
            },

            #[cfg(feature = "array")]
            (Value::Array(ary_opt), Some("NUMERIC[]")) => match ary_opt {
                Some(ary) => {
                    let mut floats = Vec::with_capacity(ary.len());

                    for val in ary.into_iter().map(|v| v.as_decimal()) {
                        match val {
                            Some(float) => {
                                floats.push(float);
                            }
                            None => {
                                let msg = "Non-numeric parameter when storing a NUMERIC[]";
                                let kind = ErrorKind::conversion(msg);

                                Err(Error::builder(kind).build())?
                            }
                        }
                    }

                    self.bind(floats)
                }
                None => self.bind(Option::<Vec<f64>>::None),
            },

            #[cfg(feature = "array")]
            (Value::Array(ary_opt), Some("MONEY[]")) => match ary_opt {
                Some(ary) => {
                    let mut moneys = Vec::with_capacity(ary.len());

                    for val in ary.into_iter().map(|v| v.as_decimal()) {
                        match val {
                            Some(decimal) => moneys.push(PgMoney::from_decimal(decimal, 2)),
                            None => {
                                let msg = "Non-numeric parameter when storing a MONEY[]";
                                let kind = ErrorKind::conversion(msg);

                                Err(Error::builder(kind).build())?
                            }
                        }
                    }

                    self.bind(moneys)
                }
                None => self.bind(Option::<Vec<f64>>::None),
            },

            #[cfg(feature = "array")]
            (Value::Array(ary_opt), Some("BOOL[]")) => match ary_opt {
                Some(ary) => {
                    let mut boos = Vec::with_capacity(ary.len());

                    for val in ary.into_iter().map(|v| v.as_bool()) {
                        match val {
                            Some(boo) => {
                                boos.push(boo);
                            }
                            None => {
                                let msg = "Non-boolean parameter when storing a BOOL[]";
                                let kind = ErrorKind::conversion(msg);

                                Err(Error::builder(kind).build())?
                            }
                        }
                    }

                    self.bind(boos)
                }
                None => self.bind(Option::<Vec<bool>>::None),
            },

            #[cfg(all(feature = "array", feature = "chrono-0_4"))]
            (Value::Array(ary_opt), Some("TIMESTAMPTZ[]")) => match ary_opt {
                Some(ary) => {
                    let mut vals = Vec::with_capacity(ary.len());

                    for val in ary.into_iter().map(|v| v.as_datetime()) {
                        match val {
                            Some(val) => {
                                vals.push(val);
                            }
                            None => {
                                let msg = "Non-datetime parameter when storing a TIMESTAMPTZ[]";
                                let kind = ErrorKind::conversion(msg);

                                Err(Error::builder(kind).build())?
                            }
                        }
                    }

                    self.bind(vals)
                }
                None => self.bind(Option::<Vec<chrono::DateTime<chrono::Utc>>>::None),
            },

            #[cfg(all(feature = "array", feature = "chrono-0_4"))]
            (Value::Array(ary_opt), Some("TIMESTAMP[]")) => match ary_opt {
                Some(ary) => {
                    let mut vals = Vec::with_capacity(ary.len());

                    for val in ary.into_iter().map(|v| v.as_datetime()) {
                        match val {
                            Some(val) => {
                                vals.push(val.naive_utc());
                            }
                            None => {
                                let msg = "Non-datetime parameter when storing a TIMESTAMP[]";
                                let kind = ErrorKind::conversion(msg);

                                Err(Error::builder(kind).build())?
                            }
                        }
                    }

                    self.bind(vals)
                }
                None => self.bind(Option::<Vec<chrono::NaiveDateTime>>::None),
            },

            #[cfg(all(feature = "array", feature = "chrono-0_4"))]
            (Value::Array(ary_opt), Some("DATE[]")) => match ary_opt {
                Some(ary) => {
                    let mut vals = Vec::with_capacity(ary.len());

                    for val in ary.into_iter().map(|v| v.as_date()) {
                        match val {
                            Some(val) => {
                                vals.push(val);
                            }
                            None => {
                                let msg = "Non-date parameter when storing a DATE[]";
                                let kind = ErrorKind::conversion(msg);

                                Err(Error::builder(kind).build())?
                            }
                        }
                    }

                    self.bind(vals)
                }
                None => self.bind(Option::<Vec<chrono::NaiveDate>>::None),
            },

            #[cfg(all(feature = "array", feature = "chrono-0_4"))]
            (Value::Array(ary_opt), Some("TIME[]")) => match ary_opt {
                Some(ary) => {
                    let mut vals = Vec::with_capacity(ary.len());

                    for val in ary.into_iter().map(|v| v.as_time()) {
                        match val {
                            Some(val) => {
                                vals.push(val);
                            }
                            None => {
                                let msg = "Non-time parameter when storing a TIME[]";
                                let kind = ErrorKind::conversion(msg);

                                Err(Error::builder(kind).build())?
                            }
                        }
                    }

                    self.bind(vals)
                }
                None => self.bind(Option::<Vec<chrono::NaiveTime>>::None),
            },

            #[cfg(all(feature = "array", feature = "chrono-0_4"))]
            (Value::Array(ary_opt), Some("TIMETZ[]")) => match ary_opt {
                Some(ary) => {
                    let mut vals = Vec::with_capacity(ary.len());

                    for val in ary.into_iter().map(|v| v.as_datetime()) {
                        match val {
                            Some(val) => {
                                let timetz = PgTimeTz {
                                    time: val.time(),
                                    offset: chrono::FixedOffset::east(0),
                                };

                                vals.push(timetz);
                            }
                            None => {
                                let msg = "Non-time parameter when storing a TIMETZ[]";
                                let kind = ErrorKind::conversion(msg);

                                Err(Error::builder(kind).build())?
                            }
                        }
                    }

                    self.bind(vals)
                }
                None => self.bind(Option::<Vec<PgTimeTz>>::None),
            },

            #[cfg(all(feature = "array", feature = "json-1"))]
            (Value::Array(ary_opt), t) if t == Some("JSON[]") || t == Some("JSONB[]") => match ary_opt {
                Some(ary) if ary.first().map(|val| val.is_json()).unwrap_or(false) => {
                    let mut vals = Vec::with_capacity(ary.len());

                    for val in ary.into_iter().map(|v| v.into_json()) {
                        match val {
                            Some(val) => {
                                vals.push(Json(val));
                            }
                            None => {
                                let msg = "Non-json parameter when storing a JSON[]";
                                let kind = ErrorKind::conversion(msg);

                                Err(Error::builder(kind).build())?
                            }
                        }
                    }

                    self.bind(vals)
                }
                Some(ary) => {
                    let mut vals = Vec::with_capacity(ary.len());

                    for val in ary.into_iter().map(|v| v.into_string()) {
                        match val {
                            Some(val) => {
                                let json = serde_json::from_str(val.as_str()).map_err(|_| {
                                    let msg = "Non-json parameter when storing a JSON[]";
                                    let kind = ErrorKind::conversion(msg);

                                    Error::builder(kind).build()
                                })?;
                                vals.push(Json(json));
                            }
                            None => {
                                let msg = "Non-json parameter when storing a JSON[]";
                                let kind = ErrorKind::conversion(msg);

                                Err(Error::builder(kind).build())?
                            }
                        }
                    }

                    self.bind(vals)
                }
                None => self.bind(Option::<Vec<Json<serde_json::Value>>>::None),
            },

            #[cfg(feature = "array")]
            (Value::Array(ary_opt), Some("\"CHAR\"[]")) => match ary_opt {
                Some(ary) => {
                    let mut vals = Vec::with_capacity(ary.len());

                    for val in ary.into_iter().map(|v| v.as_char()) {
                        match val {
                            Some(val) => {
                                vals.push(val as i8);
                            }
                            None => {
                                let msg = "Non-char parameter when storing a CHAR[]";
                                let kind = ErrorKind::conversion(msg);

                                Err(Error::builder(kind).build())?
                            }
                        }
                    }

                    self.bind(vals)
                }
                None => self.bind(Option::<Vec<i8>>::None),
            },

            #[cfg(any(feature = "array", feature = "uuid-0_8"))]
            (Value::Array(ary_opt), Some("UUID[]")) => match ary_opt {
                Some(ary) if ary.first().map(|v| v.is_uuid()).unwrap_or(false) => {
                    let mut vals = Vec::with_capacity(ary.len());

                    for val in ary.into_iter().map(|v| v.as_uuid()) {
                        match val {
                            Some(val) => {
                                vals.push(val);
                            }
                            None => {
                                let msg = "Non-uuid parameter when storing a UUID[]";
                                let kind = ErrorKind::conversion(msg);

                                Err(Error::builder(kind).build())?
                            }
                        }
                    }

                    self.bind(vals)
                }
                Some(ary) => {
                    let mut vals = Vec::with_capacity(ary.len());

                    for val in ary.into_iter().map(|v| v.into_string()) {
                        match val {
                            Some(val) => {
                                let id: uuid::Uuid = val.parse().map_err(|_| {
                                    let kind = ErrorKind::conversion(format!(
                                        "The given string '{}' could not be converted to UUID.",
                                        val
                                    ));
                                    Error::builder(kind).build()
                                })?;
                                vals.push(id);
                            }
                            None => {
                                let msg = "Non-uuid parameter when storing a UUID[]";
                                let kind = ErrorKind::conversion(msg);

                                Err(Error::builder(kind).build())?
                            }
                        }
                    }

                    self.bind(vals)
                }
                None => self.bind(Option::<Vec<uuid::Uuid>>::None),
            },

            #[cfg(feature = "array")]
            (Value::Array(ary_opt), t)
                if t == Some("TEXT[]") || t == Some("VARCHAR[]") || t == Some("NAME[]") || t == Some("CHAR[]") =>
            {
                match ary_opt {
                    Some(ary) => {
                        let mut vals = Vec::with_capacity(ary.len());

                        for val in ary.into_iter().map(|v| v.into_string()) {
                            match val {
                                Some(val) => {
                                    vals.push(val);
                                }
                                None => {
                                    let msg = "Non-string parameter when storing a string array";
                                    let kind = ErrorKind::conversion(msg);

                                    Err(Error::builder(kind).build())?
                                }
                            }
                        }

                        self.bind(vals)
                    }
                    None => self.bind(Option::<Vec<String>>::None),
                }
            }

            #[cfg(feature = "array")]
            (Value::Array(ary_opt), t) if t == Some("BYTEA[]") => match ary_opt {
                Some(ary) => {
                    let mut vals = Vec::with_capacity(ary.len());

                    for val in ary.into_iter().map(|v| v.into_bytes()) {
                        match val {
                            Some(val) => {
                                vals.push(val);
                            }
                            None => {
                                let msg = "Non-bytes parameter when storing a bytea array";
                                let kind = ErrorKind::conversion(msg);

                                Err(Error::builder(kind).build())?
                            }
                        }
                    }

                    self.bind(vals)
                }
                None => self.bind(Option::<Vec<String>>::None),
            },

            #[cfg(all(feature = "array", feature = "ipnetwork"))]
            (Value::Array(ary_opt), t) if t == Some("INET[]") || t == Some("CIDR[]") => match ary_opt {
                Some(ary) => {
                    let mut ips = Vec::with_capacity(ary.len());

                    for val in ary.into_iter() {
                        match val.into_string() {
                            Some(s) => {
                                let ip: sqlx::types::ipnetwork::IpNetwork = s.parse().map_err(|_| {
                                    let msg = format!("Provided IP address ({}) not in the right format.", s);
                                    let kind = ErrorKind::conversion(msg);

                                    Error::builder(kind).build()
                                })?;

                                ips.push(ip);
                            }
                            None => {
                                let msg = "Non-string parameter when storing an IP array";
                                let kind = ErrorKind::conversion(msg);

                                Err(Error::builder(kind).build())?
                            }
                        }
                    }

                    self.bind(ips)
                }
                None => self.bind(Option::<Vec<sqlx::types::ipnetwork::IpNetwork>>::None),
            },

            #[cfg(feature = "array")]
            (Value::Array(ary_opt), t) => match t {
                _ if type_info
                    .map(|ti| matches!(ti.kind(), PgTypeKind::Array(ti) if matches!(ti.kind(), PgTypeKind::Enum(_))))
                    .unwrap_or(false) =>
                {
                    match ary_opt {
                        Some(ary) => {
                            let mut vals = Vec::with_capacity(ary.len());

                            for val in ary.into_iter().map(|v| v.into_string()) {
                                match val {
                                    Some(val) => {
                                        vals.push(val);
                                    }
                                    None => {
                                        let msg = "Non-string parameter when storing a string array";
                                        let kind = ErrorKind::conversion(msg);

                                        Err(Error::builder(kind).build())?
                                    }
                                }
                            }

                            self.bind(vals)
                        }
                        None => self.bind(Option::<Vec<String>>::None),
                    }
                }
                Some(t) => {
                    let msg = format!("Postgres type {} not supported yet", t);
                    let kind = ErrorKind::conversion(msg);

                    Err(Error::builder(kind).build())?
                }
                None => {
                    let kind = ErrorKind::conversion("Untyped Postgres arrays are not supported");
                    Err(Error::builder(kind).build())?
                }
            },

            #[cfg(feature = "json-1")]
            (Value::Json(json), _) => self.bind(json.map(Json)),

            #[cfg(feature = "uuid-0_8")]
            (Value::Uuid(uuid), _) => self.bind(uuid),

            #[cfg(feature = "chrono-0_4")]
            (Value::DateTime(dt), Some("TIMETZ")) => {
                let time_tz = dt.map(|dt| PgTimeTz {
                    time: dt.time(),
                    offset: chrono::FixedOffset::east(0),
                });

                self.bind(time_tz)
            }

            #[cfg(feature = "chrono-0_4")]
            (Value::DateTime(dt), Some("TIME")) => self.bind(dt.map(|dt| dt.time())),

            #[cfg(feature = "chrono-0_4")]
            (Value::DateTime(dt), Some("DATE")) => self.bind(dt.map(|dt| dt.date().naive_utc())),

            #[cfg(feature = "chrono-0_4")]
            (Value::DateTime(dt), _) => self.bind(dt),

            #[cfg(feature = "chrono-0_4")]
            (Value::Date(date), _) => self.bind(date),

            #[cfg(feature = "chrono-0_4")]
            (Value::Time(time), _) => self.bind(time),
        };

        Ok(query)
    }
}

pub fn map_row<'a>(row: PgRow) -> Result<Vec<Value<'a>>, sqlx::Error> {
    let mut result = Vec::with_capacity(row.len());

    for i in 0..row.len() {
        let type_info = row.columns()[i].type_info();

        let value = match type_info.name() {
            // Singular types from here down, arrays after these.
            "\"CHAR\"" => {
                let int_opt: Option<i8> = row.get_unchecked(i);
                Value::Char(int_opt.map(|i| (i as u8) as char))
            }

            "INT2" => {
                let int_opt: Option<i16> = row.get_unchecked(i);
                Value::Integer(int_opt.map(|i| i as i64))
            }

            "INT4" => {
                let int_opt: Option<i32> = row.get_unchecked(i);
                Value::Integer(int_opt.map(|i| i as i64))
            }

            "INT8" => Value::Integer(row.get_unchecked(i)),

            "OID" => {
                let int_opt: Option<u32> = row.get_unchecked(i);
                Value::Integer(int_opt.map(|i| i as i64))
            }

            "MONEY" => {
                let money_opt: Option<PgMoney> = row.get_unchecked(i);

                // We assume the default setting of 2 decimals.
                let decimal_opt = money_opt.map(|money| money.to_decimal(2));

                Value::Real(decimal_opt)
            }

            "NUMERIC" => Value::Real(row.get_unchecked(i)),

            "FLOAT4" => {
                let f_opt: Option<f32> = row.get_unchecked(i);
                Value::Real(f_opt.map(|f| Decimal::from_f32(f).unwrap()))
            }

            "FLOAT8" => {
                let f_opt: Option<f64> = row.get_unchecked(i);
                Value::Real(f_opt.map(|f| Decimal::from_f64(f).unwrap()))
            }

            "TEXT" | "VARCHAR" | "NAME" | "CHAR" => {
                let string_opt: Option<String> = row.get_unchecked(i);
                Value::Text(string_opt.map(Cow::from))
            }

            "BYTEA" => {
                let bytes_opt: Option<Vec<u8>> = row.get_unchecked(i);
                Value::Bytes(bytes_opt.map(Cow::from))
            }

            "BOOL" => Value::Boolean(row.get_unchecked(i)),

            "INET" | "CIDR" => {
                let ip_opt: Option<sqlx::types::ipnetwork::IpNetwork> = row.get_unchecked(i);
                Value::Text(ip_opt.map(|ip| format!("{}", ip)).map(Cow::from))
            }

            #[cfg(feature = "uuid-0_8")]
            "UUID" => Value::Uuid(row.get_unchecked(i)),

            #[cfg(feature = "chrono-0_4")]
            "TIMESTAMPTZ" => Value::DateTime(row.get_unchecked(i)),

            #[cfg(feature = "chrono-0_4")]
            "DATE" => Value::Date(row.get_unchecked(i)),

            #[cfg(feature = "chrono-0_4")]
            "TIME" => Value::Time(row.get_unchecked(i)),

            #[cfg(all(feature = "chrono-0_4", feature = "array"))]
            "TIMESTAMP" => {
                let naive: Option<chrono::NaiveDateTime> = row.get_unchecked(i);
                let dt = naive.map(|d| chrono::DateTime::<chrono::Utc>::from_utc(d, chrono::Utc));
                Value::DateTime(dt)
            }

            #[cfg(feature = "chrono-0_4")]
            "TIMETZ" => {
                let timetz_opt: Option<PgTimeTz> = row.get_unchecked(i);

                let dt_opt = timetz_opt.map(|time_tz| {
                    let dt = chrono::NaiveDate::from_ymd(1970, 1, 1).and_time(time_tz.time);
                    let dt = chrono::DateTime::<chrono::Utc>::from_utc(dt, chrono::Utc);
                    let dt = dt.with_timezone(&time_tz.offset);

                    chrono::DateTime::from_utc(dt.naive_utc(), chrono::Utc)
                });

                Value::DateTime(dt_opt)
            }

            #[cfg(feature = "json-1")]
            "JSON" | "JSONB" => Value::Json(row.get_unchecked(i)),

            #[cfg(feature = "bit-vec")]
            "BIT" | "VARBIT" => {
                let bit_opt: Option<bit_vec::BitVec> = row.get_unchecked(i);
                Value::Text(bit_opt.map(bits_to_string).map(Cow::from))
            }

            // arrays from here on
            #[cfg(feature = "array")]
            "\"CHAR\"[]" => {
                let ary_opt: Option<Vec<i8>> = row.get_unchecked(i);

                let chars = ary_opt.map(|ary| {
                    ary.into_iter()
                        .map(|i| (i as u8) as char)
                        .map(Value::character)
                        .collect()
                });

                Value::Array(chars)
            }

            #[cfg(feature = "array")]
            "INT2[]" => {
                let ary_opt: Option<Vec<i16>> = row.get_unchecked(i);
                Value::Array(ary_opt.map(|ary| ary.into_iter().map(Value::integer).collect()))
            }

            #[cfg(feature = "array")]
            "INT4[]" => {
                let ary_opt: Option<Vec<i32>> = row.get_unchecked(i);
                Value::Array(ary_opt.map(|ary| ary.into_iter().map(Value::integer).collect()))
            }

            #[cfg(feature = "array")]
            "INT8[]" => {
                let ary_opt: Option<Vec<i64>> = row.get_unchecked(i);
                Value::Array(ary_opt.map(|ary| ary.into_iter().map(Value::integer).collect()))
            }

            #[cfg(feature = "array")]
            "OID[]" => {
                let ary_opt: Option<Vec<u32>> = row.get_unchecked(i);
                Value::Array(ary_opt.map(|ary| ary.into_iter().map(Value::integer).collect()))
            }

            #[cfg(feature = "array")]
            "MONEY[]" => {
                let ary_opt: Option<Vec<PgMoney>> = row.get_unchecked(i);

                // We assume the default setting of 2 decimals.
                let decs = ary_opt.map(|ary| {
                    ary.into_iter()
                        .map(|money| money.to_decimal(2))
                        .map(Value::real)
                        .collect()
                });

                Value::Array(decs)
            }

            #[cfg(feature = "array")]
            "NUMERIC[]" => {
                let ary_opt: Option<Vec<Decimal>> = row.get_unchecked(i);
                let decs = ary_opt.map(|ary| ary.into_iter().map(Value::real).collect());

                Value::Array(decs)
            }

            #[cfg(feature = "array")]
            "FLOAT4[]" => {
                let ary_opt: Option<Vec<f32>> = row.get_unchecked(i);

                let decs = ary_opt.map(|ary| {
                    ary.into_iter()
                        .map(|f| Decimal::from_f32(f).unwrap())
                        .map(Value::real)
                        .collect()
                });

                Value::Array(decs)
            }

            #[cfg(feature = "array")]
            "FLOAT8[]" => {
                let ary_opt: Option<Vec<f64>> = row.get_unchecked(i);

                let decs = ary_opt.map(|ary| {
                    ary.into_iter()
                        .map(|f| Decimal::from_f64(f).unwrap())
                        .map(Value::real)
                        .collect()
                });

                Value::Array(decs)
            }

            #[cfg(feature = "array")]
            "TEXT[]" | "VARCHAR[]" | "NAME[]" | "CHAR[]" => {
                let ary_opt: Option<Vec<String>> = row.get_unchecked(i);
                Value::Array(ary_opt.map(|ary| ary.into_iter().map(Value::text).collect()))
            }

            #[cfg(feature = "array")]
            "BOOL[]" => {
                let ary_opt: Option<Vec<bool>> = row.get_unchecked(i);
                Value::Array(ary_opt.map(|ary| ary.into_iter().map(Value::boolean).collect()))
            }

            #[cfg(feature = "array")]
            "CIDR[]" | "INET[]" => {
                let ary_opt: Option<Vec<sqlx::types::ipnetwork::IpNetwork>> = row.get_unchecked(i);
                let strs = ary_opt.map(|ary| ary.into_iter().map(|ip| Value::text(format!("{}", ip))).collect());

                Value::Array(strs)
            }

            #[cfg(feature = "array")]
            "BYTEA[]" => {
                let ary_opt: Option<Vec<Vec<u8>>> = row.get_unchecked(i);
                let bytes = ary_opt.map(|ary| ary.into_iter().map(Value::bytes).collect());

                Value::Array(bytes)
            }

            #[cfg(all(feature = "chrono-0_4", feature = "array"))]
            "TIMESTAMPTZ[]" => {
                let ary_opt: Option<Vec<chrono::DateTime<chrono::Utc>>> = row.get_unchecked(i);
                Value::Array(ary_opt.map(|ary| ary.into_iter().map(Value::datetime).collect()))
            }

            #[cfg(all(feature = "chrono-0_4", feature = "array"))]
            "DATE[]" => {
                let ary_opt: Option<Vec<chrono::NaiveDate>> = row.get_unchecked(i);
                Value::Array(ary_opt.map(|ary| ary.into_iter().map(Value::date).collect()))
            }

            #[cfg(all(feature = "chrono-0_4", feature = "array"))]
            "TIMESTAMP[]" => {
                let ary_opt: Option<Vec<chrono::NaiveDateTime>> = row.get_unchecked(i);

                Value::Array(ary_opt.map(|ary| {
                    ary.into_iter()
                        .map(|d| chrono::DateTime::<chrono::Utc>::from_utc(d, chrono::Utc))
                        .map(Value::datetime)
                        .collect()
                }))
            }

            #[cfg(all(feature = "chrono-0_4", feature = "array"))]
            "TIME[]" => {
                let ary_opt: Option<Vec<chrono::NaiveTime>> = row.get_unchecked(i);
                Value::Array(ary_opt.map(|ary| ary.into_iter().map(Value::time).collect()))
            }

            #[cfg(all(feature = "chrono-0_4", feature = "array"))]
            "TIMETZ[]" => {
                let ary_opt: Option<Vec<PgTimeTz>> = row.get_unchecked(i);

                let dts = ary_opt.map(|ary| {
                    ary.into_iter()
                        .map(|time_tz| {
                            let dt = chrono::NaiveDate::from_ymd(1970, 1, 1).and_time(time_tz.time);
                            let dt = chrono::DateTime::<chrono::Utc>::from_utc(dt, chrono::Utc);
                            let dt = dt.with_timezone(&time_tz.offset);

                            chrono::DateTime::from_utc(dt.naive_utc(), chrono::Utc)
                        })
                        .map(Value::datetime)
                        .collect()
                });

                Value::Array(dts)
            }

            #[cfg(all(feature = "json-1", feature = "array"))]
            "JSON[]" | "JSONB[]" => {
                let ary_opt: Option<Vec<Json<serde_json::Value>>> = row.get_unchecked(i);
                let jsons = ary_opt.map(|ary| ary.into_iter().map(|j| Value::json(j.0)).collect());

                Value::Array(jsons)
            }

            #[cfg(all(feature = "bit-vec", feature = "array"))]
            "BIT[]" | "VARBIT[]" => {
                let ary_opt: Option<Vec<bit_vec::BitVec>> = row.get_unchecked(i);
                let strs = ary_opt.map(|ary| ary.into_iter().map(bits_to_string).map(Value::text).collect());

                Value::Array(strs)
            }

            #[cfg(all(feature = "uuid-0_8", feature = "array"))]
            "UUID[]" => {
                let ary_opt: Option<Vec<uuid::Uuid>> = row.get_unchecked(i);
                let uuids = ary_opt.map(|ary| ary.into_iter().map(Value::uuid).collect());

                Value::Array(uuids)
            }

            name => match type_info {
                ti if matches!(ti.kind(), PgTypeKind::Enum(_)) => {
                    let string_opt: Option<String> = row.get_unchecked(i);
                    Value::Enum(string_opt.map(Cow::from))
                }
                ti if matches!(ti.kind(), PgTypeKind::Array(ti) if matches!(ti.kind(), PgTypeKind::Enum(_))) => {
                    let ary_opt: Option<Vec<String>> = row.get_unchecked(i);
                    Value::Array(ary_opt.map(|ary| ary.into_iter().map(Value::enum_variant).collect()))
                }
                _ => {
                    let msg = format!("Type {} is not yet supported in the PostgreSQL connector.", name);
                    let kind = ErrorKind::conversion(msg.clone());

                    let mut builder = Error::builder(kind);
                    builder.set_original_message(msg);

                    let error = sqlx::Error::ColumnDecode {
                        index: format!("{}", i),
                        source: Box::new(builder.build()),
                    };

                    Err(error)?
                }
            },
        };

        result.push(value);
    }

    Ok(result)
}

#[cfg(feature = "bit-vec")]
fn bits_to_string(bits: bit_vec::BitVec) -> String {
    let mut s = String::with_capacity(bits.len());

    for bit in bits {
        if bit {
            s.push('1');
        } else {
            s.push('0');
        }
    }

    s
}

#[cfg(feature = "bit-vec")]
fn string_to_bits(s: &str) -> crate::Result<bit_vec::BitVec> {
    let mut bits = bit_vec::BitVec::with_capacity(s.len());

    for c in s.chars() {
        match c {
            '0' => bits.push(false),
            '1' => bits.push(true),
            _ => {
                let msg = "Unexpected character for bits input. Expected only 1 and 0.";
                let kind = ErrorKind::conversion(msg);

                Err(Error::builder(kind).build())?
            }
        }
    }

    Ok(bits)
}
