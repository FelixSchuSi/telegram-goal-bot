use crate::{
    config::config::Config, filter::competition::CompetitionName,
    telegram::send::send_message_direct, GoalSubmission,
};
use futures_util::StreamExt;
use log::info;
use roux::{responses::BasicThing, subreddit::responses::SubredditCommentsData, Subreddit};
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
        let id = match comment.link_id.clone() {
            None => continue,
            Some(id) => id.replace("t3_", ""),
        };

        let goal_submission: Option<GoalSubmission>;
        goal_submission = listen_for_replays_submission_ids
            .lock()
            .unwrap()
            .iter()
            .find(|goal_submission| goal_submission.submission_id == id)
            .map(|gs| gs.clone());

        if goal_submission.is_none() {
            continue;
        }
        let goal_submission = goal_submission.unwrap();
        let competition = match goal_submission.competition.clone() {
            CompetitionName::Bundesliga => &config.bundesliga,
            CompetitionName::ChampionsLeague => &config.champions_league,
            CompetitionName::PremierLeague => &config.premier_league,
            CompetitionName::Internationals => &config.internationals,
        };

        let all_comments_from_submission = subreddit
            .article_comments("article", Some(3), Some(200))
            .await;
        if all_comments_from_submission.is_err() {
            info!(
                "Error getting all comments for submission with id: {} competition: {:?}",
                goal_submission.submission_id, goal_submission.competition
            );
            continue;
        }
        let all_comments_from_submission = all_comments_from_submission.unwrap().data.children;
        if !is_comment_alternative_angle(&comment, &all_comments_from_submission) {
            info!(
                "Comment is not a alternative angle: {} competition: {:?}",
                comment
                    .body
                    .clone()
                    .unwrap_or("Comment has no body".to_string()),
                goal_submission.competition
            );
            continue;
        }
        info!(
            "sending video with id: {} competition: {:?}",
            goal_submission.submission_id, goal_submission.competition
        );

        send_message_direct(
            &comment.body.clone().unwrap_or("Replay".to_string()).clone(),
            &bot,
            competition,
        )
        .await;
    }

    join_handle
        .await
        .expect("Error getting data from reddit while streaming comments")
        .expect("Received SendError from reddit while streaming comments");
}

fn is_comment_alternative_angle<'a>(
    comment: &'a SubredditCommentsData,
    all_comments_from_submission: &Vec<BasicThing<SubredditCommentsData>>,
) -> bool {
    if is_single_comment_alternative_angle(comment) {
        return true;
    }
    if comment.parent_id.is_none() {
        return false;
    }
    let parent_id = comment.parent_id.clone().unwrap().clone();

    let parent_comment = all_comments_from_submission
        .iter()
        .find(|potential_parent_comment| {
            let potential_parent_id = potential_parent_comment
                .data
                .parent_id
                .as_ref()
                .unwrap_or(&"".to_string())
                .to_owned();
            potential_parent_id == parent_id
        });

    if parent_comment.is_none() {
        return false;
    }
    if parent_comment.unwrap().data.parent_id.is_none() {
        return false;
    }
    return is_comment_alternative_angle(
        &parent_comment.unwrap().data,
        all_comments_from_submission,
    );
}

fn is_single_comment_alternative_angle(comment: &SubredditCommentsData) -> bool {
    return comment.body.is_some()
        && comment
            .body
            .as_ref()
            .unwrap()
            .contains("Mirrors / Alternative Angles");
}
