use log::{error, info};
use reqwest::Url;
use teloxide::{
    payloads::{SendMessageSetters, SendVideoSetters},
    requests::{Request, Requester},
    types::{ChatId, InputFile, Message, MessageId, ParseMode},
    Bot,
};

use crate::{filter::competition::Competition, scrape::scrape::scrape_video};

pub async fn send_video(caption: &str, bot: &Bot, url: &str, competition: &Competition) -> Message {
    let scraped_url = scrape_video(String::clone(&url.to_string())).await;

    if scraped_url.is_err() {
        error!("Scraping failed: {}", scraped_url.err().unwrap().0);
        return send_message(caption, bot, url, competition).await;
    }
    let scraped_url = scraped_url.unwrap();
    let input_file = InputFile::url(Url::parse(&scraped_url).expect("invalid url"));

    let msg = bot
        .send_video(competition.get_chat_id(), input_file)
        .caption(caption)
        .send()
        .await;

    if msg.is_err() {
        error!("Scraping failed: {} {}", msg.unwrap_err(), scraped_url);
        return send_message(caption, bot, url, competition).await;
    }

    msg.unwrap()
}

pub async fn send_message(
    caption: &str,
    bot: &Bot,
    url: &str,
    competition: &Competition,
) -> Message {
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

pub async fn send_message_direct(content: &str, bot: &Bot, competition: &Competition) -> Message {
    info!("🟩 SENDING MESSAGE: title:\"{}", content);
    bot.send_message(competition.get_chat_id(), content)
        .send()
        .await
        .expect("Failed to send message")
}

#[allow(dead_code)]
pub async fn send_reply(bot: &Bot, message: &str, chat_id: ChatId, reply_to_message_id: MessageId) {
    info!("🟩 SENDING REPLY: {}", message);
    bot.send_message(chat_id, message)
        .parse_mode(ParseMode::Html)
        .reply_to_message_id(reply_to_message_id)
        .send()
        .await
        .expect("Failed to send message");
}