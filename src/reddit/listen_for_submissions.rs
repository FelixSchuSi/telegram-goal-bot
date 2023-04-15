use futures_util::StreamExt;
use log::error;
use roux::Subreddit;
use roux_stream::stream_submissions;
use std::{
    sync::{Arc, Mutex},
    time::Duration,
};
use teloxide::Bot;
use tokio_retry::strategy::ExponentialBackoff;

use crate::{
    config::config::Config,
    filter::{competition::CompetitionName, filter::submission_filter},
    telegram::send::send_video,
    GoalSubmission,
};

pub async fn listen_for_submissions(
    subreddit: Arc<Subreddit>,
    bot: Arc<Bot>,
    config: Arc<Config>,
    listen_for_replays_submission_ids: Arc<Mutex<Vec<GoalSubmission>>>,
) {
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
            listen_for_replays_submission_ids
                .lock()
                .unwrap()
                .push(GoalSubmission {
                    submission_id: submission.id.clone(),
                    competition: CompetitionName::ChampionsLeague,
                });
        }
        if submission_filter(&submission, &config.bundesliga) {
            send_video(&submission.title, &bot, url, &config.bundesliga).await;
            listen_for_replays_submission_ids
                .lock()
                .unwrap()
                .push(GoalSubmission {
                    submission_id: submission.id.clone(),
                    competition: CompetitionName::Bundesliga,
                });
        }
        if submission_filter(&submission, &config.internationals) {
            send_video(&submission.title, &bot, url, &config.internationals).await;
            listen_for_replays_submission_ids
                .lock()
                .unwrap()
                .push(GoalSubmission {
                    submission_id: submission.id.clone(),
                    competition: CompetitionName::Internationals,
                });
        }
        if submission_filter(&submission, &config.premier_league) {
            send_video(&submission.title, &bot, url, &config.premier_league).await;
            listen_for_replays_submission_ids
                .lock()
                .unwrap()
                .push(GoalSubmission {
                    submission_id: submission.id.clone(),
                    competition: CompetitionName::PremierLeague,
                });
        }
    }

    join_handle
        .await
        .expect("Error getting data from reddit while streaming submissions")
        .expect("Received SendError from reddit while streaming submissions");
}