use super::*;

pub trait FromValue<'a>
where
    Self: Sized + 'a,
{
    fn from_value_ref(value: &Value<'static>) -> crate::Result<Option<Self>>;

    fn from_value(value: Value<'static>) -> crate::Result<Option<Self>> {
        Self::from_value_ref(&value)
    }
}

impl<'a> FromValue<'a> for bool {
    fn from_value_ref(value: &'a Value<'static>) -> crate::Result<Option<Self>> {
        match value {
            Value::Boolean(b) => Ok(*b),
            val => {
                let kind = ErrorKind::conversion(format!("Could not convert {:?} into a bool.", val));
                Err(Error::builder(kind).build())
            }
        }
    }
}

impl<'a> FromValue<'a> for char {
    fn from_value_ref(value: &'a Value<'static>) -> crate::Result<Option<Self>> {
        match value {
            Value::Char(c) => Ok(*c),
            val => {
                let kind = ErrorKind::conversion(format!("Could not convert {:?} into a bool.", val));
                Err(Error::builder(kind).build())
            }
        }
    }
}

impl<'a> FromValue<'a> for i64 {
    fn from_value_ref(value: &'a Value<'static>) -> crate::Result<Option<Self>> {
        match value {
            Value::Integer(int) => Ok(*int),
            val => {
                let kind = ErrorKind::conversion(format!("Could not convert {:?} into an i64.", val));
                Err(Error::builder(kind).build())
            }
        }
    }
}

impl<'a> FromValue<'a> for i32 {
    fn from_value_ref(value: &'a Value<'static>) -> crate::Result<Option<Self>> {
        match value {
            Value::Integer(Some(int)) => Ok(Some(i32::try_from(*int)?)),
            Value::Integer(None) => Ok(None),
            val => {
                let kind = ErrorKind::conversion(format!("Could not convert {:?} into an i32.", val));
                Err(Error::builder(kind).build())
            }
        }
    }
}

impl<'a> FromValue<'a> for i16 {
    fn from_value_ref(value: &'a Value<'static>) -> crate::Result<Option<Self>> {
        match value {
            Value::Integer(Some(int)) => Ok(Some(i16::try_from(*int)?)),
            Value::Integer(None) => Ok(None),
            val => {
                let kind = ErrorKind::conversion(format!("Could not convert {:?} into an i16.", val));
                Err(Error::builder(kind).build())
            }
        }
    }
}

impl<'a> FromValue<'a> for i8 {
    fn from_value_ref(value: &'a Value<'static>) -> crate::Result<Option<Self>> {
        match value {
            Value::Integer(Some(int)) => Ok(Some(i8::try_from(*int)?)),
            Value::Integer(None) => Ok(None),
            val => {
                let kind = ErrorKind::conversion(format!("Could not convert {:?} into an i8.", val));
                Err(Error::builder(kind).build())
            }
        }
    }
}

impl<'a> FromValue<'a> for u64 {
    fn from_value_ref(value: &'a Value<'static>) -> crate::Result<Option<Self>> {
        match value {
            Value::Integer(Some(int)) => Ok(Some(u64::try_from(*int)?)),
            Value::Integer(None) => Ok(None),
            val => {
                let kind = ErrorKind::conversion(format!("Could not convert {:?} into an u64.", val));
                Err(Error::builder(kind).build())
            }
        }
    }
}

impl<'a> FromValue<'a> for u32 {
    fn from_value_ref(value: &'a Value<'static>) -> crate::Result<Option<Self>> {
        match value {
            Value::Integer(Some(int)) => Ok(Some(u32::try_from(*int)?)),
            Value::Integer(None) => Ok(None),
            val => {
                let kind = ErrorKind::conversion(format!("Could not convert {:?} into an u32.", val));
                Err(Error::builder(kind).build())
            }
        }
    }
}

impl<'a> FromValue<'a> for u16 {
    fn from_value_ref(value: &'a Value<'static>) -> crate::Result<Option<Self>> {
        match value {
            Value::Integer(Some(int)) => Ok(Some(u16::try_from(*int)?)),
            Value::Integer(None) => Ok(None),
            val => {
                let kind = ErrorKind::conversion(format!("Could not convert {:?} into an u16.", val));
                Err(Error::builder(kind).build())
            }
        }
    }
}

impl<'a> FromValue<'a> for f32 {
    fn from_value_ref(value: &'a Value<'static>) -> crate::Result<Option<Self>> {
        match value {
            Value::Float(f) => Ok(*f),
            #[cfg(feature = "bigdecimal")]
            Value::Numeric(Some(f)) => Ok(Some(f.to_f32().unwrap())),
            #[cfg(feature = "bigdecimal")]
            Value::Numeric(None) => Ok(None),
            val => {
                let kind = ErrorKind::conversion(format!("Could not convert {:?} into a f32.", val));
                Err(Error::builder(kind).build())
            }
        }
    }
}

impl<'a> FromValue<'a> for f64 {
    fn from_value_ref(value: &'a Value<'static>) -> crate::Result<Option<Self>> {
        match value {
            Value::Double(f) => Ok(*f),
            Value::Float(Some(f)) => Ok(Some(f64::try_from(*f).unwrap())),
            Value::Float(None) => Ok(None),
            #[cfg(feature = "bigdecimal")]
            Value::Numeric(Some(f)) => Ok(Some(f.to_f64().unwrap())),
            #[cfg(feature = "bigdecimal")]
            Value::Numeric(None) => Ok(None),
            val => {
                let kind = ErrorKind::conversion(format!("Could not convert {:?} into a f64.", val));
                Err(Error::builder(kind).build())
            }
        }
    }
}

#[cfg(feature = "bigdecimal")]
impl<'a> FromValue<'a> for BigDecimal {
    fn from_value_ref(value: &'a Value<'static>) -> crate::Result<Option<Self>> {
        match value {
            Value::Numeric(value) => Ok(value.clone()),
            Value::Double(Some(f)) => Ok(Some(BigDecimal::from_f64(*f).unwrap())),
            Value::Double(None) => Ok(None),
            Value::Float(Some(f)) => Ok(Some(BigDecimal::from_f32(*f).unwrap())),
            Value::Float(None) => Ok(None),
            val => {
                let kind = ErrorKind::conversion(format!("Could not convert {:?} into a f64.", val));
                Err(Error::builder(kind).build())
            }
        }
    }

    fn from_value(value: Value<'static>) -> crate::Result<Option<Self>> {
        match value {
            Value::Numeric(value) => Ok(value),
            _ => Self::from_value_ref(&value),
        }
    }
}

impl<'a> FromValue<'a> for String {
    fn from_value_ref(value: &'a Value<'static>) -> crate::Result<Option<Self>> {
        match value {
            Value::Text(s) => Ok(s.as_ref().map(|s| s.to_string())),
            Value::Enum(s) => Ok(s.as_ref().map(|s| s.to_string())),
            Value::Xml(s) => Ok(s.as_ref().map(|s| s.to_string())),
            Value::Char(c) => Ok(c.as_ref().map(|c| c.to_string())),
            #[cfg(feature = "json")]
            Value::Json(Some(json)) => Ok(Some(serde_json::to_string(json).unwrap())),
            #[cfg(feature = "uuid")]
            Value::Uuid(s) => Ok(s.as_ref().map(|u| u.to_hyphenated().to_string())),
            val => {
                let kind = ErrorKind::conversion(format!("Could not convert {:?} into a String.", val));
                Err(Error::builder(kind).build())
            }
        }
    }

    fn from_value(value: Value<'static>) -> crate::Result<Option<Self>> {
        match value {
            Value::Text(s) => Ok(s.map(|s| s.into_owned())),
            Value::Enum(s) => Ok(s.map(|s| s.into_owned())),
            Value::Xml(s) => Ok(s.map(|s| s.into_owned())),
            _ => Self::from_value_ref(&value),
        }
    }
}

impl<'a> FromValue<'a> for &'a str {
    fn from_value_ref(value: &'a Value<'a>) -> crate::Result<Option<Self>> {
        match value {
            Value::Text(s) => Ok(s.as_ref().map(|s| s.as_ref())),
            Value::Enum(s) => Ok(s.as_ref().map(|s| s.as_ref())),
            Value::Xml(s) => Ok(s.as_ref().map(|s| s.as_ref())),
            val => {
                let kind = ErrorKind::conversion(format!("Could not convert {:?} into a String.", val));
                Err(Error::builder(kind).build())
            }
        }
    }
}

impl<'a> FromValue<'a> for Vec<u8> {
    fn from_value_ref(value: &'a Value<'static>) -> crate::Result<Option<Self>> {
        match value {
            Value::Bytes(b) => Ok(b.as_ref().map(|b| b.to_vec())),
            val => {
                let kind = ErrorKind::conversion(format!("Could not convert {:?} into bytes.", val));
                Err(Error::builder(kind).build())
            }
        }
    }

    fn from_value(value: Value<'static>) -> crate::Result<Option<Self>> {
        match value {
            Value::Bytes(b) => Ok(b.map(|b| b.into_owned())),
            _ => Self::from_value_ref(&value),
        }
    }
}

#[cfg(feature = "json")]
impl<'a> FromValue<'a> for serde_json::Value {
    fn from_value_ref(value: &'a Value<'static>) -> crate::Result<Option<Self>> {
        match value {
            Value::Json(json) => Ok(json.clone()),
            val => {
                let kind = ErrorKind::conversion(format!("Could not convert {:?} into json.", val));
                Err(Error::builder(kind).build())
            }
        }
    }

    fn from_value(value: Value<'static>) -> crate::Result<Option<Self>> {
        match value {
            Value::Json(json) => Ok(json),
            _ => Self::from_value_ref(&value),
        }
    }
}

#[cfg(feature = "uuid")]
impl<'a> FromValue<'a> for uuid::Uuid {
    fn from_value_ref(value: &'a Value<'static>) -> crate::Result<Option<Self>> {
        match value {
            Value::Uuid(uuid) => Ok(*uuid),
            val => {
                let kind = ErrorKind::conversion(format!("Could not convert {:?} into uuid.", val));
                Err(Error::builder(kind).build())
            }
        }
    }
}

#[cfg(feature = "chrono")]
impl<'a> FromValue<'a> for chrono::DateTime<Utc> {
    fn from_value_ref(value: &'a Value<'static>) -> crate::Result<Option<Self>> {
        match value {
            Value::DateTime(dt) => Ok(*dt),
            val => {
                let kind = ErrorKind::conversion(format!("Could not convert {:?} into a DateTime.", val));
                Err(Error::builder(kind).build())
            }
        }
    }
}

#[cfg(feature = "chrono")]
impl<'a> FromValue<'a> for chrono::NaiveDate {
    fn from_value_ref(value: &'a Value<'static>) -> crate::Result<Option<Self>> {
        match value {
            Value::Date(dt) => Ok(*dt),
            val => {
                let kind = ErrorKind::conversion(format!("Could not convert {:?} into a Date.", val));
                Err(Error::builder(kind).build())
            }
        }
    }
}

#[cfg(feature = "chrono")]
impl<'a> FromValue<'a> for chrono::NaiveTime {
    fn from_value_ref(value: &'a Value<'static>) -> crate::Result<Option<Self>> {
        match value {
            Value::Time(dt) => Ok(*dt),
            val => {
                let kind = ErrorKind::conversion(format!("Could not convert {:?} into a Time.", val));
                Err(Error::builder(kind).build())
            }
        }
    }
}

impl<'a, T> FromValue<'a> for Vec<Option<T>>
where
    T: FromValue<'a> + Copy,
{
    fn from_value_ref(value: &'a Value<'static>) -> crate::Result<Option<Self>> {
        match value {
            Value::Array(Some(arr)) => {
                let mut result: Vec<Option<T>> = Vec::with_capacity(arr.len());

                for value in arr.iter() {
                    result.push(T::from_value_ref(value)?)
                }

                Ok(Some(result))
            }
            Value::Array(None) => Ok(None),
            val => {
                let kind = ErrorKind::conversion(format!("Could not convert {:?} into an array.", val));
                Err(Error::builder(kind).build())
            }
        }
    }

    fn from_value(value: Value<'static>) -> crate::Result<Option<Self>> {
        match value {
            Value::Array(Some(arr)) => {
                let mut result: Vec<Option<T>> = Vec::with_capacity(arr.len());

                for value in arr.into_iter() {
                    result.push(T::from_value(value)?)
                }

                Ok(Some(result))
            }
            Value::Array(None) => Ok(None),
            val => {
                let kind = ErrorKind::conversion(format!("Could not convert {:?} into an array.", val));
                Err(Error::builder(kind).build())
            }
        }
    }
}
