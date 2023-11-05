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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::config::Config;
    use dotenv::dotenv;
    use std::sync::Arc;

    #[tokio::test]
    #[ignore]
    async fn test_send_link() {
        dotenv().ok();
        let config = Arc::new(Config::init());
        let bot = Arc::new(Bot::from_env());
        let msg = send_link("test", &bot, "https://downloader.disk.yandex.ru/disk/dca64fefe808e120d267dddb8b9d9da90bdf8c514e6582831ec435c8e33810e7/6547c83b/MuDSbA9z5TnczT15nZM5tzpmO4vQDUpkdaDVD6JlOlA4PhQqRR3MxvyLDbUylX9tmeGJ68h9E34HcikwzEsRmw%3D%3D?uid=465360380&filename=01db19c9.mp4&disposition=attachment&hash=&limit=0&content_type=video%2Fmp4&owner_uid=465360380&fsize=872682&hid=c82b2feb994bc4d27e864df1ba19630a&media_type=video&tknv=v2&etag=3dbc418f39d7e288c07b9828a4e7e58d&expires=1699203131#t=0.1", &config.bundesliga_2).await;

        println!("msg: {:?}", msg);
    }
}
