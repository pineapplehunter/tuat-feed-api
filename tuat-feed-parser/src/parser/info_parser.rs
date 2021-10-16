use super::error::ParseError;
use crate::Info;
use scraper::{Html, Selector};

pub async fn info_parser(content: &str, id: u32) -> Result<Info, ParseError> {
    let mut information = Info::new(id);

    let info_doc = Html::parse_document(content);
    let tr_selector = Selector::parse("table>tbody>tr").unwrap();

    for infos in info_doc.select(&tr_selector) {
        let data_selector = Selector::parse("td").unwrap();
        let mut data = infos.select(&data_selector);
        if let Some(label_elem) = data.next() {
            if label_elem.value().attr("class") != Some("defLabel") {
                continue;
            }
            let mut label_text = label_elem.text().collect::<String>();
            let data_text;
            match &label_text {
                text if text.starts_with("添付ファイル") => {
                    label_text = String::from("添付ファイル");
                    let ancor = Selector::parse("a").unwrap();
                    data_text = data
                        .next()
                        .unwrap()
                        .select(&ancor)
                        .filter_map(|elem| -> Option<String> {
                            Some(format!(
                                "[{}](http://t-board.office.tuat.ac.jp{})",
                                elem.text().collect::<String>().trim(),
                                elem.value().attr("href")?
                            ))
                        })
                        .collect::<Vec<String>>()
                        .join("\n");
                }
                text if text.starts_with("対象") => continue,
                _ => {
                    data_text = data
                        .map(|elem| {
                            let mut string: String = elem
                                .text()
                                .map(|s| s.trim().to_owned())
                                .collect::<Vec<String>>()
                                .join("\n");
                            if string.ends_with('\n') {
                                string.pop();
                            }
                            string
                        })
                        .filter(|val| !val.contains("テーブル表示"))
                        .collect::<Vec<String>>()
                        .join("\n");
                }
            }
            information.data.insert(label_text, data_text);
        }
    }

    Ok(information)
}

#[cfg(test)]
mod test {
    const TEST_DATA: &str = include_str!("../../test_assets/test_info.html");
    use super::info_parser;
    use crate::Info;
    use std::collections::HashMap;

    #[tokio::test]
    async fn info_parse() {
        let info: Info = info_parser(TEST_DATA, 8000).await.unwrap();

        let mut correct = HashMap::<String, String>::new();
        correct.insert("カテゴリー".into(), "休講・補講".into());
        correct.insert("担当者".into(), "(柴田\u{3000}和樹)".into());
        correct.insert("発信元".into(), "教務係".into());
        correct.insert("本文".into(), "".into());
        correct.insert(
            "公開期間".into(),
            "2018/10/05(Fri) 〜 2019/02/01(Fri)".into(),
        );
        correct.insert("最終更新日".into(), "2019/03/27(Wed)".into());
        correct.insert(
            "タイトル".into(),
            "【 線形代数学II 】 2019/02/01 金曜１限".into(),
        );

        assert_eq!(info.data, correct);
    }
}
