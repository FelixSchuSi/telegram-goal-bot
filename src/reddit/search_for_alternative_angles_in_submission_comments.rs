use crate::{
    config::config::Config, filter::competition::CompetitionName,
    telegram::send::reply_with_retries, GoalSubmission,
};
use jsonpath_rust::JsonPathFinder;
use log::info;
use roux::{
    subreddit::responses::{comments::SubredditReplies, SubredditCommentsData},
    Subreddit,
};
use std::{
    sync::{Arc, Mutex},
    time::Duration,
};
use teloxide::{types::MessageId, Bot};
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
            process_goal_submission_for_alternative_angles(
                subreddit.clone(),
                bot.clone(),
                config.clone(),
                listen_for_replays_submission_ids.clone(),
                goal_submission,
            )
            .await;
        }
    }
}

async fn process_goal_submission_for_alternative_angles(
    subreddit: Arc<Subreddit>,
    bot: Arc<Bot>,
    config: Arc<Config>,
    listen_for_replays_submission_ids: Arc<Mutex<Vec<GoalSubmission>>>,
    goal_submission: &GoalSubmission,
) {
    let competition = match goal_submission.competition {
        CompetitionName::Bundesliga => &config.bundesliga,
        CompetitionName::PremierLeague => &config.premier_league,
        CompetitionName::ChampionsLeague => &config.champions_league,
        CompetitionName::Internationals => &config.internationals,
    };

    let comments_depth_1 = match subreddit
        .article_comments(&goal_submission.submission_id, Some(2), Some(100))
        .await
    {
        Ok(comments) => comments,
        Err(_) => return,
    };

    let comments_depth_1_flattened: Vec<&SubredditCommentsData> = comments_depth_1
        .data
        .children
        .iter()
        .flat_map(|comment| flatten_comment_with_replies(&comment.data))
        .collect();

    let aa_comment = match comments_depth_1_flattened
        .iter()
        .find(|comment| is_aa_comment(comment))
    {
        Some(comment) => comment,
        None => return,
    };
    let aa_comment_id = aa_comment.id.clone().unwrap();

    let relevant_comments = comments_depth_1_flattened
        .iter()
        // Only comments that are replies to the aa_comment are relevant
        .filter(|comment| comment.parent_id.as_ref().unwrap() == &format!("t1_{}", aa_comment_id))
        // Ignore comments that were already sent
        .filter(|comment| {
            // Get the current list of sent comments since it could have changed
            let mut goal_submissions = listen_for_replays_submission_ids.lock().unwrap();
            let goal_submission = goal_submissions
                .iter_mut()
                .find(|gs| gs.submission_id == goal_submission.submission_id)
                .unwrap();
            let sent_commets = &mut goal_submission.sent_comment_ids;
            let res = !sent_commets.contains(&comment.id.clone().unwrap());
            // We are sending this comment, so we should add this comment to the list of sent comments
            sent_commets.push(comment.id.clone().unwrap());
            return res;
        })
        .collect::<Vec<&&SubredditCommentsData>>();

    for comment in relevant_comments {
        let content = get_reddit_comment_body(
            goal_submission.submission_id.clone(),
            comment.id.as_ref().unwrap().to_string(),
        )
        .await
        .unwrap_or_default();

        info!(
            "Sending replay in competition: {:?}, submission_id: {:?}, content: {:?}",
            competition.name, goal_submission.submission_id, content
        );

        reply_with_retries(
            &bot,
            &content,
            competition.get_chat_id_replies(),
            MessageId(goal_submission.reply_id),
        )
        .await;
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

async fn get_reddit_comment_body(submission_id: String, comment_id: String) -> Option<String> {
    let request_url = format!(
        "https://old.reddit.com/r/soccer/comments/{}//{}.json",
        submission_id, comment_id
    );
    let json_content = reqwest::get(&request_url).await.ok()?.text().await.ok()?;
    let json_path_result =
        JsonPathFinder::from_str(&json_content, "[1].data.children[0].data.body_html")
            .unwrap()
            .find();

    let html_encoded = json_path_result.as_array()?[0].as_str()?.to_string();
    let reuslt = html_escape::decode_html_entities(&html_encoded)
        .to_string()
        .replace("<div class=\"md\">", "")
        .replace("</div>", "")
        .replace("<p>", "")
        .replace("</p>", "");
    return Some(reuslt.to_string());
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::config::config::Config;
    use dotenv::dotenv;
    use roux::Subreddit;
    use std::sync::Arc;
    use teloxide::Bot;

    // TODO: Refactor this so we can mock the telegram bot
    // so that we do not send messages when testing
    #[tokio::test]
    #[ignore]
    async fn process_goal_submission_for_alternative_angles_test1() {
        dotenv().ok();
        let config = Arc::new(Config::init());
        let subreddit = Arc::new(Subreddit::new("soccer"));
        // TODO: Mock telegram bot
        let bot = Arc::new(Bot::from_env());
        let goal_submission = GoalSubmission {
            // This test relies on this submission existing: https://old.reddit.com/r/soccer/comments/12s8sb8/bayern_munich_0_1_manchester_city_04_on_agg/
            submission_id: String::from("12s8sb8"),
            competition: CompetitionName::ChampionsLeague,
            sent_comment_ids: Vec::new(),
            reply_id: 0,
        };

        let listen_for_replays_submission_ids: Arc<Mutex<Vec<GoalSubmission>>> =
            Arc::new(Mutex::new(vec![goal_submission.clone()]));

        process_goal_submission_for_alternative_angles(
            subreddit,
            bot,
            config,
            Arc::clone(&listen_for_replays_submission_ids),
            &goal_submission,
        )
        .await;

        assert_eq!(listen_for_replays_submission_ids.lock().unwrap().len(), 1);
    }

    #[tokio::test]
    async fn test_reddit_comment_body() {
        let submission_id = "12s8sb8".to_string();
        let comment_id = "jgxcm5l".to_string();
        let result = get_reddit_comment_body(submission_id, comment_id).await;
        assert_eq!(
            Some("<a href=\"https://www.ziscore.com/qa30/\">REPLAY </a>\n".to_string()),
            result
        );
    }
}
