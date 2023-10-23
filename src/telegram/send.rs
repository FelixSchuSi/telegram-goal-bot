use std::time::Duration;

use log::error;
use reqwest::Url;
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
    scraped_url: &Url,
    competition: &Competition,
) -> Result<Message, RequestError> {
    bot.send_video(
        competition.get_chat_id(),
        InputFile::url(scraped_url.clone()),
    )
    .caption(caption)
    .send()
    .await
}

pub async fn send_video_with_retries(
    caption: &str,
    bot: &Bot,
    scraped_url: &Url,
    competition: &Competition,
) -> Result<Message, RequestError> {
    let mut send_video_result = send_video(caption, bot, scraped_url, competition).await;
    if send_video_result.is_ok() {
        return send_video_result;
    }

    for i in 2..10 {
        send_video_result = send_video(caption, bot, scraped_url, competition)
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
    use crate::config::config::Config;
    use dotenv::dotenv;
    use std::sync::Arc;

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

        let res = send_video_with_retries(
            "test",
            &bot,
            &Url::parse("https://dubzalt.com/storage/videos/ce7d4c.mp4").unwrap(),
            &config.premier_league,
        )
        .await;

        println!("res: {:?}", res.unwrap_err());
    }
}
