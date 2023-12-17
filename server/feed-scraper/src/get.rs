use std::{future::Future, time::Duration};

use thiserror::Error;

/// the error that happens when accessing the internet
#[derive(Error, Debug)]
pub enum GetError {
    /// when there is a problem regarding networking
    #[error("internet connention error")]
    ConnectionError(#[from] reqwest::Error),
    /// when it can't get the text
    #[error("invalid text on page")]
    InvalidTextError,
}

#[tracing::instrument(skip(fut))]
async fn retry<T, E, F>(fut: impl Fn() -> F) -> Result<T, E>
where
    F: Future<Output = Result<T, E>>,
{
    let mut result = fut().await;
    for _ in 0..5 {
        if result.is_ok() {
            break;
        }
        tokio::time::sleep(Duration::from_secs(5)).await;
        result = fut().await;
    }
    result
}

/// does the actual getting from the internet part
#[tracing::instrument]
pub async fn get(feed_url: &str) -> Result<String, GetError> {
    retry(|| async {
        reqwest::get(feed_url)
            .await
            .map_err(GetError::ConnectionError)?
            .text()
            .await
            .map_err(|_e| GetError::InvalidTextError)
    })
    .await
}
