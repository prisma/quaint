use crate::ast::{Params, Value};
use std::{future::Future, time::Instant};

pub(crate) async fn query<'a, F, T, U>(
    tag: &'static str,
    query: &'a str,
    params: &'a [Value<'_>],
    f: F,
) -> crate::Result<T>
where
    F: FnOnce() -> U + 'a,
    U: Future<Output = crate::Result<T>>,
{
    let start = Instant::now();
    let res = f().await;
    let end = Instant::now();

    if *crate::LOG_QUERIES {
        #[cfg(not(feature = "tracing-log"))]
        {
            info!(
                "query: \"{}\", params: {} (in {}ms)",
                query,
                Params(params),
                start.elapsed().as_millis(),
            );
        }
        #[cfg(feature = "tracing-log")]
        {
            tracing::info!(
                query,
                item_type = "query",
                params = %Params(params),
                duration_ms = start.elapsed().as_millis() as u64,
            )
        }
    }

    timing!(format!("{}.query.time", tag), start, end);

    res
}

pub(crate) async fn query_new<'a, F, T, U>(
    tag: &'static str,
    query: &'a str,
    params: Vec<Value<'a>>,
    f: F,
) -> crate::Result<T>
where
    F: FnOnce(Vec<Value<'a>>) -> U + 'a,
    U: Future<Output = crate::Result<T>> + 'a,
{
    if *crate::LOG_QUERIES {
        let start = Instant::now();
        let res = f(params.clone()).await;
        let end = Instant::now();

        #[cfg(not(feature = "tracing-log"))]
        {
            info!(
                "query: \"{}\", params: {} (in {}ms)",
                query,
                Params(&params),
                start.elapsed().as_millis(),
            );
        }
        #[cfg(feature = "tracing-log")]
        {
            tracing::info!(
                query,
                item_type = "query",
                params = %Params(&params),
                duration_ms = start.elapsed().as_millis() as u64,
            )
        }

        timing!(format!("{}.query.time", tag), start, end);

        res
    } else {
        f(params).await
    }
}
