use tauri::State;

use crate::config::{ConfigState, CoreConfig, RConfig};
use crate::message::{ConfigMsg, MsgSender};
use crate::utils::error::VResult;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

/// Send rua config to frontend
#[tauri::command]
pub async fn get_rua_config(state: State<'_, ConfigState>) -> VResult<RConfig> {
    let config = state.lock().await;
    Ok(config.rua.clone())
}

/// Send core config to frontend
#[tauri::command]
pub async fn get_core_config(state: State<'_, ConfigState>) -> VResult<Option<CoreConfig>> {
    let config = state.lock().await;
    Ok(config.core.clone())
}

/// Update core config file from frontend
/// Core will restart alfter write config to file
#[tauri::command]
pub async fn update_core(
    state: State<'_, ConfigState>,
    tx: State<'_, MsgSender>,
    core_config: CoreConfig,
) -> VResult<()> {
    let mut config = state.lock().await;
    config.core = Some(core_config);
    config.write_core()?;
    tx.send(ConfigMsg::RestartCore).await?;
    Ok(())
}
