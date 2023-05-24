use base64::engine::general_purpose;
use base64::Engine;
use std::env;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use tauri::State;

use crate::config::{Node, VConfig};
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
    // Serizlize outbound nodes to json
    let subscripition = subscripition
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            let line = line.replace("vmess://", "");
            let line = general_purpose::STANDARD.decode(line)?;
            let line = String::from_utf8_lossy(&line).to_string();
            Ok(serde_json::from_str::<Node>(&line)?)
        })
        .collect::<VResult<Vec<_>>>()?;
    dbg!(&subscripition);
    Ok(())
}

#[tauri::command]
pub fn get_config(state: State<'_, Arc<Mutex<VConfig>>>) -> VResult<String> {
    let config = serde_json::to_string(state.inner())?;
    Ok(config)
}
