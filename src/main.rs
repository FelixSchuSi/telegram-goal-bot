use chrono::{DateTime, Local};
use dotenv::dotenv;
use filter::competition::CompetitionName;
use log::info;
use reddit::listen_for_submissions::RedditHandle;
use std::io::Write;
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
    pub added_time: DateTime<Local>,
}

#[tokio::main]
async fn main() {
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Info)
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

    let mut reddit_handle = RedditHandle::new().await;
    info!("reddit submission handle successfully started");
    reddit_handle.listen_for_submissions().await;
}
