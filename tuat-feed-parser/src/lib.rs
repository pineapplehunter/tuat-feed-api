#![warn(missing_docs)]

//! # tuat-feed-parser
//! this crate provides a api to access the tuat feed as a struct.

use std::{collections::HashMap, time::Duration};

use thiserror::Error;

mod get;
mod info;
mod parser;

pub use get::{get, GetError};
pub use info::Info;
use parser::{error::ParseError, info_parser, main_page_parser};

use log::info;

/// campas feed url
pub const CAMPUS_FEED_URL: &str =
    "http://t-board.office.tuat.ac.jp/T/boar/resAjax.php?bAnno=0&par=20&skip=0";
/// academic feed url
pub const ACADEMIC_FEED_URL: &str =
    "http://t-board.office.tuat.ac.jp/T/boar/resAjax.php?bAnno=1&par=20&skip=0";
const INFO_URL_BASE: &str = "http://t-board.office.tuat.ac.jp/T/boar/vewAjax.php?i=";

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

/// For academic and Campus
pub struct Feed {
    feed_url: &'static str,
    buffer: HashMap<u32, Info>,
}

impl Feed {
    /// initialize feed
    pub fn new(feed_url: &'static str) -> Self {
        Self {
            feed_url,
            buffer: HashMap::new(),
        }
    }

    /// get the actual feed
    pub async fn get(&mut self) -> Result<Vec<Info>, TuatFeedParserError> {
        info!("fetching campus feed");
        let content = get(self.feed_url).await?;
        let ids = main_page_parser(&content).await?;

        let mut informations = Vec::new();
        for id in ids {
            let mut info = self.buffer.get(&id).cloned();
            if info.is_none() {
                info!("fetching new info {}", id);
                tokio::time::sleep(Duration::from_secs(1)).await;
                let content_result = get(&format!("{}{}", INFO_URL_BASE, id)).await;
                if content_result.is_err() {
                    continue;
                }
                let info_result = info_parser(&content_result.unwrap(), id).await;
                if info_result.is_err() {
                    continue;
                }
                info = Some(info_result.unwrap());
                self.buffer.insert(id, info.clone().unwrap());
            }
            informations.push(info.unwrap());
        }

        Ok(informations)
    }
}
