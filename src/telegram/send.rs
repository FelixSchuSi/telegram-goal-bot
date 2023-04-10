use reqwest::Url;
use teloxide::{
    payloads::{SendMessageSetters, SendVideoSetters},
    requests::{Request, Requester},
    types::{ChatId, InputFile, MessageId, ParseMode},
    Bot,
};

use crate::{filter::competition::Competition, scrape::scrape::scrape_video};

pub async fn send_video(caption: &str, bot: &Bot, url: &str, competition: &Competition) {
    let scraped_url = scrape_video(String::clone(&url.to_string())).await;

    if scraped_url.is_err() {
        send_message(caption, bot, url, competition).await;
        return;
    }

    let input_file = InputFile::url(Url::parse(&scraped_url.unwrap()).expect("invalid url"));

    bot.send_video(competition.get_chat_id(), input_file)
        .caption(caption)
        .send()
        .await
        .expect("Failed to send video");
}

pub async fn send_message(caption: &str, bot: &Bot, url: &str, competition: &Competition) {
    println!(
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
    .expect("Failed to send message");
}

pub async fn send_reply(bot: &Bot, message: &str, chat_id: ChatId, reply_to_message_id: MessageId) {
    println!("🟩 SENDING REPLY: {}", message);
    bot.send_message(chat_id, message)
        .parse_mode(ParseMode::Html)
        .reply_to_message_id(reply_to_message_id)
        .send()
        .await
        .expect("Failed to send message");
}
