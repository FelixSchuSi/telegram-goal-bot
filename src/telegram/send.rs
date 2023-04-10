use reqwest::Url;
use roux::subreddit::responses::SubmissionsData;
use teloxide::{
    payloads::{SendMessageSetters, SendVideoSetters},
    requests::{Request, Requester},
    types::{InputFile, ParseMode},
    Bot,
};

use crate::{filter::competition::Competition, scrape::scrape::scrape_video};

pub async fn send_video(
    submission: &SubmissionsData,
    bot: &Bot,
    url: &str,
    competition: &Competition,
) {
    let scraped_url = scrape_video(String::clone(&url.to_string())).await;

    if scraped_url.is_err() {
        send_message(submission, bot, url, competition).await;
        return;
    }

    let input_file = InputFile::url(Url::parse(&scraped_url.unwrap()).expect("invalid url"));

    bot.send_video(competition.get_chat_id(), input_file)
        .caption(submission.title.to_owned())
        .send()
        .await
        .is_err()
        .then(|| send_message(submission, bot, url, competition));
}

pub async fn send_message(
    submission: &SubmissionsData,
    bot: &Bot,
    url: &str,
    competition: &Competition,
) {
    println!(
        "🟩 SENDING MESSAGE: title:\"{}\" OG link: \"{}\"",
        submission.title, url
    );
    bot.send_message(
        competition.get_chat_id(),
        format!(
            "<b><a href=\"{}\">{}</a></b>",
            url,
            submission.title.to_owned()
        ),
    )
    .parse_mode(ParseMode::Html)
    .send()
    .await
    .expect("Failed to send message");
}
