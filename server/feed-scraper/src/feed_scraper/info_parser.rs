use super::error::ParseError;
use crate::Post;
use scraper::{Html, Selector};

pub async fn info_parser(content: &str, id: u32) -> Result<Post, ParseError> {
    let mut information = Post::new(id);

    let info_doc = Html::parse_document(content);
    let tr_selector = Selector::parse("table>tbody>tr").unwrap();

    for infos in info_doc.select(&tr_selector) {
        let data_selector = Selector::parse("td").unwrap();
        let mut data = infos.select(&data_selector);
        if let Some(label_elem) = data.next() {
            if label_elem.value().attr("class") != Some("defLabel") {
                continue;
            }
            let label_text = label_elem.text().collect::<String>();
            match label_text.trim() {
                "対象" => continue,
                label_text if label_text.starts_with("添付ファイル") => {
                    let ancor = Selector::parse("a").unwrap();
                    let attachment_iter = data.next().unwrap().select(&ancor).filter_map(
                        |elem| -> Option<(String, String)> {
                            Some((
                                elem.text().collect::<String>().trim().to_string(),
                                format!(
                                    "http://t-board.office.tuat.ac.jp{}",
                                    elem.value().attr("href")?
                                ),
                            ))
                        },
                    );
                    for (key, val) in attachment_iter {
                        information.attachment.insert(key, val);
                    }
                }
                _ => {
                    let data_text = data
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
                    match label_text.trim() {
                        "タイトル" => {
                            information.title.replace(data_text);
                        }
                        "本文" => {
                            information.contents.replace(data_text);
                        }
                        "担当者" => {
                            information.person_in_charge.replace(data_text);
                        }
                        "発信元" => {
                            information.origin.replace(data_text);
                        }
                        "カテゴリー" => {
                            information.category.replace(data_text);
                        }
                        "最終更新日" => {
                            let mut data_text = data_text;
                            while data_text.pop() != Some('(') {}
                            let data_text = data_text.replace('/', "-");
                            information.updated_date.replace(data_text);
                        }
                        "公開期間" => {
                            let (start, end) = data_text.split_once(" 〜 ").unwrap();
                            let mut start = start.to_string();
                            while start.pop() != Some('(') {}
                            let start = start.replace('/', "-");

                            let mut end = end.to_string();
                            while end.pop() != Some('(') {}
                            let end = end.replace('/', "-");

                            information.show_date.replace((start, end));
                        }
                        _ => {
                            information.other.insert(label_text, data_text);
                        }
                    };
                }
            }
        }
    }

    Ok(information)
}

#[cfg(test)]
mod test {
    const TEST_DATA: &str = include_str!("../../test_assets/test_info.html");

    use super::info_parser;
    use crate::Post;
    use std::collections::HashMap;

    #[tokio::test]
    async fn info_parse() {
        let info: Post = info_parser(TEST_DATA, 8000).await.unwrap();

        let mut attachment = HashMap::new();
        attachment.insert("【工学府】R3後期集中講義一覧1007.pdf".to_string(), "http://t-board.office.tuat.ac.jp/T/fmapi/getFile.php?path=%2Ffmi%2Fxml%2Fcnt%2F%25E3%2580%2590%25E5%25B7%25A5%25E5%25AD%25A6%25E5%25BA%259C%25E3%2580%2591R3%25E5%25BE%258C%25E6%259C%259F%25E9%259B%2586%25E4%25B8%25AD%25E8%25AC%259B%25E7%25BE%25A9%25E4%25B8%2580%25E8%25A6%25A71007.pdf%3F-db%3DTUTw%26-lay%3DBoarVewType0%26-recid%3D11367%26-field%3DBoarFile%3A%3ArFile%281%29.15534&name=%E3%80%90%E5%B7%A5%E5%AD%A6%E5%BA%9C%E3%80%91R3%E5%BE%8C%E6%9C%9F%E9%9B%86%E4%B8%AD%E8%AC%9B%E7%BE%A9%E4%B8%80%E8%A6%A71007.pdf".to_string());

        let correct = Post {
            post_id: 8000,
                title: Some("10/7更新\n【工学府】 2021年度後期集中講義の開講について".to_string()),
                contents: Some("10/7更新：ゲノム情報解析工学特論、先端ゲノム情報解析工学特論について追記しました。\n\n詳細は添付ファイルを参照。\n※講義ごとの開講案内を随時掲載します。\n※未定については、わかり次第お知らせします。".to_string()),
                updated_date: Some("2021-10-07".to_string()),
                show_date: Some(("2021-10-07".to_string(),"2022-03-31".to_string())),
                person_in_charge: Some("教務係".to_string()),
                origin: Some("教務係".to_string()),
                category: Some("集中講義 Intensive Lectures".to_string()),
                attachment,
                other: HashMap::new(),
        };

        let _ = dbg!(serde_json::to_string(&correct));

        assert_eq!(info, correct);
    }
}
