use chrono::{DateTime, Utc};
use config::config::Config;
use dotenv::dotenv;
use filter::competition::CompetitionName;
use filter::get_competitions_of_submission::get_competitions_of_submission;
use log::info;
use reddit_scrape::scrape_reddit_submission::{scrape_submissions, RedditSubmission};
use std::io::Write;
use std::time::Duration;
use telegram::send_link::send_link;
use teloxide::Bot;
use tokio::time::{self, Interval};
mod config;
mod download_video;
mod filter;
mod reddit_scrape;
mod scrape;
mod telegram;

#[derive(Debug, Clone)]
pub struct TelegramSubmission {
    pub reddit_submission: RedditSubmission,
    pub competition: CompetitionName,
    pub sent_comment_ids: Vec<String>,
    pub telegram_reply_id: i32,
}

#[derive(Debug)]
pub struct TelegramGoalBot {
    pub posted_telegram_submissions: Vec<TelegramSubmission>,
    pub config: Config,
    pub interval: Interval,
    pub telegram_bot: Bot,
}

#[tokio::main]
async fn main() {
    println!("Starting TelegramGoalBot init");
    info!("Starting TelegramGoalBot init");
    let mut bot = TelegramGoalBot::init();
    println!("TelegramGoalBot init successful!");
    info!("TelegramGoalBot init successful!");

    loop {
        bot.interval.tick().await;

        let scraped_submissions = scrape_submissions().await.unwrap();

        info!("{:?}", scraped_submissions);

        let mut filtered_submissions = bot.filter_submissions(scraped_submissions);

        info!("{:?}", filtered_submissions);

        for submission in filtered_submissions.clone() {
            let comp = bot
                .config
                .clone()
                .into_iter()
                .find(|e| e.name == submission.competition)
                .unwrap();

            send_link(
                &submission.reddit_submission.title,
                &bot.telegram_bot,
                &submission.reddit_submission.url,
                &comp,
            )
            .await;
        }

        bot.posted_telegram_submissions
            .append(&mut filtered_submissions);

        bot.cleanup_posted_telegram_submissions();
    }
}

impl TelegramGoalBot {
    pub fn init() -> TelegramGoalBot {
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
        let interval = time::interval(Duration::from_secs(30));
        return TelegramGoalBot {
            config: Config::init(),
            posted_telegram_submissions: Vec::new(),
            interval,
            telegram_bot: Bot::from_env(),
        };
    }

    pub fn filter_submissions(
        &self,
        reddit_submissions: Vec<RedditSubmission>,
    ) -> Vec<TelegramSubmission> {
        return reddit_submissions
            .iter()
            .filter_map(|submission| self.filter_submission(submission.to_owned()))
            .flat_map(|reddit_submission| {
                let tmp: Vec<TelegramSubmission> =
                    get_competitions_of_submission(&reddit_submission, &self.config)
                        .iter()
                        .map(|competition| TelegramSubmission {
                            reddit_submission: reddit_submission.clone(),
                            competition: competition.name.clone(),
                            sent_comment_ids: Vec::new(),
                            telegram_reply_id: 0,
                        })
                        .collect();
                return tmp;
            })
            .collect();
    }

    fn filter_submission(&self, submission: RedditSubmission) -> Option<RedditSubmission> {
        let comps = get_competitions_of_submission(&submission, &self.config);
        if comps.len() == 0 {
            return None;
        }

        if self
            .posted_telegram_submissions
            .iter()
            .find(|e| e.reddit_submission.id == submission.id)
            .is_some()
        {
            return None;
        }

        return Some(submission);
    }

    pub fn cleanup_posted_telegram_submissions(&mut self) {
        self.posted_telegram_submissions = self
            .posted_telegram_submissions
            .iter()
            .filter(|submission| {
                DateTime::from_timestamp(submission.reddit_submission.created_utc, 0)
                    .map_or(false, |datetime| {
                        datetime > Utc::now() - Duration::from_secs(60 * 60)
                    })
            })
            .map(|submission| submission.to_owned())
            .collect()
    }
}
