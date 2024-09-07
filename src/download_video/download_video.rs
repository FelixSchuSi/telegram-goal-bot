use log::{error, info, warn};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use std::{path::PathBuf, time::Duration};
use tokio::{fs::File, io::AsyncWriteExt, time::sleep};

async fn download_video(url: &str) -> Result<PathBuf, String> {
    let video_bytes = reqwest::get(url)
        .await
        .map_err(|e| format!("Failed to download video: {}", e))?
        .bytes()
        .await
        .map_err(|e| format!("Failed to extract bytes form downloaded video: {}", e))?;

    let filename = PathBuf::from(format!("/tmp/{}.mp4", random_string(30)));

    let mut file = File::create(&filename)
        .await
        .map_err(|e| format!("Failed to create file: {}", e))?;

    file.write_all(&video_bytes)
        .await
        .map_err(|e| format!("Failed to write to file: {}", e))?;

    Ok(filename)
}

pub async fn download_video_with_retries(url: &str) -> Result<PathBuf, String> {
    let mut download_video_result = download_video(url).await;
    for i in 1..10 {
        if download_video_result.is_ok() {
            info!(
                "Downloading video was successfull in try {} out of 10. url: {}",
                i, url
            );
            break;
        }
        warn!(
            "Downloading video failed in try {} out of 10. url: {}, trying again in 10 seconds. err: {}",
            i, url, download_video_result.err().unwrap_or("".to_string())
        );
        sleep(Duration::from_secs(10)).await;
        download_video_result = download_video(url).await;
    }
    error!("Downloading video failed in all 10 tries. url: {}", url);
    download_video_result
}

fn random_string(n: usize) -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(n)
        .map(char::from)
        .collect()
}
