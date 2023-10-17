use log::info;
use teloxide::{
    payloads::SendMessageSetters,
    requests::{Request, Requester},
    types::{Message, ParseMode},
    Bot,
};

use crate::filter::competition::Competition;

pub async fn send_link(caption: &str, bot: &Bot, url: &str, competition: &Competition) -> Message {
    info!(
        "🟨 SENDING MESSAGE: title:\"{}\" OG link: \"{}\"",
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
