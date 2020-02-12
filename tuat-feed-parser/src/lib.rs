use scraper::{Html, Selector};
use serde::Serialize;
use std::collections::HashMap;

static FEED_URL: &str = "http://t-board.office.tuat.ac.jp/T/boar/resAjax.php?bAnno=0&par=20&skip=0";
static INFO_URL_BASE: &str = "http://t-board.office.tuat.ac.jp/T/boar/vewAjax.php?i=";

#[derive(Debug, Serialize)]
pub struct Info {
    pub id: u32,
    pub data: HashMap<String, String>,
}

impl Info {
    pub fn new(id: u32) -> Self {
        Self {
            id,
            data: HashMap::new(),
        }
    }
}

pub async fn parser() -> Result<Vec<Info>, String> {
    let content: String = reqwest::get(FEED_URL)
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

    println!("{:?}", ids);

    let mut informations = Vec::new();

    for id in ids.iter() {
        let mut information = Info::new(*id);

        let content: String = reqwest::get(&format!("{}{}", INFO_URL_BASE, id))
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

        informations.push(information);
    }
    Ok(informations)
}
