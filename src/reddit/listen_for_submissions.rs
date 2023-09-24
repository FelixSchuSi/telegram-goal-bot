use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use futures_util::StreamExt;
use log::{error, info};
use roux::Subreddit;
use roux::util::RouxError;
use roux_stream::{stream_submissions, StreamError};
use teloxide::Bot;
use tokio::time::sleep;
use tokio_retry::strategy::ExponentialBackoff;

use crate::{
    config::config::Config,
    filter::{competition::CompetitionName, filter::submission_filter},
    GoalSubmission,
    telegram::send::{get_latest_message_id_of_group, send_video},
};

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
                let err = submission.unwrap_err();
                error!("Error getting submission: {}", err);
                match err {
                    StreamError::TimeoutError(timeout) => {
                        print!("is StreamError");
                        error!("timeout: {}", timeout);
                        print!("is StreamError");
                    }
                    StreamError::SourceError(e) => {
                        print!("is SourceError");
                        error!("rouxerror: {}", e);
                        match e {
                            RouxError::Network(netE) => {
                                print!("is RouxError::Network");
                                error!("Network: {}", netE);
                            }
                            RouxError::Status(statE) => {
                                print!("is RouxError::Status");
                                error!("Status: ");
                            }
                            RouxError::Parse(parseE) => {
                                print!("is RouxError::Parse");
                                error!("Parse: {}", parseE);
                            }
                            RouxError::Auth(e) => {
                                print!("is RouxError::Auth");
                                error!("Auth: {}", e);
                            }
                        }
                        print!("is SourceError");
                    }
                }
                print!("ich bin hier lol");
                continue;
            };
            let url = match &submission.url {
                Some(property) => property,
                None => continue,
            };
            if submission_filter(&submission, &self.config.champions_league) {
                send_video(
                    &submission.title,
                    &self.bot,
                    url,
                    &self.config.champions_league,
                )
                    .await;
                sleep(Duration::from_secs(20)).await;
                let reply_id = get_latest_message_id_of_group(
                    &&self.bot,
                    self.config.champions_league.get_chat_id_replies(),
                )
                    .await
                    .0;
                info!(
                        "MessageId found of submission - MessageId: {:?}, submission_title: {:?}, submission_id: {:?}",
                        reply_id, submission.title, submission.id
                    );
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
            if submission_filter(&submission, &self.config.bundesliga) {
                send_video(&submission.title, &self.bot, url, &self.config.bundesliga).await;
                sleep(Duration::from_secs(20)).await;
                let reply_id = get_latest_message_id_of_group(
                    &self.bot,
                    self.config.bundesliga.get_chat_id_replies(),
                )
                    .await
                    .0;
                info!(
                        "MessageId found of submission - MessageId: {:?}, submission_title: {:?}, submission_id: {:?}",
                        reply_id, submission.title, submission.id
                    );
                self.listen_for_replays_submission_ids
                    .lock()
                    .unwrap()
                    .push(GoalSubmission {
                        submission_id: submission.id.clone(),
                        competition: CompetitionName::Bundesliga,
                        sent_comment_ids: Vec::new(),
                        reply_id,
                        added_time: chrono::offset::Local::now(),
                    });
            }
            if submission_filter(&submission, &self.config.internationals) {
                send_video(
                    &submission.title,
                    &self.bot,
                    url,
                    &self.config.internationals,
                )
                    .await;
                sleep(Duration::from_secs(20)).await;
                let reply_id = get_latest_message_id_of_group(
                    &self.bot,
                    self.config.internationals.get_chat_id_replies(),
                )
                    .await
                    .0;
                info!(
                        "MessageId found of submission - MessageId: {:?}, submission_title: {:?}, submission_id: {:?}",
                        reply_id, submission.title, submission.id
                    );
                self.listen_for_replays_submission_ids
                    .lock()
                    .unwrap()
                    .push(GoalSubmission {
                        submission_id: submission.id.clone(),
                        competition: CompetitionName::Internationals,
                        sent_comment_ids: Vec::new(),
                        reply_id,
                        added_time: chrono::offset::Local::now(),
                    });
            }
            if submission_filter(&submission, &self.config.premier_league) {
                send_video(
                    &submission.title,
                    &self.bot,
                    url,
                    &self.config.premier_league,
                )
                    .await;
                sleep(Duration::from_secs(20)).await;
                let reply_id = get_latest_message_id_of_group(
                    &self.bot,
                    self.config.premier_league.get_chat_id_replies(),
                )
                    .await
                    .0;
                info!(
                    "MessageId found of submission - MessageId: {:?}, submission_title: {:?}, submission_id: {:?}",
                    reply_id, submission.title, submission.id
                );
                self.listen_for_replays_submission_ids
                    .lock()
                    .unwrap()
                    .push(GoalSubmission {
                        submission_id: submission.id.clone(),
                        competition: CompetitionName::PremierLeague,
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
