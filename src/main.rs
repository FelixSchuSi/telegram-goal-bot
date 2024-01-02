use chrono::{DateTime, Local};
use config::config::Config;
use dotenv::dotenv;
use filter::competition::CompetitionName;
use futures_util::future;
use log::info;
use reddit::listen_for_submissions::RedditHandle;
use roux::{Reddit, Subreddit};
use std::env;
use std::io::Write;
use std::sync::{Arc, Mutex};
use teloxide::Bot;
mod config;
mod download_video;
mod filter;
mod reddit;
mod scrape;
mod telegram;

#[derive(Debug, Clone)]
pub struct GoalSubmission {
    pub submission_id: String,
    pub competition: CompetitionName,
    pub sent_comment_ids: Vec<String>,
    pub reply_id: i32,
    pub added_time: DateTime<Local>,
}

#[tokio::main]
async fn main() {
    env_logger::Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "[{}:{} {}] {}",
                record.file().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                record.level(),
                record.args()
            )
        })
        .init();
    dotenv().ok();
    info!("successfully read dotenv");

    // List of submissions that are goals and were posted to telegram.
    // We want to listen for comments for this submission to find replays of that goal to post them to telegram as well.
    let listen_for_replays_submission_ids: Arc<Mutex<Vec<GoalSubmission>>> =
        Arc::new(Mutex::new(Vec::new()));

    let mut reddit_handle_submissions =
        create_reddit_handle(Arc::clone(&listen_for_replays_submission_ids)).await;
    let mut reddit_handle_comments =
        create_reddit_handle(Arc::clone(&listen_for_replays_submission_ids)).await;

    info!("reddit comment handle and reddit submission handle successfully started");

    future::join(
        reddit_handle_submissions.listen_for_submissions(),
        reddit_handle_comments.search_for_alternative_angles_in_submission_comments(),
    )
    .await;
}

async fn create_reddit_handle(
    listen_for_replays_submission_ids: Arc<Mutex<Vec<GoalSubmission>>>,
) -> RedditHandle {
    let client = Reddit::new(
        &env::var("REDDIT_USER_AGENT").expect("REDDIT_USER_AGENT must be present"),
        &env::var("REDDIT_CLIENT_ID").expect("REDDIT_CLIENT_ID must be present"),
        &env::var("REDDIT_CLIENT_SECRET").expect("REDDIT_CLIENT_SECRET must be present"),
    )
    .username(&env::var("REDDIT_USERNAME").expect("REDDIT_USERNAME must be present"))
    .password(&env::var("REDDIT_PASSWORD").expect("REDDIT_PASSWORD must be present"))
    .login()
    .await
    .unwrap();
    let config = Arc::new(Config::init());
    let bot = Arc::new(Bot::from_env());

    let subreddit = Subreddit::new_oauth("soccer", &client.client);

    RedditHandle {
        subreddit,
        bot,
        config,
        listen_for_replays_submission_ids: Arc::clone(&listen_for_replays_submission_ids),
    }
}
