use std::str::FromStr;
use std::time::Duration;

use log::{error, info};
use scraper::Html;
use tokio::time::sleep;
use url::{ParseError, Url};

use crate::filter::videohost::VideoHost;
use crate::scrape::get_html_with_browser::get_html_with_browser;
use crate::scrape::get_html_without_browser::get_html_without_browser;
use crate::scrape::scrape_html::scrape_html;

#[derive(Debug, Clone, PartialEq)]
pub struct ScrapeError(pub String);

pub struct ScrapeRetryWithTimeoutOptions {
    pub timeout_ms: u64,
    pub max_retries: u8,
    pub url: String,
}

pub async fn scrape_video(url: &str) -> Result<Url, ScrapeError> {
    let retry_options = ScrapeRetryWithTimeoutOptions {
        max_retries: 12,
        timeout_ms: 10_000,
        url: String::from(url),
    };

    let scrape_result = scrape_with_retries(&retry_options).await?;

    if scrape_result.ends_with(".mov") {
        return Err(ScrapeError(
            "Scrape Error: .mov files are not properly supported by telegram".to_string(),
        ));
    }

    Url::parse(&scrape_result).map_err(|open_error| {
        error!(
            "Parsing the scraped url failed scrape_result: {} original url: {}",
            scrape_result, url
        );
        ScrapeError(open_error.to_string())
    })
}

async fn scrape_with_retries(
    options: &ScrapeRetryWithTimeoutOptions,
) -> Result<String, ScrapeError> {
    let video_host = VideoHost::from_str(&options.url)
        .map_err(|_| ScrapeError("Unkown VideoHost".to_string()))?;
    for i in 0..options.max_retries {
        let html: Html = get_html(&options.url).await?;
        let scrape_result = scrape_from_html(&html, &video_host);
        match scrape_result {
            Ok(result) => {
                return Ok(result);
            }
            Err(err) => {
                if i == options.max_retries {
                    error!(
                        "Scraping of url {} failed after {} attempts with timeout {}: {}",
                        &options.url, options.max_retries, options.timeout_ms, err.0
                    );
                    return Err(err);
                } else {
                    info!("Scraping of url {} failed in attempt no. {} out of {}: {} Trying again in {} ms.",&options.url, i, options.max_retries, err.0, options.timeout_ms);
                    sleep(Duration::from_millis(options.timeout_ms)).await;
                    info!("Trying again...");
                }
            }
        }
    }
    return Err(ScrapeError(format!(
        "Scraping of url {} failed after {} attempts with timeout {}",
        &options.url, options.max_retries, options.timeout_ms
    )));
}

async fn get_html(url: &str) -> Result<Html, ScrapeError> {
    let video_host =
        VideoHost::from_str(url).map_err(|_| ScrapeError("Unkown VideoHost".to_string()))?;
    match video_host {
        VideoHost::Streamwo
        | VideoHost::Streamja
        | VideoHost::Streamye
        | VideoHost::Streamable
        | VideoHost::Imgtc
        | VideoHost::Clippituser
        | VideoHost::Vimeo
        | VideoHost::Streamvi
        | VideoHost::Juststream
        | VideoHost::Streamgg
        | VideoHost::Streamin => get_html_without_browser(&url).await,
        VideoHost::Dubz => {
            // Dubz has some clever blocking mechanism. They use cloudflare ddos/bot protection which is hard to bypass.
            // However this protection is only enabled on 'dubz.link' and 'dubz.co' but not on 'dubz.cc'.
            // Urls on dubz are valid across all Top Level Domains.
            // Therefore we manually change the url from '.link' and '.co' to '.cc' to bypass the ddos protection.
            let source_url = String::from(url);
            let target_url = source_url
                .replacen(".link", ".cc", 1)
                .replacen(".co", ".cc", 1);
            get_html_without_browser(&target_url).await
        }
        VideoHost::Streambug | VideoHost::Streamff => get_html_with_browser(&url, "video").await,
    }
}

fn scrape_from_html(html: &Html, video_host: &VideoHost) -> Result<String, ScrapeError> {
    match video_host {
        VideoHost::Streamwo => {
            let selector = "body video > source";
            let attribute = "src";
            let result = scrape_html(&html, selector, attribute);
            result.map(|ok_value| add_host_if_url_is_relative(&ok_value, "https://streamwo.com"))?
        }
        VideoHost::Streamja => {
            let selector = "video > source";
            let attribute = "src";
            scrape_html(&html, selector, attribute)
        }
        VideoHost::Streamye => {
            let selector = "body video > source";
            let attribute = "src";
            let result = scrape_html(&html, selector, attribute);
            result.map(|ok_value| add_host_if_url_is_relative(&ok_value, "https://streamye.com"))?
        }
        VideoHost::Streamable => {
            let selector = "div > video";
            let attribute = "src";
            scrape_html(&html, selector, attribute)
        }
        VideoHost::Imgtc => {
            let selector = "div#player>iframe";
            let attribute = "src";
            scrape_html(&html, selector, attribute)
        }
        VideoHost::Clippituser => {
            let selector = "#player-container";
            let attribute = "data-hd-file";
            scrape_html(&html, selector, attribute)
        }
        VideoHost::Vimeo => {
            let selector = "div#player>iframe";
            let attribute = "src";
            scrape_html(&html, selector, attribute)
        }
        VideoHost::Streamvi => {
            let selector = "video > source";
            let attribute = "src";
            scrape_html(&html, selector, attribute)
        }
        VideoHost::Juststream => {
            let selector = "div#player>iframe";
            let attribute = "src";
            scrape_html(&html, selector, attribute)
        }
        VideoHost::Streamff => {
            let selector = "video";
            let attribute = "src";
            scrape_html(&html, selector, attribute)
        }
        VideoHost::Streamgg => {
            let selector = "video > source";
            let attribute = "src";
            scrape_html(&html, selector, attribute)
        }
        VideoHost::Streamin => {
            let selector = "video";
            let attribute = "src";
            let result = scrape_html(&html, selector, attribute);
            result.map(|ok_value| add_host_if_url_is_relative(&ok_value, "https://streamin.me"))?
        }
        VideoHost::Dubz => {
            let attribute = "src";
            scrape_html(&html, "video > source", attribute)
                .or_else(|_| scrape_html(&html, "video", attribute))
        }
        VideoHost::Streambug => {
            let selector = "video";
            let attribute = "src";
            scrape_html(&html, selector, attribute)
        }
    }
}

fn add_host_if_url_is_relative(url: &str, host: &str) -> Result<String, ScrapeError> {
    match Url::parse(url) {
        Ok(url) => Ok(url.to_string()),
        Err(error) => match error {
            ParseError::RelativeUrlWithoutBase => {
                let mut url = String::from(url);
                let mut host = String::from(host);

                if url.starts_with(".") {
                    url = url.replacen(".", "", 1);
                }
                if !url.starts_with("/") {
                    url = "/".to_owned() + &url;
                }
                if host.ends_with("/") {
                    host.replace_range(host.len()..host.len(), "")
                }

                let absolute_url = host + &url;

                match Url::parse(&absolute_url) {
                    Ok(value) => Ok(value.to_string()),
                    Err(error) => Err(ScrapeError(format!(
                        "Concatted url is not valid: {} error: {}",
                        absolute_url, error
                    ))),
                }
            }
            _ => Err(ScrapeError(format!(
                "Keine gültige URL: {} Host: {} Fehler: {}",
                url, host, error
            ))),
        },
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[tokio::test]
    async fn unkown_host() {
        let result = scrape_video("https://streamall.me/ra_goO7Rm").await;
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap(),
            ScrapeError("Unkown VideoHost".to_string())
        );
    }

    #[test]
    #[ignore]
    fn test_without_request_streamin01() {
        let html =
            Html::parse_document(&fs::read_to_string("src/scrape/test_streamin_01.html").unwrap());
        let scrape_result = scrape_from_html(&html, &VideoHost::Streamin);

        assert_eq!(
            &scrape_result.unwrap(),
            "https://downloader.disk.yandex.ru/disk/93bd60a079d5726fda7721bbc65d27e96431c058d31b42dd7fb2a1c69f339f9d/65183569/MuDSbA9z5TnczT15nZM5twEz0OtIxeLw0cLxB6HQnV1NDOt2lhxpEWa_fdc4Sqp5z6QiBEyy2_PsAQ3xS9fODQ%3D%3D?uid=465360380&filename=3727f22e.mp4&disposition=attachment&hash=&limit=0&content_type=video%2Fmp4&owner_uid=465360380&fsize=20087591&hid=7c480a3c85d2da524ebd87aa41f15646&media_type=video&tknv=v2&etag=624eb7c4caff4cf91bb941a8de743347&expires=1696084952#t=0.1"
        );
    }

    #[tokio::test]
    #[ignore]
    async fn test_with_request_streamin01() {
        let result = scrape_video("https://streamin.me/v/3727f22e").await;
        assert!(result.is_ok());
        // streamin is rotating the concrete source url of the videos.
        // Testing that we are able to successfully scrape is all we can do.
    }

    #[tokio::test]
    #[ignore]
    async fn test_with_request_streamin02() {
        let result = scrape_video("https://streamin.me/v/1d981b6d").await;
        assert!(result.is_ok());
        // streamin is rotating the concrete source url of the videos.
        // Testing that we are able to successfully scrape is all we can do.
    }

    #[tokio::test]
    #[ignore]
    async fn test_with_request_streamin03() {
        let result = scrape_video("https://streamin.me/v/fa0ebe20").await;
        assert!(result.is_ok());
        // streamin is rotating the concrete source url of the videos.
        // Testing that we are able to successfully scrape is all we can do.
    }

    #[tokio::test]
    #[ignore]
    async fn test_with_request_streamff01() {
        let result = scrape_video("https://streamff.com/v/qagwUNlcwP").await;
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            Url::parse("https://files.catbox.moe/3cdo9q.mp4").unwrap()
        );
    }

    #[test]
    fn test_without_request_dubz01() {
        let html =
            Html::parse_document(&fs::read_to_string("src/scrape/test_dubz_01.html").unwrap());
        let scrape_result = scrape_from_html(&html, &VideoHost::Dubz);

        assert_eq!(
            &scrape_result.unwrap(),
            "https://dubzalt.com/storage/videos/3ea24e.mp4"
        );
    }

    #[tokio::test]
    #[ignore]
    async fn test_with_request_dubz01() {
        let result = scrape_video("https://dubz.link/v/akm002").await;
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            Url::parse("https://squeelab.com/uploaded/1700257299.mp4#t=0.1").unwrap()
        );
    }

    #[tokio::test]
    async fn test_failed_retry() {
        let retry_options = ScrapeRetryWithTimeoutOptions {
            timeout_ms: 1,
            max_retries: 5,
            url: String::from("https://streamin.me/v/9e5d5f6b"),
        };
        let result = scrape_with_retries(&retry_options).await;
        let err = result.unwrap_err();
        assert_eq!(err.0, String::from("Scraping of url https://streamin.me/v/9e5d5f6b failed after 5 attempts with timeout 1"));
    }

    #[test]
    fn test_add_host_if_url_is_relative() {
        assert_eq!(
            add_host_if_url_is_relative("/uploads/fc38d308.mp4#t=0.1", "https://streamin.me"),
            Ok("https://streamin.me/uploads/fc38d308.mp4#t=0.1".to_string())
        );
        assert_eq!(
            add_host_if_url_is_relative("./uploads/2e5be99a.mp4#t=0.1", "https://streamin.me"),
            Ok("https://streamin.me/uploads/2e5be99a.mp4#t=0.1".to_string())
        );
        assert_eq!(
            add_host_if_url_is_relative(
                "https://dubzalt.com/storage/videos/3ea24e.mp4",
                "https://dubzalt.com"
            ),
            Ok("https://dubzalt.com/storage/videos/3ea24e.mp4".to_string())
        );
    }
}
