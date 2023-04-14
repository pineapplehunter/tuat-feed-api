use backoff::future::retry;
use backoff::ExponentialBackoff;
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

/// does the actual getting from the internet part
#[tracing::instrument]
pub async fn get(feed_url: &str) -> Result<String, GetError> {
    retry(ExponentialBackoff::default(), || async {
        Ok(reqwest::get(feed_url)
            .await
            .map_err(GetError::ConnectionError)?
            .text()
            .await
            .map_err(|_e| GetError::InvalidTextError)?)
    })
    .await
}
