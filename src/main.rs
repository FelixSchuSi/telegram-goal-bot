use crate::{filter::filter::submission_filter, telegram::send::send_video};
mod config;
mod filter;
mod replay;
mod scrape;
mod telegram;
use config::config::Config;
use dotenv::dotenv;
use futures_util::stream::StreamExt;
use log::error;
use roux::Subreddit;
use roux_stream::stream_submissions;
use std::time::Duration;
use teloxide::Bot;

use tokio_retry::strategy::ExponentialBackoff;

#[tokio::main]
async fn main() {
    env_logger::init();
    dotenv().ok();
    let config = Config::init();
    let subreddit = Subreddit::new("soccer");
    let bot = Bot::from_env();

    // send_reply(
    //     &bot,
    //     "schönes Tor",
    //     ChatId(-1000000000000 - 1273971265),
    //     MessageId(34),
    // )
    // .await;
    let (mut stream, join_handle) = stream_submissions(
        &subreddit,
        Duration::from_secs(5),
        ExponentialBackoff::from_millis(5).factor(100).take(3),
        Some(Duration::from_secs(10)),
    );

    while let Some(submission) = stream.next().await {
        // `submission` is an `Err` if getting the latest submissions
        // from Reddit failed even after retrying.
        let Ok(submission) = submission else {
            error!("Error getting submission: {}", submission.unwrap_err());
            continue;
        };
        let url = match &submission.url {
            Some(property) => property,
            None => continue,
        };

        if submission_filter(&submission, &config.champions_league) {
            send_video(&submission.title, &bot, url, &config.champions_league).await;
        }
        if submission_filter(&submission, &config.bundesliga) {
            send_video(&submission.title, &bot, url, &config.bundesliga).await;
        }
        if submission_filter(&submission, &config.internationals) {
            send_video(&submission.title, &bot, url, &config.internationals).await;
        }
        if submission_filter(&submission, &config.premier_league) {
            send_video(&submission.title, &bot, url, &config.premier_league).await;
        }
    }

    join_handle
        .await
        .expect("Error getting data from reddit")
        .expect("Received SendError from reddit");
}
