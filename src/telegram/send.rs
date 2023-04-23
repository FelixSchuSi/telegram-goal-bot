use crate::{filter::competition::Competition, scrape::scrape::scrape_video};
use log::{error, info};
use reqwest::Url;
use teloxide::{
    payloads::{SendMessageSetters, SendVideoSetters},
    requests::{Request, Requester},
    types::{ChatId, InputFile, Message, MessageId, ParseMode},
    Bot,
};

pub async fn send_video(caption: &str, bot: &Bot, url: &str, competition: &Competition) -> Message {
    let scraped_url = scrape_video(String::clone(&url.to_string())).await;

    if scraped_url.is_err() {
        error!(
            "Scraping failed: {} caption: {} url: {}",
            scraped_url.err().unwrap().0,
            caption,
            url
        );
        return send_link(caption, bot, url, competition).await;
    }
    let scraped_url = scraped_url.unwrap();
    let scraped_parsed_url = Url::parse(&scraped_url);
    if scraped_parsed_url.is_err() {
        error!(
            "Scraping failed: {} scraped_url: {} caption: {} url: {}",
            scraped_parsed_url.err().unwrap(),
            scraped_url,
            caption,
            url
        );
        return send_link(caption, bot, url, competition).await;
    }
    let input_file = InputFile::url(scraped_parsed_url.expect("invalid url"));

    let msg = bot
        .send_video(competition.get_chat_id(), input_file)
        .caption(caption)
        .send()
        .await;

    if msg.is_err() {
        error!(
            "Scraping failed: {} scraped_url: {} caption: {} url: {}",
            msg.unwrap_err(),
            scraped_url,
            caption,
            url
        );
        return send_link(caption, bot, url, competition).await;
    }

    msg.unwrap()
}

pub async fn send_link(caption: &str, bot: &Bot, url: &str, competition: &Competition) -> Message {
    info!(
        "🟩 SENDING MESSAGE: title:\"{}\" OG link: \"{}\"",
        caption, url
    );
    bot.send_message(
        competition.get_chat_id(),
        format!("<b><a href=\"{}\">{}</a></b>", url, caption),
    )
    .parse_mode(ParseMode::Html)
    .send()
    .await
    .expect(&format!("Failed to send message {:?}", competition.name))
}

// We do not have a proper way to get the messageid of the last message in a group
// As a workaround we send a message, get the message_id of that message and then immediately delete it
pub async fn get_latest_message_id_of_group(bot: &Bot, chat_id: ChatId) -> MessageId {
    let message = bot
        .send_message(chat_id, "temp")
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
}
