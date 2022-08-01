mod aggregate_to_string;
#[cfg(feature = "postgresql")]
mod all_operator;
#[cfg(feature = "postgresql")]
mod any_operator;
mod average;
mod coalesce;
mod concat;
mod count;
#[cfg(all(feature = "json", any(feature = "postgresql", feature = "mysql")))]
mod json_extract;
#[cfg(all(feature = "json", any(feature = "postgresql", feature = "mysql")))]
mod json_extract_array;
#[cfg(all(feature = "json", any(feature = "postgresql", feature = "mysql")))]
mod json_unquote;
mod lower;
mod maximum;
mod minimum;
mod row_number;
#[cfg(all(feature = "json", feature = "postgresql"))]
mod row_to_json;
#[cfg(any(feature = "postgresql", feature = "mysql"))]
mod search;
mod sum;
mod upper;

#[cfg(feature = "mysql")]
mod uuid;

pub use aggregate_to_string::*;
#[cfg(feature = "postgresql")]
pub use all_operator::*;
#[cfg(feature = "postgresql")]
pub use any_operator::*;
pub use average::*;
pub use coalesce::*;
pub use concat::*;
pub use count::*;
#[cfg(all(feature = "json", any(feature = "postgresql", feature = "mysql")))]
pub use json_extract::*;
#[cfg(all(feature = "json", any(feature = "postgresql", feature = "mysql")))]
pub(crate) use json_extract_array::*;
#[cfg(all(feature = "json", any(feature = "postgresql", feature = "mysql")))]
pub use json_unquote::*;
pub use lower::*;
pub use maximum::*;
pub use minimum::*;
pub use row_number::*;
#[cfg(all(feature = "json", feature = "postgresql"))]
pub use row_to_json::*;
#[cfg(any(feature = "postgresql", feature = "mysql"))]
pub use search::*;
pub use sum::*;
pub use upper::*;

#[cfg(feature = "mysql")]
pub use self::uuid::*;

use super::{Aliasable, Expression};
use std::borrow::Cow;

/// A database function definition
#[derive(Debug, Clone, PartialEq)]
pub struct Function<'a> {
    pub(crate) typ_: FunctionType<'a>,
    pub(crate) alias: Option<Cow<'a, str>>,
}

/// A database function type
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum FunctionType<'a> {
    #[cfg(all(feature = "json", feature = "postgresql"))]
    RowToJson(RowToJson<'a>),
    RowNumber(RowNumber<'a>),
    Count(Count<'a>),
    AggregateToString(AggregateToString<'a>),
    Average(Average<'a>),
    Sum(Sum<'a>),
    Lower(Lower<'a>),
    Upper(Upper<'a>),
    Minimum(Minimum<'a>),
    Maximum(Maximum<'a>),
    Coalesce(Coalesce<'a>),
    #[cfg(all(feature = "json", any(feature = "postgresql", feature = "mysql")))]
    JsonExtract(JsonExtract<'a>),
    #[cfg(all(feature = "json", any(feature = "postgresql", feature = "mysql")))]
    JsonExtractLastArrayElem(JsonExtractLastArrayElem<'a>),
    #[cfg(all(feature = "json", any(feature = "postgresql", feature = "mysql")))]
    JsonExtractFirstArrayElem(JsonExtractFirstArrayElem<'a>),
    #[cfg(all(feature = "json", any(feature = "postgresql", feature = "mysql")))]
    JsonUnquote(JsonUnquote<'a>),
    #[cfg(any(feature = "postgresql", feature = "mysql"))]
    TextSearch(TextSearch<'a>),
    #[cfg(any(feature = "postgresql", feature = "mysql"))]
    TextSearchRelevance(TextSearchRelevance<'a>),
    #[cfg(feature = "mysql")]
    UuidToBin,
    #[cfg(feature = "mysql")]
    UuidToBinSwapped,
    #[cfg(feature = "mysql")]
    Uuid,
    #[cfg(feature = "postgresql")]
    AnyOperator(AnyOperator<'a>),
    #[cfg(feature = "postgresql")]
    AllOperator(AllOperator<'a>),
    Concat(Concat<'a>),
}

impl<'a> Aliasable<'a> for Function<'a> {
    type Target = Function<'a>;

    fn alias<T>(mut self, alias: T) -> Self::Target
    where
        T: Into<Cow<'a, str>>,
    {
        self.alias = Some(alias.into());
        self
    }
}

#[cfg(all(feature = "json", feature = "postgresql"))]
function!(RowToJson);

#[cfg(all(feature = "json", any(feature = "postgresql", feature = "mysql")))]
function!(JsonExtract);

#[cfg(all(feature = "json", any(feature = "postgresql", feature = "mysql")))]
function!(JsonExtractLastArrayElem);

#[cfg(all(feature = "json", any(feature = "postgresql", feature = "mysql")))]
function!(JsonExtractFirstArrayElem);

#[cfg(all(feature = "json", any(feature = "postgresql", feature = "mysql")))]
function!(JsonUnquote);

#[cfg(any(feature = "postgresql", feature = "mysql"))]
function!(TextSearch);

#[cfg(any(feature = "postgresql", feature = "mysql"))]
function!(TextSearchRelevance);

#[cfg(feature = "postgresql")]
function!(AnyOperator);

#[cfg(feature = "postgresql")]
function!(AllOperator);

function!(
    RowNumber,
    Count,
    AggregateToString,
    Average,
    Sum,
    Lower,
    Upper,
    Minimum,
    Maximum,
    Coalesce,
    Concat
);
