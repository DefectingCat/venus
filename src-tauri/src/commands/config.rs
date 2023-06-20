use tauri::State;

use crate::config::{ConfigState, CoreConfig, RConfig};
use crate::utils::error::VResult;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

#[tauri::command]
/// Get frontend config
pub async fn get_rua_config(state: State<'_, ConfigState>) -> VResult<RConfig> {
    let config = state.lock().await;
    Ok(config.rua.clone())
}

#[tauri::command]
pub async fn get_core_config(state: State<'_, ConfigState>) -> VResult<Option<CoreConfig>> {
    let config = state.lock().await;
    Ok(config.core.clone())
}
