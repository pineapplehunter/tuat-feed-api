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
        correct.insert("カテゴリー".into(), "集中講義 Intensive Lectures".into());
        correct.insert("担当者".into(), "教務係".into());
        correct.insert("発信元".into(), "教務係".into());
        correct.insert("本文".into(), "10/7更新：ゲノム情報解析工学特論、先端ゲノム情報解析工学特論について追記しました。\n\n詳細は添付ファイルを参照。\n※講義ごとの開講案内を随時掲載します。\n※未定については、わかり次第お知らせします。".into());
        correct.insert(
            "公開期間".into(),
            "2021/10/07(Thu) 〜 2022/03/31(Thu)".into(),
        );
        correct.insert("最終更新日".into(), "2021/10/07(Thu)".into());
        correct.insert(
            "タイトル".into(),
            "10/7更新\n【工学府】 2021年度後期集中講義の開講について".into(),
        );
        correct.insert(
            "添付ファイル".into(), 
            "[【工学府】R3後期集中講義一覧1007.pdf](http://t-board.office.tuat.ac.jp/T/fmapi/getFile.php?path=%2Ffmi%2Fxml%2Fcnt%2F%25E3%2580%2590%25E5%25B7%25A5%25E5%25AD%25A6%25E5%25BA%259C%25E3%2580%2591R3%25E5%25BE%258C%25E6%259C%259F%25E9%259B%2586%25E4%25B8%25AD%25E8%25AC%259B%25E7%25BE%25A9%25E4%25B8%2580%25E8%25A6%25A71007.pdf%3F-db%3DTUTw%26-lay%3DBoarVewType0%26-recid%3D11367%26-field%3DBoarFile%3A%3ArFile%281%29.15534&name=%E3%80%90%E5%B7%A5%E5%AD%A6%E5%BA%9C%E3%80%91R3%E5%BE%8C%E6%9C%9F%E9%9B%86%E4%B8%AD%E8%AC%9B%E7%BE%A9%E4%B8%80%E8%A6%A71007.pdf)".into()
        );

        assert_eq!(info.data, correct);
    }
}
