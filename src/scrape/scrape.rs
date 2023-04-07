use std::str::FromStr;

use reqwest::Error;

use crate::filter::videohost::VideoHost;

pub async fn scrape_video(url: String) -> Result<String, Error> {
    let video_host = VideoHost::from_str(&url).unwrap();
    let response = reqwest::get(url).await?.text().await?;
    let document = scraper::Html::parse_document(&response);
    let selector: &str;
    let attribute: &str;
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
            selector = "video";
            attribute = "src";
        }
    };
    let title_selector = scraper::Selector::parse(selector).unwrap();
    let element = document.select(&title_selector).next().unwrap();
    Ok(element.value().attr(attribute).unwrap().to_string())
}
