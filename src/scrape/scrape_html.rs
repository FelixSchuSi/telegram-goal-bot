use crate::scrape::scrape::ScrapeError;
use scraper::Html;

pub fn scrape_html(html: &Html, selector: &str, attribute: &str) -> Result<String, ScrapeError> {
    let title_selector = scraper::Selector::parse(selector).map_err(|_| {
        ScrapeError("The given String could not be parsed to a CSS selector".to_string())
    })?;
    let element = html
        .select(&title_selector)
        .next()
        .ok_or(ScrapeError(
            "No Element matching the CSS Selector present in the scraped site".to_string(),
        ))
        .map_err(|_| {
            ScrapeError(
                "No Element matching the CSS Selector present in the scraped site".to_string(),
            )
        })?;
    let attribute_value = element.value().attr(attribute).ok_or(ScrapeError(format!(
        "The given DOM-Element does not have the {} attribute",
        attribute
    )));
    let attribute_value_str = attribute_value
        .expect("Error when extracting attribute while scraping")
        .to_owned();
    return Ok(attribute_value_str);
}
