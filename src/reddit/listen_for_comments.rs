use crate::{
    config::config::Config, filter::competition::CompetitionName,
    telegram::send::send_message_direct, GoalSubmission,
};

use futures_util::StreamExt;
use log::info;
use roux::Subreddit;
use roux_stream::stream_comments;
use std::{
    sync::{Arc, Mutex},
    time::Duration,
};
use teloxide::Bot;
use tokio_retry::strategy::ExponentialBackoff;

pub async fn listen_for_comments(
    subreddit: Arc<Subreddit>,
    bot: Arc<Bot>,
    config: Arc<Config>,
    listen_for_replays_submission_ids: Arc<Mutex<Vec<GoalSubmission>>>,
) {
    // let config = Arc::new(Config::init());
    let (mut stream, join_handle) = stream_comments(
        &subreddit,
        Duration::from_secs(5),
        ExponentialBackoff::from_millis(5).factor(100).take(3),
        Some(Duration::from_secs(10)),
    );

    while let Some(comment) = stream.next().await {
        // `comment` is an `Err` if getting the latest comments
        // from Reddit failed even after retrying.
        let comment = comment.unwrap();
        let id = match comment.link_id {
            None => continue,
            Some(id) => id.replace("t3_", ""),
        };

        let body = comment.body.unwrap_or("Replay".to_string()).clone();
        info!("considering comment {}: {}", id, &body);

        let goal_submission: Option<GoalSubmission>;
        goal_submission = listen_for_replays_submission_ids
            .lock()
            .unwrap()
            .iter()
            .find(|goal_submission| goal_submission.submission_id == id)
            .map(|gs| gs.clone());

        match goal_submission {
            None => continue,
            Some(goal_submission) => {
                let competition = match goal_submission.competition.clone() {
                    CompetitionName::Bundesliga => &config.bundesliga,
                    CompetitionName::ChampionsLeague => &config.champions_league,
                    CompetitionName::PremierLeague => &config.premier_league,
                    CompetitionName::Internationals => &config.internationals,
                };
                info!(
                    "sending video with id: {} competition: {:?}",
                    goal_submission.submission_id, goal_submission.competition
                );

                send_message_direct(&body, &bot, competition).await;
            }
        }
    }

    join_handle
        .await
        .expect("Error getting data from reddit while streaming comments")
        .expect("Received SendError from reddit while streaming comments");
}
