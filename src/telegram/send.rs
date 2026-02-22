use crate::filter::competition::Competition;
use log::error;
use std::{path::PathBuf, time::Duration};
use teloxide::{
    payloads::SendVideoSetters,
    requests::{Request, Requester},
    types::{InputFile, Message},
    Bot, RequestError,
};
use tokio::time::sleep;

pub async fn send_video(
    caption: &str,
    bot: &Bot,
    video: &PathBuf,
    competition: &Competition,
) -> Result<Message, RequestError> {
    bot.send_video(competition.get_chat_id(), InputFile::file(video))
        .caption(caption)
        .width(1920)
        .height(1080)
        .supports_streaming(true)
        .send()
        .await
}

pub async fn send_video_with_retries(
    caption: &str,
    bot: &Bot,
    video: &PathBuf,
    competition: &Competition,
) -> Result<Message, RequestError> {
    let mut send_video_result = send_video(caption, bot, video, competition).await;
    if send_video_result.is_ok() {
        return send_video_result;
    }

    for i in 2..10 {
        send_video_result = send_video(caption, bot, video, competition)
            .await
            .map_err(|err| {
                error!(
                    "Sending video failed in try {} out of 10. Error: {} caption: {}",
                    i, err, caption
                );
                err
            }).map(|res| {
            error!(
                "Sending video was successfull in try {} out of 10. MessageId: {} submission.title: {} ",
                i,
                &res.id.0,
                caption
            );
            res
        });

        if send_video_result.is_ok() {
            break;
        }
        sleep(Duration::from_secs(30)).await;
    }
    send_video_result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        config::config::Config, download_video::download_video::download_video_with_retries,
    };
    use dotenv::dotenv;
    use std::{fs::remove_file, sync::Arc};
    use tokio::fs;

    #[tokio::test]
    #[ignore]
    async fn test_dubz() {
        dotenv().ok();
        let config = Arc::new(Config::init());
        let bot = Arc::new(Bot::from_env());

        let downloaded_video =
            download_video_with_retries("https://downloader.disk.yandex.ru/disk/e8b52ed374664f4a76cef4a99abba2ebf175dece7eca9694303df3bf770a4490/653d334e/MuDSbA9z5TnczT15nZM5t-vXjxBtxqrUKkbcdLjGqS4sxjjxSFw_AzCx8cXDcG6Awi7xaQmVuiGG63m5_H150A%3D%3D?uid=465360380&filename=6022b59f.mp4&disposition=attachment&hash=&limit=0&content_type=video%2Fmp4&owner_uid=465360380&fsize=5365038&hid=d512aa5109dd00c8e746ffd2023a3ac8&media_type=video&tknv=v2&etag=e1e899a40627c96b01de5f4029a3efc2&expires=1698509646#t=0.1")
                .await
                .unwrap();
        let res = send_video_with_retries(
            "Crystal Palace [1] - 2 Tottenham - Jordan Ayew 90+4'",
            &bot,
            &downloaded_video,
            &config.premier_league,
        )
        .await;

        let cloned1 = downloaded_video.clone();
        let cloned2 = downloaded_video.clone();
        assert!(fs::metadata(downloaded_video).await.is_ok());
        remove_file(cloned1).unwrap();
        assert!(fs::metadata(cloned2).await.is_err());
        assert!(res.is_ok())
    }

    #[tokio::test]
    #[ignore]
    async fn test_dubz01() {
        dotenv().ok();
        let config = Arc::new(Config::init());
        let bot = Arc::new(Bot::from_env());

        let downloaded_video =
            download_video_with_retries("https://dubzalt.com/storage/videos/e3457f.mp4")
                .await
                .unwrap();
        let res =
            send_video_with_retries("test123", &bot, &downloaded_video, &config.premier_league)
                .await;

        let cloned1 = downloaded_video.clone();
        let cloned2 = downloaded_video.clone();
        assert!(fs::metadata(downloaded_video).await.is_ok());
        remove_file(cloned1).unwrap();
        assert!(fs::metadata(cloned2).await.is_err());
        assert!(res.is_ok())
    }
}
