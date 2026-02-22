use crate::scrape::scrape::ScrapeError;
use scraper::Html;

pub async fn get_html_without_browser(url: &str) -> Result<Html, ScrapeError> {
    let document = reqwest::get(url)
        .await
        .map_err(|err| ScrapeError("Error while getting url: ".to_owned() + &*err.to_string()))?
        .text()
        .await
        .map_err(|err| {
            ScrapeError(
                "Error while extracting text from response: ".to_owned() + &*err.to_string(),
            )
        })?;
    Ok(Html::parse_document(&document))
}

#[cfg(test)]
mod tests {
    use crate::scrape::get_html_without_browser::get_html_without_browser;

    #[tokio::test]
    async fn test_example_org() {
        let html = get_html_without_browser("https://www.example.org").await;
        let html = html.unwrap();
        let selector = scraper::Selector::parse("h1");
        let heading_element = html.select(&selector.unwrap()).next();
        assert_eq!(
            heading_element.unwrap().inner_html().trim(),
            String::from("Example Domain")
        )
    }
}
