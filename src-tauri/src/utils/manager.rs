use crate::utils::api::LatestRelease;
use anyhow::Result;
use reqwest::header::USER_AGENT;
use reqwest::Client;

const API_URL: &str = "https://api.github.com/";

pub async fn latest_release() -> Result<LatestRelease> {
    let result = Client::new()
        .get(format!(
            "{}{}",
            API_URL, "repos/v2fly/v2ray-core/releases/latest"
        ))
        .header(USER_AGENT, "V2rayR")
        .send()
        .await?
        .json::<LatestRelease>()
        .await?;
    Ok(result)
}

pub async fn test() {
    println!("hello world");
    let client = Client::new();
    let test = client
        .get(format!(
            "{}{}",
            API_URL, "repos/v2fly/v2ray-core/releases/latest"
        ))
        .header(USER_AGENT, "V2rayR")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    println!("{:?}", test)
}

#[cfg(test)]
mod test {
    use crate::utils::manager::{latest_release, test};

    #[tokio::test]
    async fn test_test() {
        test().await;
    }

    #[tokio::test]
    async fn test_latest_release() {
        // latest_release().await.expect("TODO: panic message");
    }
}
