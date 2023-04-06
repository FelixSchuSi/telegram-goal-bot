// def filter_submission(submission, competition):
//   title = submission.title.lower().split()
//   # title must contain a hyphen AND not be a u19 or u21 game.
//   if (any('-' in e for e in title) and not any('u19' in e for e in title) and not any('u21' in e for e in title) and not any('w' in e for e in title)) or len(competition.teams) == 0:
//     # video must be hosted on one of the specified services.
//       diff = datetime.utcnow() - datetime.utcfromtimestamp(submission.created_utc)
//       # post must be younger than 3 minutes.
//       if (diff.total_seconds() / 60) < 3:
//         # title must contain two teams of the specified competition.
//         if competition.is_competition(title):
//           return True

use std::str::FromStr;

use super::videohost::VideoHost;
const UNDER_7_TO_UNDER_21: [&str; 15] = [
    "u7", "u8", "u9", "u10", "u11", "u12", "u13", "u14", "u15", "u16", "u17", "u18", "u19", "u20",
    "u21",
];
#[allow(dead_code)]
pub fn filter(title: &str, host: &str) -> bool {
    let lower_title = title.to_lowercase();
    let mut title_split = lower_title.split_whitespace();

    // Titles of goal videos are expected to be in the format: "team1 [1] - [0] team2"
    // So a valid title has to contain a hyphen
    if !title_split.any(|s| s == "-") {
        return false;
    }

    // Ignore u19 and u21 games
    if lower_title.contains("u19") || lower_title.contains("u21") {
        return false;
    }

    // Ignore all u7 to u21 games
    if title_split.any(|s| UNDER_7_TO_UNDER_21.contains(&s)) {
        return false;
    }

    // Also ignore womens games
    if title_split.any(|s| s == "w") {
        return false;
    }

    // Check if the video is hosted on one of the specified VideoHosts
    if VideoHost::from_str(host).is_err() {
        return false;
    }

    // TODO: post must be younger than 3 minutes.

    // TODO: competition check
    false
}
