#[derive(Debug, Clone)]
pub struct Submission {
    pub submission_id: String,
    pub title: String,
    pub link: String,
    pub posted_timestamp: DateTime<Local>,
}
