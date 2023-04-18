use crate::{
    config::config::Config, filter::competition::CompetitionName, replay,
    telegram::send::send_message_direct, GoalSubmission,
};
use roux::{
    subreddit::responses::{comments::SubredditReplies, SubredditCommentsData},
    Subreddit,
};
use std::{
    sync::{Arc, Mutex},
    time::Duration,
};
use teloxide::Bot;
use tokio::time;

pub async fn search_for_alternative_angles_in_submission_comments(
    subreddit: Arc<Subreddit>,
    bot: Arc<Bot>,
    config: Arc<Config>,
    listen_for_replays_submission_ids: Arc<Mutex<Vec<GoalSubmission>>>,
) {
    let mut interval = time::interval(Duration::from_secs(60));
    loop {
        interval.tick().await;

        let replay_submission_ids_copied: Vec<GoalSubmission>;
        {
            let replay_submission_ids = listen_for_replays_submission_ids.lock().unwrap();
            replay_submission_ids_copied = replay_submission_ids.clone();
        }
        for goal_submission in replay_submission_ids_copied.iter() {
            let all_comments_from_submission = subreddit
                .article_comments(&goal_submission.submission_id, Some(2), Some(100))
                .await;
            let all_comments_from_submission = match all_comments_from_submission {
                Ok(comments) => comments,
                Err(_) => {
                    continue;
                }
            };
            let all_comments_from_submission = all_comments_from_submission.data.children;

            let mut all_comments_flattened = Vec::new();

            for comment in all_comments_from_submission.iter() {
                all_comments_flattened.extend(flatten_comment_with_replies(&comment.data));
            }

            for comment in all_comments_flattened.iter() {
                if !is_aa_comment(comment) {
                    continue;
                }

                let skip = replay_submission_ids_copied.iter().find(|e| {
                    e.submission_id == goal_submission.submission_id
                        && e.sent_comment_ids.contains(&comment.id.clone().unwrap())
                });
                if skip.is_some() {
                    continue;
                }

                let content = comment.body.as_ref().unwrap();
                let comp = match goal_submission.competition {
                    CompetitionName::Bundesliga => &config.bundesliga,
                    CompetitionName::PremierLeague => &config.premier_league,
                    CompetitionName::ChampionsLeague => &config.champions_league,
                    CompetitionName::Internationals => &config.internationals,
                };
                {
                    let id = comment.id.clone().unwrap();
                    let goal = listen_for_replays_submission_ids.lock().unwrap();
                    let goal = goal
                        .iter()
                        .find(|x| x.submission_id == goal_submission.submission_id);
                    if goal.is_some() {
                        let mut sent_comment_ids = goal.unwrap().to_owned().sent_comment_ids;
                        sent_comment_ids.push(id);
                    }
                }
                send_message_direct(&content, &bot, comp).await;
            }
        }
    }
}

fn is_aa_comment(comment: &SubredditCommentsData) -> bool {
    return comment.body.is_some()
        && comment
            .body
            .as_ref()
            .unwrap()
            .contains("Mirrors / Alternative Angles");
}

fn flatten_comment_with_replies(
    comment_tree: &SubredditCommentsData,
) -> Vec<&SubredditCommentsData> {
    let mut result: Vec<&SubredditCommentsData> = Vec::new();
    result.push(&comment_tree);

    if comment_tree.replies.is_some() {
        let replies = comment_tree.replies.as_ref().unwrap();
        let replies = match replies {
            SubredditReplies::Str(_) => None,
            SubredditReplies::Reply(reply) => Some(reply),
        };
        if replies.is_none() {
            return result;
        }
        let replies = &replies.unwrap().data.children;

        for reply in replies.iter() {
            result.push(&reply.data);
            result.append(&mut flatten_comment_with_replies(&reply.data));
        }
    }
    return result;
}
