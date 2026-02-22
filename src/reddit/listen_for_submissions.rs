use super::log_roux_err::log_roux_err;
use crate::{
    config::config::Config,
    download_video::download_video::download_video_with_retries,
    filter::{
        competition::Competition, get_competitions_of_submission::get_competitions_of_submission,
    },
    reddit::get_fresh_subreddit::{get_fresh_subreddit, RedditError},
    scrape::scrape::scrape_video,
    telegram::{send::send_video_with_retries, send_link::send_link},
    GoalSubmission,
};
use chrono::{DateTime, Local, Utc};
use futures_util::StreamExt;
use log::{error, info, warn};
use roux::{submission::SubmissionData, Subreddit};
use roux_stream::stream_submissions;
use std::{fs::remove_file, time::Duration};
use teloxide::Bot;
use tokio_retry::strategy::ExponentialBackoff;

// Token is valid for 24h, but we refresh when less than 1h remains
const TOKEN_VALIDITY_DURATION: Duration = Duration::from_secs(24 * 60 * 60); // 24 hours
const TOKEN_REFRESH_THRESHOLD: Duration = Duration::from_secs(60 * 60); // 1 hour

pub struct RedditHandle {
    pub subreddit: Subreddit,
    pub bot: Bot,
    pub config: Config,
    pub listen_for_replays_submission_ids: Vec<GoalSubmission>,
    pub token_created_at: DateTime<Utc>,
}

impl RedditHandle {
    pub async fn new() -> Self {
        let subreddit: Subreddit = get_fresh_subreddit().await.unwrap();
        let config = Config::init();
        let bot = Bot::from_env();

        RedditHandle {
            subreddit,
            bot,
            config,
            listen_for_replays_submission_ids: Vec::new(),
            token_created_at: Local::now().into(),
        }
    }

    /// Check if the token needs to be refreshed (less than 1 hour validity remaining)
    fn should_refresh_token(&self) -> bool {
        let elapsed = Utc::now().signed_duration_since(self.token_created_at);
        let elapsed_std = Duration::from_secs(elapsed.num_seconds().max(0) as u64);

        // Calculate remaining time
        let remaining = TOKEN_VALIDITY_DURATION.saturating_sub(elapsed_std);

        remaining < TOKEN_REFRESH_THRESHOLD
    }

    /// Refresh the Reddit client and subreddit
    async fn refresh_token(&mut self) -> Result<(), RedditError> {
        info!("Refreshing Reddit OAuth token...");

        self.subreddit = get_fresh_subreddit().await?;
        self.token_created_at = Utc::now();

        info!("Successfully refreshed Reddit OAuth token");
        Ok(())
    }

    /// Check and refresh token if needed before making API calls
    async fn ensure_valid_token(&mut self) -> Result<(), RedditError> {
        if self.should_refresh_token() {
            warn!("Token expiring soon, refreshing...");
            self.refresh_token().await?;
        }
        Ok(())
    }

    pub async fn listen_for_submissions(&mut self) {
        loop {
            // Ensure we have a valid token before starting the stream
            if let Err(_e) = self.ensure_valid_token().await {
                error!("Failed to refresh token. Retrying in 60 seconds...");
                tokio::time::sleep(Duration::from_secs(60)).await;
                continue;
            }

            let (mut stream, join_handle) = stream_submissions(
                &self.subreddit,
                Duration::from_secs(5),
                ExponentialBackoff::from_millis(5).factor(100).take(3),
                Some(Duration::from_secs(10)),
            );

            while let Some(submission) = stream.next().await {
                if let Err(_e) = self.ensure_valid_token().await {
                    error!("Failed to refresh token during stream");
                    break; // Break inner loop to recreate stream with new token
                }

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
                                e, submission.title, scraped_url, url
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

            // Clean up the join handle
            match join_handle.await {
                Ok(Ok(())) => {
                    info!("Stream ended normally, restarting with fresh token...");
                }
                Ok(Err(e)) => {
                    error!(
                        "Received SendError from reddit while streaming submissions: {:?}",
                        e
                    );
                }
                Err(e) => {
                    error!(
                        "Error getting data from reddit while streaming submissions: {:?}",
                        e
                    );
                }
            }

            // Wait a bit before restarting to avoid hammering the API
            tokio::time::sleep(Duration::from_secs(30)).await;
        }
    }

    async fn register_message_for_replays(
        &mut self,
        competition: &Competition,
        submission: &SubmissionData,
    ) {
        info!("After sending video the messageId of the video was successfully saved - submission_title: {:?}, submission_id: {:?}",  submission.title, submission.id);
        self.listen_for_replays_submission_ids.push(GoalSubmission {
            submission_id: submission.id.clone(),
            competition: competition.name.clone(),
            added_time: chrono::offset::Local::now(),
        });
    }

    fn check_if_submission_was_already_posted(
        &mut self,
        competition: &Competition,
        submission: &SubmissionData,
    ) -> bool {
        let result = self
            .listen_for_replays_submission_ids
            .iter()
            .find(|e| e.submission_id == submission.id && e.competition == competition.name);
        result.is_some()
    }
}
