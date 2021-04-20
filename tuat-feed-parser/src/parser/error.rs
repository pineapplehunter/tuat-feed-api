use thiserror::Error;

/// the error that happens when parseing the web page
#[derive(Error, Debug)]
pub enum ParseError {
    /// an error that happens when scraping
    #[error("scraping error")]
    ScrapingError(String),
    /// when parsing an invalid int
    #[error("int parse error")]
    IntParseError(#[from] std::num::ParseIntError),
}
