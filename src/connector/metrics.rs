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

    let result = match res {
        Ok(_) => "success",
        Err(_) => "error",
    };

    tracing::trace!(
        query,
        item_type = "query",
        is_query = true,
        params = %Params(params),
        duration_ms = start.elapsed().as_millis() as u64,
        result,
    );

    histogram!(format!("{}.query.time", tag), start.elapsed());

    res
}

#[cfg(feature = "pooled")]
pub(crate) async fn check_out<F, T>(f: F) -> std::result::Result<T, mobc::Error<crate::error::Error>>
where
    F: Future<Output = std::result::Result<T, mobc::Error<crate::error::Error>>>,
{
    let start = Instant::now();
    let res = f.await;

    let result = match res {
        Ok(_) => "success",
        Err(_) => "error",
    };

    tracing::trace!(
        message = "Fetched a connection from the pool",
        duration_ms = start.elapsed().as_millis() as u64,
        item_type = "query",
        is_query = true,
        result,
    );

    histogram!("pool.check_out", start.elapsed());

    res
}
