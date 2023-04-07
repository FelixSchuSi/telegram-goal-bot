use std::str::FromStr;

use chrono::Utc;
use log::info;
use roux::subreddit::responses::SubmissionsData;

use super::{
    competition::{Competition, IsValidCompetition},
    videohost::VideoHost,
};

const UNDER_7_TO_UNDER_21: [&str; 15] = [
    "u7", "u8", "u9", "u10", "u11", "u12", "u13", "u14", "u15", "u16", "u17", "u18", "u19", "u20",
    "u21",
];

#[allow(dead_code)]
pub fn submission_filter(submission: &SubmissionsData, competition: &Competition) -> bool {
    let host = submission.url.to_owned().unwrap_or_default();
    let lower_title = submission.title.to_lowercase();
    let mut title_split = lower_title.split_whitespace();

    info!("Checking submission: {}", submission.title);

    // Titles of goal videos are expected to be in the format: "team1 [1] - [0] team2"
    // So a valid title has to contain a hyphen
    if !title_split.any(|s| s == "-") {
        info!("Title does not contain a hyphen: {}", submission.title);
        return false;
    }

    // Ignore all u7 to u21 games
    if title_split.any(|s| UNDER_7_TO_UNDER_21.contains(&s)) {
        info!(
            "Title contains an age group (u7 to u21): {}",
            submission.title
        );
        return false;
    }

    // Also ignore womens games
    if title_split.any(|s| s == "w") {
        info!("Title contains a womens game: {}", submission.title);
        return false;
    }

    // Check if the video is hosted on one of the specified VideoHosts
    if VideoHost::from_str(&host).is_err() {
        info!("Video is not hosted on a valid host: {}", &host);
        return false;
    }

    // Check if the title contains two teams of the specified competition
    if !competition.is_valid_post_title_for_competition(&submission.title) {
        info!(
            "Title does not contain two teams of the specified competition: {}",
            submission.title
        );
        return false;
    }

    // Post must be younger than 3 minutes
    // if Utc::now().timestamp() - submission.created_utc as i64 > 180 {
    //     trace!(
    //         "Submission is not younger than 3 minutes: {}",
    //         submission.title
    //     );
    //     return false;
    // }

    true
}
