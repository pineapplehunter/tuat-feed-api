#![warn(missing_docs)]

//! # tuat-feed-parser
//! this crate provides a api to access the tuat feed as a struct.

use futures::stream::{self, StreamExt};
use scraper::{Html, Selector};
use serde::Serialize;
use std::collections::HashMap;

const CAMPUS_FEED_URL: &str =
    "http://t-board.office.tuat.ac.jp/T/boar/resAjax.php?bAnno=0&par=20&skip=0";
const ACADEMIC_FEED_URL: &str =
    "http://t-board.office.tuat.ac.jp/T/boar/resAjax.php?bAnno=1&par=20&skip=0";
static INFO_URL_BASE: &str = "http://t-board.office.tuat.ac.jp/T/boar/vewAjax.php?i=";

const BUFFERED_NUM: usize = 10;

/// holds the information id and the information as a hashmap
#[derive(Debug, Serialize, Clone)]
pub struct Info {
    /// the id of the information. found in the tuat feed.
    pub id: u32,
    /// the actual data. key is from the table on the tuat feed.
    pub data: HashMap<String, String>,
}

impl Info {
    /// creates a new `Info`
    pub fn new(id: u32) -> Self {
        Self {
            id,
            data: HashMap::new(),
        }
    }
}

/// get data from the campus feed
pub async fn get_campus_feed() -> Result<Vec<Info>, String> {
    parser(CAMPUS_FEED_URL, INFO_URL_BASE).await
}

/// get data from the academic feed
pub async fn get_academic_feed() -> Result<Vec<Info>, String> {
    parser(ACADEMIC_FEED_URL, INFO_URL_BASE).await
}

async fn parser(feed_url: &str, info_url: &str) -> Result<Vec<Info>, String> {
    let content: String = reqwest::get(feed_url)
        .await
        .map_err(|e| e.to_string())?
        .text()
        .await
        .unwrap();
    let document = scraper::Html::parse_document(&content);
    let selector = Selector::parse("table>tbody>tr").unwrap();
    let infos = document.select(&selector);

    let mut ids = Vec::new();

    for info in infos.into_iter() {
        let id = info.value().attr("i").unwrap().parse::<u32>().unwrap();
        ids.push(id);
    }

    let informations: Vec<Info> = stream::iter(ids)
        .map(|id| get_info(info_url, id))
        .buffered(BUFFERED_NUM)
        .filter_map(|v| async move { v.ok() })
        .collect()
        .await;

    Ok(informations)
}

async fn get_info(info_url: &str, id: u32) -> Result<Info, String> {
    let mut information = Info::new(id);

    let content: String = reqwest::get(&format!("{}{}", info_url, id))
        .await
        .map_err(|e| e.to_string())?
        .text()
        .await
        .unwrap();
    let info_doc = Html::parse_document(&content);
    let tr_selector = Selector::parse("table>tbody>tr").unwrap();

    for infos in info_doc.select(&tr_selector) {
        let data_selector = Selector::parse("td").unwrap();
        let mut data = infos.select(&data_selector);
        if let Some(label_elem) = data.next() {
            if label_elem.value().attr("class") != Some("defLabel") {
                continue;
            }
            let label_text = label_elem.text().collect::<String>();
            let data_text = data
                .map(|elem| elem.text().map(|s| s.trim()).collect::<String>())
                .filter(|val| !val.contains("テーブル表示"))
                .collect::<Vec<String>>()
                .join("\n");

            information.data.insert(label_text, data_text);
        }
    }

    Ok(information)
}
