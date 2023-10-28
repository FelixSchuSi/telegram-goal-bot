use std::{path::PathBuf, time::Duration};

use log::error;
use teloxide::{
    payloads::{SendMessageSetters, SendVideoSetters},
    requests::{Request, Requester},
    types::{ChatId, InputFile, Message, MessageId, ParseMode},
    Bot, RequestError,
};
use tokio::time::sleep;

use crate::filter::competition::Competition;

pub async fn send_video(
    caption: &str,
    bot: &Bot,
    video: &PathBuf,
    competition: &Competition,
) -> Result<Message, RequestError> {
    bot.send_video(competition.get_chat_id(), InputFile::file(video))
        .caption(caption)
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
        sleep(Duration::from_secs(10)).await;
    }
    send_video_result
}

// We do not have a proper way to get the messageid of the last message in a group
// As a workaround we send a message, get the message_id of that message and then immediately delete it
pub async fn get_latest_message_id_of_group(bot: &Bot, chat_id: ChatId) -> MessageId {
    let message = bot
        .send_message(chat_id, ".")
        .send()
        .await
        .expect("Failed to send message");
    bot.delete_message(chat_id, message.id)
        .send()
        .await
        .expect("Failed to delete message");
    MessageId(message.id.0 - 1)
}

pub async fn reply_with_retries(
    bot: &Bot,
    message: &str,
    chat_id: ChatId,
    reply_to_message_id: MessageId,
) {
    let mut latest_message_id = reply_to_message_id.clone();

    loop {
        let message = bot
            .send_message(chat_id, message)
            .parse_mode(ParseMode::Html)
            .reply_to_message_id(latest_message_id)
            .send()
            .await;

        if message.is_ok() {
            break;
        };

        latest_message_id = MessageId(latest_message_id.0 - 1);
    }
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
    async fn test_send_message() {
        dotenv().ok();
        let config = Arc::new(Config::init());
        let bot = Arc::new(Bot::from_env());

        let latest_message_id =
            get_latest_message_id_of_group(&bot, config.premier_league.get_chat_id_replies()).await;
        reply_with_retries(
            &bot,
            "test2",
            config.premier_league.get_chat_id_replies(),
            latest_message_id,
        )
        .await;

        assert_eq!(true, true);
    }

    #[tokio::test]
    #[ignore]
    async fn test_() {
        dotenv().ok();
        let config = Arc::new(Config::init());
        let bot = Arc::new(Bot::from_env());

        let latest_message_id =
            get_latest_message_id_of_group(&bot, config.bundesliga.get_chat_id_replies()).await;

        println!("latest_message_id: {:?}", latest_message_id);
        assert_eq!(true, true);
    }

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
}
