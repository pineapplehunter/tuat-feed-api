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
