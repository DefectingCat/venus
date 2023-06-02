use std::env;
use std::path::PathBuf;

use tauri::State;

use crate::config::ConfigState;
use crate::utils::error::VResult;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
pub fn current_dir() -> Result<PathBuf, String> {
    Ok(env::current_dir().expect("error while read current dir"))
}

#[tauri::command]
pub async fn get_subscriptions(state: State<'_, ConfigState>) -> VResult<String> {
    let config = state.lock().await;
    let subs = serde_json::to_string(&config.rua.subscriptions)?;
    Ok(subs)
}