use anyhow::Result;
use reqwest::{header, Client};

use crate::utils::api::LatestRelease;
use crate::version::VERSION;

const API_URL: &str = "https://api.github.com/";

pub struct HttpClient {
    client: Client,
}

impl HttpClient {
    pub fn new() -> Result<Self> {
        let mut h = header::HeaderMap::new();
        h.insert(
            header::USER_AGENT,
            header::HeaderValue::from_str(&format!("V2rayR - {}", VERSION)[..])?,
        );
        dbg!(&h);

        Ok(Self {
            client: Client::builder().default_headers(h).build()?,
        })
    }
}

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

#[cfg(test)]
mod test {
    use crate::utils::manager::{latest_release, test, HttpClient};
    use reqwest::Client;

    #[tokio::test]
    async fn test_latest_release() {
        let client = HttpClient::new().unwrap().client;
        let result = latest_release(&client).await.unwrap();
        println!("{}", result.name);
    }
}
