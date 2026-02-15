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

    let filename = PathBuf::from(format!("{}.mp4", random_string(30)));

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
            "Downloading video failed in try {} out of 10. url: {}, trying again in 10 seconds",
            i, url
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

#[cfg(test)]
mod tests {
    use super::*;
    #[ignore]
    #[tokio::test]
    async fn test_download_video() {
        let url = "https://c-cdn.streamin.top/uploads/2d487fb9.mp4?Justnow#t=0.1";
        let result = download_video_with_retries(url).await;
        assert!(result.is_ok());
        let path = result.unwrap();
        assert!(path.exists());
        println!(
            "Absolute path to the downloaded video: {:?}",
            std::fs::canonicalize(&path).unwrap()
        );
    }
}
