use scraper::{Html, Selector};
use thiserror::Error;

use crate::Info;

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

pub async fn main_page_parser(content: &String) -> Result<Vec<u32>, ParseError> {
    let document = scraper::Html::parse_document(content);
    let selector = Selector::parse("table>tbody>tr").unwrap();
    let infos = document.select(&selector);

    let mut ids = Vec::new();

    for info in infos.into_iter() {
        let id = info
            .value()
            .attr("i")
            .ok_or(ParseError::ScrappingError("could not find attr 'i'".into()))?
            .parse::<u32>()?;
        ids.push(id);
    }

    Ok(ids)
}

pub async fn info_parser(content: &String, id: u32) -> Result<Info, ParseError> {
    let mut information = Info::new(id);

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
