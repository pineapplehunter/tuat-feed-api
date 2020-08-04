#![warn(missing_docs)]

//! # tuat-feed-parser
//! this crate provides a api to access the tuat feed as a struct.

use futures::stream::{self, StreamExt};
use thiserror::Error;

mod get;
mod info;
mod parser;

pub use get::{get, GetError};
pub use info::Info;
use parser::{info_parser, main_page_parser, ParseError};

use log::info;

const CAMPUS_FEED_URL: &str =
    "http://t-board.office.tuat.ac.jp/T/boar/resAjax.php?bAnno=0&par=20&skip=0";
const ACADEMIC_FEED_URL: &str =
    "http://t-board.office.tuat.ac.jp/T/boar/resAjax.php?bAnno=1&par=20&skip=0";
static INFO_URL_BASE: &str = "http://t-board.office.tuat.ac.jp/T/boar/vewAjax.php?i=";

const BUFFERED_NUM: usize = 10;

/// Any Error That may happen in this library
#[derive(Error, Debug)]
pub enum TuatFeedParserError {
    /// error in parsing
    #[error("parse error")]
    ParseError(#[from] ParseError),
    /// error in getting
    #[error("get error")]
    GetError(#[from] GetError),
}

/// get data from the campus feed
pub async fn get_campus_feed() -> Result<Vec<Info>, TuatFeedParserError> {
    info!("fetching campus feed");
    let content = get(CAMPUS_FEED_URL).await?;
    let ids = main_page_parser(&content).await?;

    let informations: Vec<Info> = stream::iter(ids)
        .map(|id| async move { (get(&format!("{}{}", INFO_URL_BASE, id)).await, id) })
        .buffered(BUFFERED_NUM)
        .filter_map(|(content, id)| async move {
            if let Ok(content) = content {
                Some((content, id))
            } else {
                None
            }
        })
        .map(|(content, id)| async move { info_parser(&content, id).await })
        .buffered(BUFFERED_NUM)
        .filter_map(|info| async move { info.ok() })
        .collect()
        .await;

    Ok(informations)
}

/// get data from the academic feed
pub async fn get_academic_feed() -> Result<Vec<Info>, TuatFeedParserError> {
    info!("fetching academic feed");
    let content = get(ACADEMIC_FEED_URL).await?;
    let ids = main_page_parser(&content).await?;

    let informations: Vec<Info> = stream::iter(ids)
        .map(|id| async move { (get(&format!("{}{}", INFO_URL_BASE, id)).await, id) })
        .buffered(BUFFERED_NUM)
        .filter_map(|(content, id)| async move {
            if let Ok(content) = content {
                Some((content, id))
            } else {
                None
            }
        })
        .map(|(content, id)| async move { info_parser(&content, id).await })
        .buffered(BUFFERED_NUM)
        .filter_map(|info| async move { info.ok() })
        .collect()
        .await;

    Ok(informations)
}
