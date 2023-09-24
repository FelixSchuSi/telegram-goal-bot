use crate::scrape::scrape::ScrapeError;

pub async fn scrape_without_browser(
    url: &str,
    selector: &str,
    attribute: &str,
) -> Result<String, ScrapeError> {
    let response = reqwest::get(url).await.map_err(|err| ScrapeError("Error while getting url: ".to_owned() + &*err.to_string()))?.text().await.map_err(|err| ScrapeError("Error while extracting text from response: ".to_owned() + &*err.to_string()))?;

    let document = scraper::Html::parse_document(&response);

    let title_selector = scraper::Selector::parse(selector).map_err(|_| {
        ScrapeError("The given String could not be parsed to a CSS selector".to_string())
    })?;
    let element = document.select(&title_selector).next().ok_or(ScrapeError(
        "No Element matching the CSS Selector present in the scraped site".to_string(),
    )).map_err(|_| { ScrapeError("No Element matching the CSS Selector present in the scraped site".to_string()) })?;
    let attribute_value = element
        .value()
        .attr(attribute)
        .ok_or(ScrapeError(format!(
            "The given DOM-Element does not have the {} attribute",
            attribute
        )));
    let attribute_value_str = attribute_value.expect("Error when extracting attribute while scraping").to_owned();
    return Ok(attribute_value_str);
}