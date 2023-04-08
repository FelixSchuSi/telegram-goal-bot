use crate::{filter::filter::submission_filter, scrape::scrape::scrape_video};
mod config;
mod filter;
mod scrape;
use config::config::Config;
use futures_util::stream::StreamExt;

use log::{error, info};
use roux::Subreddit;
use roux_stream::stream_submissions;
use std::time::Duration;

use tokio_retry::strategy::ExponentialBackoff;

#[tokio::main]
async fn main() {
    env_logger::init();
    let config = Config::init();
    let subreddit = Subreddit::new("soccer");
    let retry_strategy = ExponentialBackoff::from_millis(5).factor(100).take(3);

    let (mut stream, join_handle) = stream_submissions(
        &subreddit,
        Duration::from_secs(5),
        retry_strategy,
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
            info!(
                "🟩 title:\"{}\" competition: champions_league scraped link: \"{}\" OG link: \"{}\"",
                submission.title,
                scrape_video(String::clone(&url)).await.expect(&url),
                url
            );
        }
        if submission_filter(&submission, &config.bundesliga) {
            info!(
                "🟩 title:\"{}\" competition: bundesliga scraped link: \"{}\" OG link: \"{}\"",
                submission.title,
                scrape_video(String::clone(&url)).await.expect(&url),
                url
            );
        }
        if submission_filter(&submission, &config.internationals) {
            info!(
                "🟩 title:\"{}\" competition: internationals scraped link: \"{}\" OG link: \"{}\"",
                submission.title,
                scrape_video(String::clone(&url)).await.expect(&url),
                url
            );
        }
        if submission_filter(&submission, &config.premier_league) {
            info!(
                "🟩 title:\"{}\" competition: premier_league scraped link: \"{}\" OG link: \"{}\"",
                submission.title,
                scrape_video(String::clone(&url)).await.expect(&url),
                url
            );
        }
    }

    // In case there was an error sending the submissions through the
    // stream, `join_handle` will report it.
    join_handle.await.unwrap().unwrap();
}
