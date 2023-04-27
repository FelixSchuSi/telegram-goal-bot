use crate::filter::videohost::VideoHost;
use std::str::FromStr;

use super::scrape_with_browser::scrape_with_browser;

#[derive(Debug, Clone, PartialEq)]
pub struct ScrapeError(pub String);

pub async fn scrape_video(url: String) -> Result<String, ScrapeError> {
    let video_host =
        VideoHost::from_str(&url).map_err(|_| ScrapeError("Unkown VideoHost".to_string()))?;

    let response = reqwest::get(&url)
        .await
        .map_err(|_| ScrapeError("Request to VideoHost site failed".to_string()))?
        .text()
        .await
        .map_err(|_| ScrapeError("Getting text Response from Request failed".to_string()))?;

    let document = scraper::Html::parse_document(&response);
    let selector: &str;
    let attribute: &str;
    let mut client_side_rendered: bool = false;
    let _ = match video_host {
        VideoHost::Streamwo => {
            selector = "body video > source";
            attribute = "src";
            // TODO: scraping Streamwo is more complicated.
        }
        VideoHost::Streamja => {
            selector = "video > source";
            attribute = "src";
        }
        VideoHost::Streamye => {
            selector = "body video > source";
            attribute = "src";
            // TODO: scraping streamye is more complicated.
        }
        VideoHost::Streamable => {
            selector = "div > video";
            attribute = "src";
        }
        VideoHost::Imgtc => {
            selector = "div#player>iframe";
            attribute = "src";
        }
        VideoHost::Clippituser => {
            selector = "#player-container";
            attribute = "data-hd-file";
        }
        VideoHost::Vimeo => {
            selector = "div#player>iframe";
            attribute = "src";
        }
        VideoHost::Streamvi => {
            selector = "video > source";
            attribute = "src";
        }
        VideoHost::Juststream => {
            selector = "div#player>iframe";
            attribute = "src";
        }
        VideoHost::Streamff => {
            selector = "video";
            attribute = "src";
            client_side_rendered = true;
        }
        VideoHost::Streamgg => {
            selector = "video > source";
            attribute = "src";
        }
        VideoHost::Streamin => {
            selector = "video";
            attribute = "src";
        }
        VideoHost::Dubz => {
            selector = "video > source";
            attribute = "src";
        }
        VideoHost::Streambug => {
            selector = "video";
            attribute = "src";
            client_side_rendered = true;
        }
    };
    let mut value: String;
    if client_side_rendered {
        let res = scrape_with_browser(&url, selector, attribute);
        value = res.map_err(|_| {
            ScrapeError(format!(
            "Client Side Scraping failed for host: {:?} url: {:?} seletor: {:?} attribute: {:?}",
            video_host, url, selector, attribute
        ))
        })?;
    } else {
        let title_selector = scraper::Selector::parse(selector).map_err(|_| {
            ScrapeError("The given String could not be parsed to a CSS selector".to_string())
        })?;
        let element = document.select(&title_selector).next().ok_or(ScrapeError(
            "No Element matching the CSS Selector present in the scraped site".to_string(),
        ))?;
        value = element
            .value()
            .attr(attribute)
            .ok_or(ScrapeError(format!(
                "The given DOM-Element does not have the {} attribute",
                attribute
            )))?
            .to_owned();
    }

    (!value.ends_with(".mov"))
        .then_some(value.to_string())
        .ok_or(ScrapeError(
            ".mov files are not properly supported by telegram".to_string(),
        ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_for_unkown_host() {
        let result = scrape_video("https://streamall.me/ra_goO7Rm".to_string()).await;
        assert!(result.is_err());
        assert!(result.err().unwrap() == ScrapeError("Unkown VideoHost".to_string()));
    }

    #[tokio::test]
    async fn test_streamin_1() {
        let result = scrape_video("https://streamin.me/v/99dadc6d".to_string()).await;
        assert!(result.is_ok());
        assert!(result.unwrap() == "https://cdn.discordapp.com/attachments/1093271807102038049/1099336438090313778/99dadc6d.mp4".to_string());
    }

    #[tokio::test]
    async fn test_client() {
        let result = scrape_video("https://streamff.com/v/qagwUNlcwP".to_string()).await;
        assert!(result.is_ok());
        assert!(result.unwrap() == "https://files.catbox.moe/3cdo9q.mp4".to_string());
    }
}
