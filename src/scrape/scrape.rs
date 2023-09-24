use std::str::FromStr;

use crate::filter::videohost::VideoHost;
use crate::scrape::scrape_with_browser::scrape_with_browser;
use crate::scrape::scrape_without_browser::scrape_without_browser;

#[derive(Debug, Clone, PartialEq)]
pub struct ScrapeError(pub String);

// impl From<Error> for ScrapeError {
//     fn from(value: Error) -> Self {
//         ScrapeError("Reqwest error:".to_string())
//     }
// }
//
// impl From<anyhow::Error> for ScrapeError {
//     fn from(value: anyhow::Error) -> Self {
//         ScrapeError("Browser error:".to_string())
//     }
// }
//
// impl From<SelectorErrorKind<'_>> for ScrapeError {
//     fn from(value: SelectorErrorKind) -> Self {
//         ScrapeError("The given String could not be parsed to a CSS selector".to_string())
//     }
// }


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
                    return "https://streamye.com".to_string() + &*ok_value.replace(".", "");
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
        return Err(ScrapeError("Scrape Error: .mov files are not properly supported by telegram".to_string()));
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
    async fn test_streamin_1() {
        let result = scrape_video("https://streamin.me/v/99dadc6d".to_string()).await;
        assert!(result.is_ok());
        assert!(result.unwrap() == "https://downloader.disk.yandex.ru/disk/6b74e9862df24b84052f9874c03dc23a953f5593619a4c67d57ea663649983d8/651096a4/MuDSbA9z5TnczT15nZM5t48mFaNo47yRELa2uYBDsKAhbc-Zc2sJKjaB7yIimz_hlK73GjhtT4QASMs0u9Ka5Q%3D%3D?uid=465360380&filename=99dadc6d.mp4&disposition=attachment&hash=&limit=0&content_type=video%2Fmp4&owner_uid=465360380&fsize=11871274&hid=c663608021832d16b1eab202d97ec9a2&media_type=video&tknv=v2&etag=51b5cbe01eab6829a15d2fe4fd4da538&expires=1695585555#t=0.1".to_string());
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
