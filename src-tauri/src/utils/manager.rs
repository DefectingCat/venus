use std::env;

use anyhow::Result;
use reqwest::{header, Client};

use crate::utils::api::LatestRelease;
use crate::version::VERSION;

const API_URL: &str = "https://api.github.com/";

pub struct HttpClient {
    pub client: Client,
}

impl HttpClient {
    pub fn new() -> Result<Self> {
        let mut h = header::HeaderMap::new();
        h.insert(
            header::USER_AGENT,
            header::HeaderValue::from_str(&format!("V2rayR - {}", VERSION)[..])?,
        );
        Ok(Self {
            client: Client::builder().default_headers(h).build()?,
        })
    }
}

/// Get the latest release from GitHub release.
pub async fn latest_release(client: &Client) -> Result<LatestRelease> {
    let result = client
        .get(format!(
            "{}{}",
            API_URL, "repos/v2fly/v2ray-core/releases/latest"
        ))
        .send()
        .await?
        .json::<LatestRelease>()
        .await?;
    Ok(result)
}

pub async fn download_latest(client: &Client) -> Result<()> {
    let latest = latest_release(&client).await?;

    let system = env::consts::OS;
    #[cfg(target_pointer_width = "64")]
    let target = &format!("v2ray-{}-64.zip", system)[..];
    #[cfg(target_pointer_width = "32")]
    let target = &format!("v2ray-{}-32.zip", system)[..];
    let target = latest.assets.iter().find(|asset| asset.name == target);
    dbg!(&target);
    dbg!(env::current_dir());
    println!("{:?}", env::current_dir());

    Ok(())
}

#[cfg(test)]
mod test {
    use crate::utils::manager::{download_latest, latest_release, HttpClient};

    #[tokio::test]
    async fn test_latest_release() {
        let client = HttpClient::new().unwrap().client;
        let result = latest_release(&client).await.unwrap();
        println!("{}", result.name);
    }

    #[tokio::test]
    async fn test_download() {
        let client = HttpClient::new().unwrap().client;
        download_latest(&client).await.unwrap();
    }
}
