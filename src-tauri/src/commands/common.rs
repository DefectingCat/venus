use base64::engine::general_purpose;
use base64::Engine;
use std::env;
use std::path::PathBuf;

use crate::utils::error::VResult;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
pub fn current_dir() -> Result<PathBuf, String> {
    Ok(env::current_dir().expect("error while read current dir"))
}

#[tauri::command]
pub async fn add_subscription(url: String) -> VResult<()> {
    let result = reqwest::get(url).await?.text().await?;

    // Decode result to vmess://...
    let subscripition = general_purpose::STANDARD.decode(result)?;
    let subscripition = String::from_utf8_lossy(&subscripition).to_string();
    let subscripition = subscripition
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            let line = line.replace("vmess://", "");
            let line = general_purpose::STANDARD.decode(line)?;
            Ok(String::from_utf8_lossy(&line).to_string())
        })
        .collect::<VResult<Vec<_>>>()?;
    dbg!(&subscripition);
    Ok(())
}
