use crate::scrape::{
    get_html_with_browser::get_html_with_browser, scrape::ScrapeError, scrape_html::scrape_html,
};
use chrono::DateTime;
use std::iter::zip;

#[derive(Debug)]
pub struct SubmissionScrape {
    pub submission_id: String,
    pub link: String,
    pub title: String,
}

#[derive(Debug, Clone)]
pub struct RedditSubmission {
    pub id: String,
    pub url: String,
    pub title: String,
    pub created_utc: i64,
}

pub async fn scrape_reddit_submission(
    submission_id: &str,
) -> Result<SubmissionScrape, ScrapeError> {
    let url = format!(
        "https://old.reddit.com/r/soccer/comments/{submission_id}/",
        submission_id = submission_id
    );

    let html = get_html_with_browser(&url, "a.embed-comment").await?;
    let title = scrape_html(&html, "a.embed-comment", "data-title")?
        .trim()
        .to_string();
    let link = scrape_html(&html, "a.title", "href")?;

    return Ok(SubmissionScrape {
        submission_id: String::from(submission_id),
        link,
        title,
    });
}

pub async fn scrape_submissions() -> Result<Vec<RedditSubmission>, ScrapeError> {
    let url = "https://old.reddit.com/r/soccer/new/";

    let html = get_html_with_browser(&url, "a.title").await?;
    let title_selector = scraper::Selector::parse("a.title").map_err(|_| {
        ScrapeError("The given String could not be parsed to a CSS selector".to_string())
    })?;

    let submission_id_selector = scraper::Selector::parse("div.thing").map_err(|_| {
        ScrapeError("The given String could not be parsed to a CSS selector".to_string())
    })?;
    let submission_id_attr = "data-fullname";

    let video_link_selector = scraper::Selector::parse("a.title").map_err(|_| {
        ScrapeError("The given String could not be parsed to a CSS selector".to_string())
    })?;
    let video_link_attr = "href";

    let created_timestamp_selector = scraper::Selector::parse(".live-timestamp").map_err(|_| {
        ScrapeError("The given String could not be parsed to a CSS selector".to_string())
    })?;
    let created_timestamp_attr = "datetime";

    let titles: Vec<String> = html
        .select(&title_selector)
        .map(|t: scraper::ElementRef| t.inner_html())
        .collect();
    let submission_ids: Vec<&str> = html
        .select(&submission_id_selector)
        .filter_map(|t: scraper::ElementRef| t.value().attr(submission_id_attr))
        .collect();
    let video_links: Vec<&str> = html
        .select(&video_link_selector)
        .filter_map(|t: scraper::ElementRef| t.value().attr(video_link_attr))
        .collect();

    let created_timestamps: Vec<i64> = html
        .select(&created_timestamp_selector)
        .filter_map(|t: scraper::ElementRef| t.value().attr(created_timestamp_attr))
        .map(|e| {
            return DateTime::parse_from_rfc3339(e)
                .map(|e| e.naive_utc())
                .unwrap_or(DateTime::UNIX_EPOCH.naive_utc())
                .timestamp();
        })
        .collect();

    return Ok(zip(
        zip(zip(titles, submission_ids), video_links),
        created_timestamps,
    )
    .map(|(((title, submission_id), video_link), created_utc)| {
        return RedditSubmission {
            url: video_link.to_string(),
            id: submission_id.to_string(),
            title,
            created_utc,
        };
    })
    .collect());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn first_scrape() {
        let scrape_result = scrape_reddit_submission("1dl5kig").await.unwrap();

        assert_eq!(scrape_result.link, "https://streamff.co/v/UqV95HVXAk");
        assert_eq!(scrape_result.submission_id, "1dl5kig");
        assert_eq!(
            scrape_result.title,
            "Slovakia 1 - [2] Ukraine - Roman Yaremchuk 80'"
        );
    }

    #[tokio::test]
    async fn scrape_submissionss() {
        let scrape_result = scrape_submissions().await.unwrap();
        assert_eq!(scrape_result.len(), 28);
        for res in scrape_result {
            assert_ne!(res.url, "");
            assert_ne!(res.id, "");
            assert_ne!(res.title, "");

            assert!(res.url.len() > 1);
            assert!(res.id.len() > 1);
            assert!(res.title.len() > 1);
        }
    }
}
