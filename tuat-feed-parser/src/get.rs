use thiserror::Error;

/// the error that happens when accessing the internet
#[derive(Error, Debug)]
pub enum GetError {
    /// wehn there is a problem regarding networking
    #[error("internet connention error")]
    ConnectionError(String),
    /// when it can't get the text
    #[error("invalid text on page")]
    InvalidTextError,
}

/// does the actual getting from the internet part
pub async fn get(feed_url: &str) -> Result<String, GetError> {
    let client = surf::client().with(surf::middleware::Redirect::default());
    let content = client
        .get(feed_url)
        .recv_string()
        .await
        .map_err(|_e| GetError::InvalidTextError)?;

    Ok(content)
}
