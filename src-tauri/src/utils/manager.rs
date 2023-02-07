use crate::consts::{API_URL, CORE_FOLDER};
use anyhow::Result;
use log::warn;
use reqwest::{header, Client};
use std::{env, fs};

use crate::utils::api::LatestRelease;

pub struct HttpClient {
    pub client: Client,
}

impl HttpClient {
    pub fn new() -> Result<Self> {
        let mut h = header::HeaderMap::new();

        let version = env!("CARGO_PKG_VERSION");
        h.insert(
            header::USER_AGENT,
            header::HeaderValue::from_str(&format!("V2rayR - {}", version)[..])?,
        );
        Ok(Self {
            client: Client::builder().default_headers(h).build()?,
        })
    }
}

/// Check v2ray-core version or exist.
pub async fn check_version() -> Result<()> {
    let paths = fs::read_dir(format!("./{}", CORE_FOLDER));
    match paths {
        Ok(entry) => {
            let bin_name = "v2ray";
            entry.for_each(|file| {
                dbg!(&file.unwrap().file_name());
            });
        }
        Err(err) => {
            warn!("{}", err);
            warn!("core not exist. string downloading");
            fs::create_dir(CORE_FOLDER)?;
        }
    }
    Ok(())
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

/// Download the latest v2ray binary file.
pub async fn download_latest(client: &Client) -> Result<()> {
    let latest = latest_release(client).await?;

    let system = env::consts::OS;
    #[cfg(target_pointer_width = "64")]
    let target = &format!("v2ray-{}-64.zip", system)[..];
    #[cfg(target_pointer_width = "32")]
    let target = &format!("v2ray-{}-32.zip", system)[..];
    let target = latest.assets.iter().find(|asset| asset.name == target);
    // dbg!(&target);
    // @TODO download binary file.
    // dbg!(env::current_dir());
    // println!("{:?}", env::current_dir());

    Ok(())
}

#[cfg(test)]
mod test {
    use crate::utils::manager::{check_version, download_latest, latest_release, HttpClient};

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

    #[tokio::test]
    async fn test_check() {
        check_version().await.unwrap();
    }
}
