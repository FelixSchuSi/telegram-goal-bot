use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use crate::{
    config::config::Config,
    filter::{
        competition::CompetitionName,
        get_competitions_of_submission::get_competitions_of_submission,
    },
    scrape::scrape::scrape_video,
    telegram::{
        send::{get_latest_message_id_of_group, send_video_with_retries},
        send_link::send_link,
    },
    GoalSubmission,
};
use futures_util::StreamExt;
use log::{error, info};

use roux::Subreddit;
use roux_stream::stream_submissions;
use teloxide::Bot;

use tokio_retry::strategy::ExponentialBackoff;

use super::log_roux_err::log_roux_err;

pub struct RedditHandle {
    pub subreddit: Subreddit,
    pub bot: Arc<Bot>,
    pub config: Arc<Config>,
    pub listen_for_replays_submission_ids: Arc<Mutex<Vec<GoalSubmission>>>,
}

impl RedditHandle {
    pub async fn listen_for_submissions(&mut self) {
        let (mut stream, join_handle) = stream_submissions(
            &self.subreddit,
            Duration::from_secs(5),
            ExponentialBackoff::from_millis(5).factor(100).take(3),
            Some(Duration::from_secs(10)),
        );

        while let Some(submission) = stream.next().await {
            // `submission` is an `Err` if getting the latest submissions
            // from Reddit failed even after retrying.
            let Ok(submission) = submission else {
                log_roux_err(submission.unwrap_err()).await;
                continue;
            };

            let url = match &submission.url {
                Some(property) => property,
                None => continue,
            };

            for competition in get_competitions_of_submission(&submission, &self.config) {
                let scrape_result = scrape_video(url).await;
                if scrape_result.is_err() {
                    error!(
                        "Scraping failed: {} submission.title: {} url: {}",
                        scrape_result.unwrap_err().0,
                        submission.title,
                        url
                    );
                    continue;
                }
                let scraped_url = scrape_result.unwrap();
                let send_video_result = send_video_with_retries(
                    &submission.title,
                    &self.bot,
                    &scraped_url,
                    &competition,
                )
                .await;

                if send_video_result.is_err() {
                    error!(
                        "Sending video failed in all 10 tries. Sending Link instead. Error: {} submission.title: {} url: {}",
                        send_video_result.unwrap_err(),
                        submission.title,
                        url
                    );
                    send_link(&submission.title, &self.bot, url, &competition).await;
                }

                let reply_id =
                    get_latest_message_id_of_group(&&self.bot, competition.get_chat_id_replies())
                        .await
                        .0;
                info!("After sending video the messageId of the video was successfully saved - MessageId: {:?}, submission_title: {:?}, submission_id: {:?}", reply_id, submission.title, submission.id);
                self.listen_for_replays_submission_ids
                    .lock()
                    .unwrap()
                    .push(GoalSubmission {
                        submission_id: submission.id.clone(),
                        competition: CompetitionName::ChampionsLeague,
                        sent_comment_ids: Vec::new(),
                        reply_id,
                        added_time: chrono::offset::Local::now(),
                    });
            }
        }

        join_handle
            .await
            .expect("Error getting data from reddit while streaming submissions")
            .expect("Received SendError from reddit while streaming submissions");
    }
}
