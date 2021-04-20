use super::error::ParseError;
use crate::Info;
use scraper::{Html, Selector};

pub async fn info_parser(content: &str, id: u32) -> Result<Info, ParseError> {
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

#[cfg(test)]
mod test {
    const TEST_DATA: &'static str = include_str!("../../test_assets/test_info.html");
    use super::info_parser;
    use crate::Info;
    use std::collections::HashMap;

    #[tokio::test]
    async fn info_parse() {
        let info: Info = info_parser(TEST_DATA, 8000).await.unwrap();

        let mut correct = HashMap::<String, String>::new();
        correct.insert("カテゴリー".into(), "休講・補講".into());
        correct.insert("担当者".into(), "(柴田\u{3000}和樹)".into());
        correct.insert("対象".into(), "工学部\n1年\n2年\n3年\n4年\n生命工学科(L)\n○\n○\n○\n○\n応用分子化学科(F)\n○\n○\n○\n○\n有機材料化学科(G)\n○\n○\n○\n○\n化学システム工学科(K)\n○\n○\n○\n○\n機械システム工学科(M)\n●\n○\n○\n○\n物理システム工学科(P)\n○\n○\n○\n○\n電気電子工学科(E)\n○\n○\n○\n○\n情報工学科(S)\n○\n○\n○\n○\n生体医用システム工学(B)2019～\n○\n○\n○\n○\n応用化学科（C)2019～\n○\n○\n○\n○\n化学物理工学科(U)2019～\n○\n○\n○\n○\n知能情報システム工学科(A)2019～\n○\n○\n○\n○\n工学府博士前期課程\n1年\n2年\n生命工学専攻\n○\n○\n応用化学専攻(C1)\n○\n○\n応用化学専攻(C2)\n○\n○\n応用化学専攻(C3)\n○\n○\n機械システム工学専攻\n○\n○\n物理システム工学専攻\n○\n○\n電気電子工学専攻\n○\n○\n情報工学専攻\n○\n○\n工学府博士後期課程\n1年\n2年\n3年\n生命工学専攻\n○\n○\n○\n応用化学専攻(C1)\n○\n○\n○\n応用化学専攻(C2)\n○\n○\n○\n応用化学専攻(C3)\n○\n○\n○\n機械システム工学専攻\n○\n○\n○\n電子情報工学(A1)\n○\n○\n○\n電子情報工学(A2)\n○\n○\n○\n電子情報工学(A3)\n○\n○\n○\n専門職学位課程（Ｉ専攻）\n1年\n2年\n産業技術専攻\n○\n○\nBASE博士前期課程\n1年\n2年\n生物機能システム科学専攻（生物システム応用科学専攻）\n○\n○\nBASE博士後期課程・博士課程\n1年\n2年\n3年\n生物機能システム科学専攻（生物システム応用科学専攻）\n○\n○\n○\n共同先進健康科学専攻\n○\n○\n○\nBASE一貫制博士課程\n1年\n2年\n3年\n4年\n5年\nリーディングプログラム\n○\n○\n○\n○\n○\n食料エネルギーシステム科学専攻\n○\n○\n○\n○\n○".into());
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
