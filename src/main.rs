use crate::reddit::listen_for_submissions::listen_for_submissions;
mod config;
mod filter;
mod reddit;
mod replay;
mod scrape;
mod telegram;
use config::config::Config;
use dotenv::dotenv;
use filter::competition::CompetitionName;
use reddit::listen_for_comments::listen_for_comments;
use roux::Subreddit;
use std::sync::{Arc, Mutex};
use teloxide::Bot;

#[derive(Debug, Clone)]
pub struct GoalSubmission {
    pub submission_id: String,
    pub competition: CompetitionName,
}

#[tokio::main]
async fn main() {
    env_logger::init();
    dotenv().ok();
    let config = Arc::new(Config::init());
    let subreddit = Arc::new(Subreddit::new("soccer"));
    let bot = Arc::new(Bot::from_env());

    // List of submissions that are goals and were posted to telegram.
    // We want to listen for comments for this submission to find replays of that goal to post them to telegram as well.
    let listen_for_replays_submission_ids: Arc<Mutex<Vec<GoalSubmission>>> =
        Arc::new(Mutex::new(Vec::new()));

    let (subreddit_cloned, bot_cloned, config_cloned, listen_for_replays_submission_ids_cloned) = (
        Arc::clone(&subreddit),
        Arc::clone(&bot),
        Arc::clone(&config),
        Arc::clone(&listen_for_replays_submission_ids),
    );
    let submissions_join_handler = tokio::spawn(async {
        listen_for_submissions(
            subreddit_cloned,
            bot_cloned,
            config_cloned,
            listen_for_replays_submission_ids_cloned,
        )
        .await;
    });

    let (subreddit_cloned, bot_cloned, config_cloned, listen_for_replays_submission_ids_cloned) = (
        Arc::clone(&subreddit),
        Arc::clone(&bot),
        Arc::clone(&config),
        Arc::clone(&listen_for_replays_submission_ids),
    );
    let comments_join_handler = tokio::spawn(async {
        listen_for_comments(
            subreddit_cloned,
            bot_cloned,
            config_cloned,
            listen_for_replays_submission_ids_cloned,
        )
        .await;
    });

    submissions_join_handler
        .await
        .expect("Panic while handling submissions");

    comments_join_handler
        .await
        .expect("Panic while handling comments");
}
