use super::error::ParseError;
use scraper::Selector;

#[tracing::instrument(skip(content))]
pub async fn main_page_parser(content: String) -> Result<Vec<u32>, ParseError> {
    tokio::task::spawn_blocking(move || {
        let document = scraper::Html::parse_document(&content);
        let selector = Selector::parse("table>tbody>tr").unwrap();
        let infos = document.select(&selector);

        let mut ids = Vec::new();

        for info in infos {
            let id = info
                .value()
                .attr("i")
                .ok_or_else(|| ParseError::ScrapingError("could not find attr 'i'".into()))?
                .parse::<u32>()?;
            ids.push(id);
        }

        Ok(ids)
    })
    .await
    .unwrap()
}

#[cfg(test)]
mod test {
    const TEST_DATA: &str = include_str!("../../test_assets/test_academic_feed.html");
    use super::main_page_parser;

    #[tokio::test]
    async fn info_parse() {
        let info = main_page_parser(TEST_DATA.to_owned()).await.unwrap();

        let correct = vec![
            10641, 10636, 10634, 10146, 10635, 10633, 10632, 10630, 10628, 10627, 10624, 10623,
            10622, 10597, 10621, 10620, 10619, 10611, 10577, 10576,
        ];

        assert_eq!(info, correct);
    }
}
