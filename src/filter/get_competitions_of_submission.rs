use crate::{config::config::Config, reddit_scrape::scrape_reddit_submission::RedditSubmission};

use super::{competition::Competition, filter::submission_filter};

pub fn get_competitions_of_submission(
    submission: &RedditSubmission,
    config: &Config,
) -> Vec<Competition> {
    config
        .clone()
        .into_iter()
        .filter(|c| submission_filter(&submission, &c))
        .collect()
}
