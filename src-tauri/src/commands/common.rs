use tauri::State;

use crate::config::ConfigState;
use crate::utils::error::VResult;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

#[tauri::command]
pub async fn get_rua_config(state: State<'_, ConfigState>) -> VResult<String> {
    let config = state.lock().await;
    let rua = serde_json::to_string(&config.rua)?;
    Ok(rua)
}