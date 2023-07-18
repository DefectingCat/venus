use log::info;
use serde::{Deserialize, Serialize};
use tauri::State;

use crate::config::{ConfigState, CoreConfig, RConfig};
use crate::message::{ConfigMsg, MsgSender};
use crate::utils::error::VResult;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ReturnConfig {
    Core(CoreConfig),
    Rua(RConfig),
}
/// Return specify config field
#[tauri::command]
pub async fn get_config(
    state: State<'_, ConfigState>,
    config_type: &str,
) -> VResult<Option<ReturnConfig>> {
    let config = state.lock().await;
    match config_type {
        "core" => {
            let core: Option<ReturnConfig> = config
                .core
                .clone()
                .and_then(|core| Some(ReturnConfig::Core(core)));
            return Ok(core);
        }
        "rua" => {
            let rua = config.rua.clone();
            return Ok(Some(ReturnConfig::Rua(rua)));
        }
        _ => return Ok(None),
    }
}

/// Update config file from frontend
/// Core will restart alfter write config to file
#[tauri::command]
pub async fn update_config(
    state: State<'_, ConfigState>,
    tx: State<'_, MsgSender>,
    core_config: Option<CoreConfig>,
    rua_config: Option<RConfig>,
) -> VResult<()> {
    if let Some(c) = core_config {
        info!("Updating core config");
        let mut config = state.lock().await;
        config.core = Some(c);
        config.write_core()?;
        tx.send(ConfigMsg::RestartCore).await?;
    }

    if let Some(r) = rua_config {
        info!("Updating rua config");
        let mut config = state.lock().await;
        config.rua = r;
        config.write_rua()?;
        tx.send(ConfigMsg::RestartCore).await?;
    }
    Ok(())
}
