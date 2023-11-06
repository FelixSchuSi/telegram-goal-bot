use super::log_roux_err::log_roux_err;
use crate::{
    config::config::Config,
    download_video::download_video::download_video_with_retries,
    filter::{
        competition::Competition, get_competitions_of_submission::get_competitions_of_submission,
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
use roux::{submission::SubmissionData, Subreddit};
use roux_stream::stream_submissions;
use std::{
    fs::remove_file,
    sync::{Arc, Mutex},
    time::Duration,
};
use teloxide::Bot;
use tokio_retry::strategy::ExponentialBackoff;

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
                if self.check_if_submission_was_already_posted(&competition, &submission) {
                    info!(
                        "Submission was already posted. Skipping this submission. submission.title: {} submission.id: {} url: {}",
                        submission.title, submission.id, url
                    );
                    continue;
                }
                let scraped_url = match scrape_video(url).await {
                    Ok(scrape_result) => scrape_result,
                    Err(e) => {
                        error!(
                            "Scraping failed: {} submission.title: {} url: {}",
                            e.0, submission.title, url,
                        );
                        send_link(&submission.title, &self.bot, url, &competition).await;
                        self.register_message_for_replays(&competition, &submission)
                            .await;
                        continue;
                    }
                };

                let download_video = match download_video_with_retries(&scraped_url.to_string())
                    .await
                {
                    Ok(download_video) => download_video,
                    Err(e) => {
                        error!(
                            "Downloading video failed in all 10 tries. Error: {} submission.title: {} scraped_url: {} url: {}",
                            e,submission.title,                            scraped_url,                            url
                        );
                        send_link(&submission.title, &self.bot, url, &competition).await;
                        self.register_message_for_replays(&competition, &submission)
                            .await;
                        continue;
                    }
                };

                match send_video_with_retries(
                    &submission.title,
                    &self.bot,
                    &download_video,
                    &competition,
                )
                .await
                {
                    Ok(video_message) => {
                        info!(
                            "Sending video was successfull. submission.title: {} url: {} message_id: {}",
                            submission.title, url, video_message.id.0
                        );
                        remove_file(download_video).expect("Failed to delete file");
                    }
                    Err(e) => {
                        error!(
                            "Sending video failed in all 10 tries. Sending Link instead. Error: {} submission.title: {} scraped_url: {} url: {}",
                            e,
                            submission.title,
                            scraped_url,
                            url
                        );
                        send_link(&submission.title, &self.bot, url, &competition).await;
                    }
                };
                self.register_message_for_replays(&competition, &submission)
                    .await;
            }
        }

        join_handle
            .await
            .expect("Error getting data from reddit while streaming submissions")
            .expect("Received SendError from reddit while streaming submissions");
    }

    async fn register_message_for_replays(
        &mut self,
        competition: &Competition,
        submission: &SubmissionData,
    ) {
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
                competition: competition.name.clone(),
                sent_comment_ids: Vec::new(),
                reply_id,
                added_time: chrono::offset::Local::now(),
            });
    }

    fn check_if_submission_was_already_posted(
        &mut self,
        competition: &Competition,
        submission: &SubmissionData,
    ) -> bool {
        let submissions = self.listen_for_replays_submission_ids.lock().unwrap();
        let result = submissions
            .iter()
            .find(|e| e.submission_id == submission.id && e.competition == competition.name);
        result.is_some()
    }
}
