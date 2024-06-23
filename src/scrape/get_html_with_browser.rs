use headless_chrome::{Browser, LaunchOptions};
use scraper::Html;

use crate::scrape::scrape::ScrapeError;

pub async fn get_html_with_browser(url: &str, selector: &str) -> Result<Html, ScrapeError> {
    let mut launch_options_builder = LaunchOptions::default_builder();
    let launch_options = launch_options_builder.headless(false);
    let browser = Browser::new(launch_options.build().map_err(|err| {
        ScrapeError("Error while configuring Browser: ".to_owned() + &*err.to_string())
    })?)
    .map_err(|err| {
        ScrapeError("Error while configuring Browser: ".to_owned() + &*err.to_string())
    })?;
    let tab = browser
        .new_tab()
        .map_err(|err| ScrapeError("Error opening new tab: ".to_owned() + &*err.to_string()))?;
    tab.navigate_to(url).map_err(|err| {
        ScrapeError("Error while navigating to URL: ".to_owned() + &*err.to_string())
    })?;
    tab.wait_for_element(selector).map_err(|err| {
        ScrapeError("Could not find Element for given Selector: ".to_owned() + &*err.to_string())
    })?;
    let html = tab.get_content().map_err(|err| {
        ScrapeError("Could not extract html from browser: ".to_owned() + &*err.to_string())
    })?;
    Ok(Html::parse_document(&html))
}

#[cfg(test)]
mod tests {
    use crate::scrape::get_html_with_browser::get_html_with_browser;

    #[tokio::test]
    async fn test_example_org() {
        let html = get_html_with_browser("https://www.example.org", "h1").await;
        let html = html.unwrap();
        let selector = scraper::Selector::parse("h1");
        let heading_element = html.select(&selector.unwrap()).next();
        assert_eq!(
            heading_element.unwrap().inner_html().trim(),
            String::from("Example Domain")
        )
    }

    #[tokio::test]
    async fn test_kickbase_plus() {
        let html =
            get_html_with_browser("https://kickbaseplus.fabian-fischer.com/#/", ".headline").await;
        let html = html.unwrap();
        let selector = scraper::Selector::parse(".headline");
        let heading_element = html.select(&selector.unwrap()).next();
        assert_eq!(
            heading_element.unwrap().inner_html().trim(),
            String::from("Login")
        )
    }
}
