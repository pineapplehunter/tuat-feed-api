#![warn(missing_docs)]

//! # tuat-feed-parser
//! this crate provides a api to access the tuat feed as a struct.

use std::{collections::HashMap, time::Duration};

use thiserror::Error;
use tuat_feed_common::Post;

mod feed_scraper;
mod get;

use feed_scraper::{error::ParseError, info_parser, main_page_parser};
pub use get::{get, GetError};

use tracing::{debug, info, Instrument};

/// campas feed url
const T_CAMPUS_FEED_URL: &str =
    "http://t-board.office.tuat.ac.jp/T/boar/resAjax.php?bAnno=0&par=20&skip=0";
/// academic feed url
const T_ACADEMIC_FEED_URL: &str =
    "http://t-board.office.tuat.ac.jp/T/boar/resAjax.php?bAnno=1&par=20&skip=0";
const T_INFO_URL_BASE: &str = "http://t-board.office.tuat.ac.jp/T/boar/vewAjax.php?i=";

/// campas feed url
const A_CAMPUS_FEED_URL: &str =
    "http://t-board.office.tuat.ac.jp/A/boar/resAjax.php?bAnno=0&par=20&skip=0";
/// academic feed url
const A_ACADEMIC_FEED_URL: &str =
    "http://t-board.office.tuat.ac.jp/A/boar/resAjax.php?bAnno=1&par=20&skip=0";
const A_INFO_URL_BASE: &str = "http://t-board.office.tuat.ac.jp/A/boar/vewAjax.php?i=";

#[derive(Debug)]
/// 学部
pub enum Gakubu {
    /// 工学部
    Technology,
    /// 農学部
    Agriculture,
}

#[derive(Debug)]
/// カテゴリ
pub enum FeedCategory {
    /// キャンパス情報
    Campus,
    /// 教務情報
    Academic,
}

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
#[derive(Debug)]
pub struct Feed {
    name: String,
    feed_url: &'static str,
    info_url: &'static str,
    buffer: HashMap<u32, Post>,
}

impl Feed {
    /// initialize feed
    pub fn new(gakubu: Gakubu, category: FeedCategory) -> Self {
        let (feed_url, info_url) = match gakubu {
            Gakubu::Technology => match category {
                FeedCategory::Campus => (T_CAMPUS_FEED_URL, T_INFO_URL_BASE),
                FeedCategory::Academic => (T_ACADEMIC_FEED_URL, T_INFO_URL_BASE),
            },
            Gakubu::Agriculture => match category {
                FeedCategory::Campus => (A_CAMPUS_FEED_URL, A_INFO_URL_BASE),
                FeedCategory::Academic => (A_ACADEMIC_FEED_URL, A_INFO_URL_BASE),
            },
        };
        Self {
            name: format!("{:?} {:?}", gakubu, category),
            feed_url,
            info_url,
            buffer: HashMap::new(),
        }
    }

    /// get the actual feed
    #[tracing::instrument]
    pub async fn fetch(&mut self) -> Result<Vec<Post>, TuatFeedParserError> {
        info!("fetching {} feed start", self.name);
        let content = get(self.feed_url).await?;
        let ids = main_page_parser(content).await?;

        let mut informations = Vec::new();
        for id in ids {
            let mut info = self.buffer.get(&id).cloned();
            if info.is_none() {
                debug!("fetching new info {} from {}", id, self.name);
                tokio::time::sleep(Duration::from_secs(1))
                    .instrument(tracing::debug_span!("delay"))
                    .await;
                let content_result = get(&format!("{}{}", self.info_url, id)).await;
                if content_result.is_err() {
                    continue;
                }
                let info_result = info_parser(content_result.unwrap(), id).await;
                if info_result.is_err() {
                    continue;
                }
                info = Some(info_result.unwrap());
                self.buffer.insert(id, info.clone().unwrap());
            }
            informations.push(info.unwrap());
        }

        info!("fetching {} feed done", self.name);
        Ok(informations)
    }
}
