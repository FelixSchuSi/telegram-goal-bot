// use futures_util::StreamExt;
// use roux::{
//     responses::BasicThing,
//     subreddit::responses::{SubmissionsData, SubredditComments, SubredditCommentsData},
//     Subreddit,
// };
// use roux_stream::stream_comments;
// use teloxide::Bot;
// use tokio::time::{sleep, Duration};
// use tokio_retry::strategy::ExponentialBackoff;

// use crate::{filter::competition::Competition, telegram::send::send_reply};

// pub fn listen_for_replays(
//     submission: &SubmissionsData,
//     bot: &Bot,
//     url: &str,
//     competition: &Competition,
// ) {
//     let id = submission.id.clone();
//     tokio::spawn(async move {
//         sleep(Duration::from_secs(90)).await;
//         // let comments = Subreddit::new("soccer")
//         //     .article_comments(&id, None, Some(25))
//         //     .await;

//         // if comments.is_err() {
//         //     return;
//         // }
//         // let aa_comment = get_aa_comment(comments.unwrap());
//         // if aa_comment.is_none() {
//         //     return;
//         // }
//         // let aa_comment = aa_comment.unwrap();
//     });
// }

// async fn get_aa_comment_from_submission_id(id: String) -> Option<SubredditCommentsData> {
//     let comments = Subreddit::new("soccer")
//         .article_comments(&id, Some(1), Some(25))
//         .await;
//     if comments.is_err() {
//         return None;
//     }
//     let comments = comments.unwrap();
//     for comment in comments.data.children {
//         let has_author = comment.data.author.is_some();
//         let has_body = comment.data.body.is_some();
//         if !has_author || !has_body {
//             continue;
//         }
//         if comment.data.author.as_ref().unwrap() == "AutoModerator"
//             && comment
//                 .data
//                 .body
//                 .as_ref()
//                 .unwrap()
//                 .contains("Mirrors / Alternative Angles")
//         {
//             return Some(comment.data);
//         }
//     }
//     None
// }

// async fn listen_for_comments(submission_id: String) {
//     let subreddit = Subreddit::new("soccer");
//     let retry_strategy = ExponentialBackoff::from_millis(5).factor(100).take(3);

//     let (mut stream, join_handle) = stream_comments(
//         &subreddit,
//         Duration::from_secs(10),
//         retry_strategy,
//         Some(Duration::from_secs(10)),
//     );

//     while let Some(comment) = stream.next().await {
//         let comment = comment.unwrap();
//         println!("{:?}", comment.parent_id);
//         // For now we will send every comment
//         if comment.body_html.is_some() {
//             // send_reply(
//             //     &comment,
//             //     comment.body_html.unwrap(),
//             //     &config.premier_league,
//             //     reply_to_message_id,
//             // )
//             // .await;
//         }

//         // if comment.parent_id.is_some() && comment.parent_id.as_ref().unwrap() != &submission_id {
//         //     continue;
//         // }
//         // comment.parent_id;
//     }
//     join_handle.await.unwrap().unwrap();
// }
