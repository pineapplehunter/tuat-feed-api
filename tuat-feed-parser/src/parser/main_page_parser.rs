use super::ParseError;
use scraper::Selector;

pub async fn main_page_parser(content: &str) -> Result<Vec<u32>, ParseError> {
    let document = scraper::Html::parse_document(content);
    let selector = Selector::parse("table>tbody>tr").unwrap();
    let infos = document.select(&selector);

    let mut ids = Vec::new();

    for info in infos.into_iter() {
        let id = info
            .value()
            .attr("i")
            .ok_or_else(|| ParseError::ScrapingError("could not find attr 'i'".into()))?
            .parse::<u32>()?;
        ids.push(id);
    }

    Ok(ids)
}

#[cfg(test)]
mod test {
    const TEST_DATA: &'static str = include_str!("../../test_assets/test_academic_feed.html");
    use super::main_page_parser;

    #[tokio::test]
    async fn info_parse() {
        let info = main_page_parser(TEST_DATA).await.unwrap();

        let correct = vec![
            10641, 10636, 10634, 10146, 10635, 10633, 10632, 10630, 10628, 10627, 10624, 10623,
            10622, 10597, 10621, 10620, 10619, 10611, 10577, 10576,
        ];

        assert_eq!(info, correct);
    }
}
