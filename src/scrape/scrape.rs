use std::str::FromStr;

use scraper::Html;

use crate::filter::videohost::VideoHost;
use crate::scrape::get_html_with_browser::get_html_with_browser;
use crate::scrape::get_html_without_browser::get_html_without_browser;
use crate::scrape::scrape_html::scrape_html;

#[derive(Debug, Clone, PartialEq)]
pub struct ScrapeError(pub String);

pub async fn scrape_video(url: String) -> Result<String, ScrapeError> {
    let video_host =
        VideoHost::from_str(&url).map_err(|_| ScrapeError("Unkown VideoHost".to_string()))?;

    let html: Html = get_html(&url).await?;
    let scrape_result = scrape_from_html(&html, &video_host)?;

    if scrape_result.ends_with(".mov") {
        return Err(ScrapeError(
            "Scrape Error: .mov files are not properly supported by telegram".to_string(),
        ));
    }

    return Ok(scrape_result);
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
        | VideoHost::Streamin
        | VideoHost::Dubz => get_html_without_browser(&url).await,
        VideoHost::Streambug | VideoHost::Streamff => get_html_with_browser(&url, "video").await,
    }
}

fn scrape_from_html(html: &Html, video_host: &VideoHost) -> Result<String, ScrapeError> {
    match video_host {
        VideoHost::Streamwo => {
            let selector = "body video > source";
            let attribute = "src";
            let result = scrape_html(&html, selector, attribute);
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
            scrape_html(&html, selector, attribute)
        }
        VideoHost::Streamye => {
            let selector = "body video > source";
            let attribute = "src";
            let result = scrape_html(&html, selector, attribute);
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
            scrape_html(&html, selector, attribute)
            // Dubz has some clever blocking mechanism.
            // The site blocks this block both if we do a server side or a client side scrape
        }
        VideoHost::Streambug => {
            let selector = "video";
            let attribute = "src";
            scrape_html(&html, selector, attribute)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[tokio::test]
    async fn unkown_host() {
        let result = scrape_video("https://streamall.me/ra_goO7Rm".to_string()).await;
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap(),
            ScrapeError("Unkown VideoHost".to_string())
        );
    }

    #[test]
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
    async fn test_with_request_streamin01() {
        let result = scrape_video("https://streamin.me/v/3727f22e".to_string()).await;
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            "https://downloader.disk.yandex.ru/disk/93bd60a079d5726fda7721bbc65d27e96431c058d31b42dd7fb2a1c69f339f9d/65183569/MuDSbA9z5TnczT15nZM5twEz0OtIxeLw0cLxB6HQnV1NDOt2lhxpEWa_fdc4Sqp5z6QiBEyy2_PsAQ3xS9fODQ%3D%3D?uid=465360380&filename=3727f22e.mp4&disposition=attachment&hash=&limit=0&content_type=video%2Fmp4&owner_uid=465360380&fsize=20087591&hid=7c480a3c85d2da524ebd87aa41f15646&media_type=video&tknv=v2&etag=624eb7c4caff4cf91bb941a8de743347&expires=1696084952#t=0.1".to_string()
        );
    }

    #[tokio::test]
    async fn test_with_request_streamin02() {
        let result = scrape_video("https://streamin.me/v/1d981b6d".to_string()).await;
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            "https://downloader.disk.yandex.ru/disk/120e783b3f4823753282ed08b88dff92d4306304a30990ddfa0be8e88a140da0/65186474/MuDSbA9z5TnczT15nZM5txl10vu2kyk2f7hYQQDQ8LHnS0IHf7DyWRtF4YbNd2e0yMqTqb5S8v1K8Mr74azPrQ%3D%3D?uid=465360380&filename=1d981b6d.mp4&disposition=attachment&hash=&limit=0&content_type=video%2Fmp4&owner_uid=465360380&fsize=4549755&hid=a8d80ec460182dbb3098aac79b800269&media_type=video&tknv=v2&etag=29fc670be24109b661408dc82d283a28&expires=1696097396#t=0.1".to_string()
        );
    }

    #[tokio::test]
    async fn test_with_request_streamin03() {
        let result = scrape_video("https://streamin.me/v/fa0ebe20".to_string()).await;
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            "https://downloader.disk.yandex.ru/disk/03b2c3594d930264936225b4c60942749197ab817bea7fa0b6186045ece1625a/651869b1/MuDSbA9z5TnczT15nZM5t_czefaxmCDPZwCdDLSQCMp7FrmgoYiNmVfHfKua7VduccOi7PHXRlia4KZ_quswZw%3D%3D?uid=465360380&filename=fa0ebe20.mp4&disposition=attachment&hash=&limit=0&content_type=video%2Fmp4&owner_uid=465360380&fsize=5753860&hid=d7e20ba9d8e8c4f1ef414f214e51337a&media_type=video&tknv=v2&etag=1f364dfd1d2940e84ce167d06b886bf5&expires=1696098737#t=0.1".to_string()
        );
    }


    #[tokio::test]
    async fn test_with_request_streamff01() {
        let result = scrape_video("https://streamff.com/v/qagwUNlcwP".to_string()).await;
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            "https://files.catbox.moe/3cdo9q.mp4".to_string()
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
        let result = scrape_video("https://dubz.link/c/3ea24e".to_string()).await;
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            "https://dubzalt.com/storage/videos/3ea24e.mp4".to_string()
        );
    }
}
