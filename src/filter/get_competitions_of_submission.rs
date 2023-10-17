use roux::submission::SubmissionData;

use crate::config::config::Config;

use super::{competition::Competition, filter::submission_filter};

pub fn get_competitions_of_submission(
    submission: &SubmissionData,
    config: &Config,
) -> Vec<Competition> {
    config
        .clone()
        .into_iter()
        .filter(|c| submission_filter(&submission, &c))
        .collect()
}
