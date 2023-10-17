use std::time::Duration;

use jsonpath_rust::JsonPathFinder;
use log::info;
use regex::Regex;
use roux::comment::CommentData;
use roux::MaybeReplies;
use teloxide::types::MessageId;
use tokio::time;

use crate::{
    filter::competition::CompetitionName, telegram::send::reply_with_retries, GoalSubmission,
};

use super::listen_for_submissions::RedditHandle;

fn is_aa_comment(comment: &CommentData) -> bool {
    return comment.body.is_some()
        && comment
            .body
            .as_ref()
            .unwrap()
            .contains("Mirrors / Alternative Angles");
}

fn flatten_comment_with_replies(comment_tree: &CommentData) -> Vec<&CommentData> {
    let mut result: Vec<&CommentData> = Vec::new();
    result.push(&comment_tree);

    if comment_tree.replies.is_some() {
        let replies = comment_tree.replies.as_ref().unwrap();
        let replies = match replies {
            MaybeReplies::Str(_) => None,
            MaybeReplies::Reply(reply) => Some(reply),
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
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/113.0.0.0 Safari/537.36")
        .build().unwrap();
    let json_content = client
        .get(&request_url)
        .send()
        .await
        .ok()?
        .text()
        .await
        .ok()?;

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

impl RedditHandle {
    pub async fn search_for_alternative_angles_in_submission_comments(&mut self) {
        let mut interval = time::interval(Duration::from_secs(60));
        loop {
            interval.tick().await;

            let replay_submission_ids_copied: Vec<GoalSubmission>;
            {
                let mut replay_submission_ids =
                    self.listen_for_replays_submission_ids.lock().unwrap();
                let new_listen_for_replays_submission_ids: Vec<GoalSubmission> =
                    replay_submission_ids
                        .clone()
                        .into_iter()
                        .filter(|goal_submission| {
                            (goal_submission.added_time.time()
                                - chrono::offset::Local::now().time())
                            .num_hours()
                                <= 1
                        })
                        .collect();
                *replay_submission_ids = new_listen_for_replays_submission_ids;
                replay_submission_ids_copied = replay_submission_ids.clone();
            }
            for goal_submission in replay_submission_ids_copied.iter() {
                self.process_goal_submission_for_alternative_angles(goal_submission)
                    .await;
            }
        }
    }

    async fn process_goal_submission_for_alternative_angles(
        &mut self,
        goal_submission: &GoalSubmission,
    ) {
        let competition = match goal_submission.competition {
            CompetitionName::Bundesliga => &self.config.bundesliga,
            CompetitionName::PremierLeague => &self.config.premier_league,
            CompetitionName::ChampionsLeague => &self.config.champions_league,
            CompetitionName::Internationals => &self.config.internationals,
        };

        let comments_depth_1 = match self
            .subreddit
            .article_comments(&goal_submission.submission_id, Some(2), Some(100))
            .await
        {
            Ok(comments) => comments,
            Err(_) => return,
        };

        let comments_depth_1_flattened: Vec<&CommentData> = comments_depth_1
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
            .filter(|comment| {
                comment.parent_id.as_ref().unwrap() == &format!("t1_{}", aa_comment_id)
            })
            // Only comments that contain a url are relevant
            .filter(|comment| {
                let url_regex = Regex::new(r"(www\.)?[-a-zA-Z0-9@:%._\+~#=]{2,256}\.[a-z]{2,4}\b([-a-zA-Z0-9@:%_\+.~#?&//=]*)").unwrap();
                match &comment.body_html {
                    None => false,
                    Some(body) => {
                        url_regex.is_match(&body)
                    }
                }
            })
            // Ignore comments that were already sent
            .filter(|comment| {
                // Get the current list of sent comments since it could have changed
                let mut goal_submissions = self.listen_for_replays_submission_ids.lock().unwrap();
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
            .collect::<Vec<&&CommentData>>();

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
                &self.bot,
                &content,
                competition.get_chat_id_replies(),
                MessageId(goal_submission.reply_id),
            )
            .await;
        }
    }
}

#[cfg(test)]
mod test {
    use std::sync::Arc;

    use dotenv::dotenv;
    use teloxide::Bot;

    use crate::config::config::Config;

    use super::*;

    // TODO: Refactor this so we can mock the telegram bot
    // so that we do not send messages when testing
    #[tokio::test]
    #[ignore]
    async fn process_goal_submission_for_alternative_angles_test1() {
        dotenv().ok();
        // let config = Arc::new(Config::init());
        // let subreddit = Arc::new(Subreddit::new("soccer"));
        // // TODO: Mock telegram bot
        // let bot = Arc::new(Bot::from_env());
        // let goal_submission = GoalSubmission {
        //     // This test relies on this submission existing: https://old.reddit.com/r/soccer/comments/12s8sb8/bayern_munich_0_1_manchester_city_04_on_agg/
        //     submission_id: String::from("12s8sb8"),
        //     competition: CompetitionName::ChampionsLeague,
        //     sent_comment_ids: Vec::new(),
        //     reply_id: 0,
        //     added_time: chrono::offset::Local::now(),
        // };

        // let listen_for_replays_submission_ids: Arc<Mutex<Vec<GoalSubmission>>> =
        //     Arc::new(Mutex::new(vec![goal_submission.clone()]));

        // process_goal_submission_for_alternative_angles(
        //     subreddit,
        //     bot,
        //     config,
        //     Arc::clone(&listen_for_replays_submission_ids),
        //     &goal_submission,
        // )
        // .await;

        // assert_eq!(listen_for_replays_submission_ids.lock().unwrap().len(), 1);
    }

    #[tokio::test]
    #[ignore = "This test used to scrape a alternative angle of the goal but the reddit comment was removed"]
    async fn test_reddit_comment_body() {
        let submission_id = "17778ok".to_string();
        let comment_id = "k4r336t".to_string();
        let result = get_reddit_comment_body(submission_id, comment_id).await;
        assert_eq!(
            Some("<a href=\"https://cazn.me/m/84617f\">REPLAY </a>\n".to_string()),
            result
        );
    }

    #[tokio::test]
    #[ignore]
    async fn test_reply() {
        dotenv().ok();
        let config = Arc::new(Config::init());
        // TODO: Mock telegram bot
        let bot = Arc::new(Bot::from_env());
        let goal_submission = GoalSubmission {
            submission_id: String::from("1316fru"),
            competition: CompetitionName::PremierLeague,
            sent_comment_ids: Vec::new(),
            reply_id: 1938,
            added_time: chrono::offset::Local::now(),
        };
        reply_with_retries(
            &bot,
            "TEST: Reply auf Tottenham [2] - 2 Manchester United - Heung-min Son 79'",
            config.premier_league.get_chat_id_replies(),
            MessageId(goal_submission.reply_id),
        )
        .await;
    }

    #[test]
    fn test_regex() {
        let url_regex = Regex::new(
            r"(www\.)?[-a-zA-Z0-9@:%._\+~#=]{2,256}\.[a-z]{2,4}\b([-a-zA-Z0-9@:%_\+.~#?&//=]*)",
        )
        .unwrap();
        assert!(url_regex.is_match("google.com"));
        assert!(url_regex.is_match("www.google.com"));
        assert!(url_regex.is_match("https://www.google.com"));
        assert!(url_regex.is_match("https://google.com"));
        assert!(url_regex.is_match("asdf https://google.com jklö"));
        assert!(!url_regex.is_match("google"));
        assert!(url_regex.is_match("google.com?key=value"));
    }
}
