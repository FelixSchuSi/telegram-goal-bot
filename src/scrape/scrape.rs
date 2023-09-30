use std::str::FromStr;

use crate::filter::videohost::VideoHost;
use crate::scrape::scrape_with_browser::scrape_with_browser;
use crate::scrape::scrape_without_browser::scrape_without_browser;

#[derive(Debug, Clone, PartialEq)]
pub struct ScrapeError(pub String);

pub async fn scrape_video(url: String) -> Result<String, ScrapeError> {
    let video_host =
        VideoHost::from_str(&url).map_err(|_| ScrapeError("Unkown VideoHost".to_string()))?;

    let scrape_result: Result<String, ScrapeError> = match video_host {
        VideoHost::Streamwo => {
            let selector = "body video > source";
            let attribute = "src";
            let result = scrape_without_browser(&url, selector, attribute).await;
            result.map(|ok_value| {
                if ok_value.starts_with(".") {
                    return "https://streamwo.com".to_string() + &*ok_value.replace(".", "");
                }
                return ok_value;
            })
        }
        VideoHost::Streamja => {
            let selector = "video > source";
            let attribute = "src";
            scrape_without_browser(&url, selector, attribute).await
        }
        VideoHost::Streamye => {
            let selector = "body video > source";
            let attribute = "src";
            let result = scrape_without_browser(&url, selector, attribute).await;
            result.map(|ok_value| {
                if ok_value.starts_with(".") {
                    return "https://streamye.com".to_string() + &*ok_value.replace(".", "");
                }
                return ok_value;
            })
        }
        VideoHost::Streamable => {
            let selector = "div > video";
            let attribute = "src";
            scrape_without_browser(&url, selector, attribute).await
        }
        VideoHost::Imgtc => {
            let selector = "div#player>iframe";
            let attribute = "src";
            scrape_without_browser(&url, selector, attribute).await
        }
        VideoHost::Clippituser => {
            let selector = "#player-container";
            let attribute = "data-hd-file";
            scrape_without_browser(&url, selector, attribute).await
        }
        VideoHost::Vimeo => {
            let selector = "div#player>iframe";
            let attribute = "src";
            scrape_without_browser(&url, selector, attribute).await
        }
        VideoHost::Streamvi => {
            let selector = "video > source";
            let attribute = "src";
            scrape_without_browser(&url, selector, attribute).await
        }
        VideoHost::Juststream => {
            let selector = "div#player>iframe";
            let attribute = "src";
            scrape_without_browser(&url, selector, attribute).await
        }
        VideoHost::Streamff => {
            let selector = "video";
            let attribute = "src";
            scrape_with_browser(&url, selector, attribute)
        }
        VideoHost::Streamgg => {
            let selector = "video > source";
            let attribute = "src";
            scrape_without_browser(&url, selector, attribute).await
        }
        VideoHost::Streamin => {
            let selector = "video";
            let attribute = "src";
            let result = scrape_without_browser(&url, selector, attribute).await;
            result.map(|ok_value| {
                if ok_value.starts_with(".") {
                    return "https://streamin.me".to_string() + &*ok_value.replace(".", "");
                }
                return ok_value;
            })
        }
        VideoHost::Dubz => {
            let selector = "video > source";
            let attribute = "src";
            scrape_without_browser(&url, selector, attribute).await
            // Dubz has some clever blocking mechanism.
            // The site blocks this block both if we do a server side or a client side scrape
        }
        VideoHost::Streambug => {
            let selector = "video";
            let attribute = "src";
            scrape_with_browser(&url, selector, attribute)
        }
    };

    if scrape_result.is_ok() && scrape_result.clone().unwrap().ends_with(".mov") {
        return Err(ScrapeError(
            "Scrape Error: .mov files are not properly supported by telegram".to_string(),
        ));
    }

    return scrape_result;
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
    async fn test_client() {
        let result = scrape_video("https://streamff.com/v/qagwUNlcwP".to_string()).await;
        assert!(result.is_ok());
        assert!(result.unwrap() == "https://files.catbox.moe/3cdo9q.mp4".to_string());
    }

    #[tokio::test]
    #[ignore]
    async fn test_haaland() {
        let result = scrape_video("https://dubz.link/c/3ea24e".to_string()).await;
        assert!(result.is_ok());
        assert!(result.unwrap() == "https://dubzalt.com/storage/videos/3ea24e.mp4".to_string());
    }
}
