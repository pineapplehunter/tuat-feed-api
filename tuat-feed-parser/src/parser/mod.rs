use thiserror::Error;

mod info_parser;
mod main_page_parser;
pub use info_parser::info_parser;
pub use main_page_parser::main_page_parser;

/// the error that happens when parseing the web page
#[derive(Error, Debug)]
pub enum ParseError {
    /// an error that happens when scraping
    #[error("scraping error")]
    ScrappingError(String),
    /// when parsing an invalid int
    #[error("int parse error")]
    IntParseError(#[from] std::num::ParseIntError),
}
