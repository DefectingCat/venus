use crate::config::{CoreConfig, RConfig};
use crate::message::{ConfigMsg, MSG_TX};
use crate::utils::error::VResult;
use crate::{CONFIG, LOGGING};
use log::info;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ReturnConfig {
    Core(Box<CoreConfig>),
    Rua(Box<RConfig>),
}
/// Return specify config field
#[tauri::command]
pub async fn get_config(config_type: &str) -> VResult<Option<ReturnConfig>> {
    let config = CONFIG.lock().await;
    match config_type.to_lowercase().as_str() {
        "core" => {
            let core: Option<ReturnConfig> = config
                .core
                .clone()
                .map(|core| ReturnConfig::Core(Box::new(core)));
            Ok(core)
        }
        "rua" => {
            let rua = config.rua.clone();
            Ok(Some(ReturnConfig::Rua(Box::new(rua))))
        }
        _ => Ok(None),
    }
}

/// Update config file from frontend
/// Core will restart alfter write config to file
#[tauri::command]
pub async fn update_config(
    core_config: Option<CoreConfig>,
    rua_config: Option<RConfig>,
) -> VResult<()> {
    if let Some(c) = core_config {
        info!("Updating core config");
        let mut config = CONFIG.lock().await;
        config.core = Some(c);
        config.write_core()?;
        MSG_TX.lock().await.send(ConfigMsg::RestartCore).await?;
    }

    if let Some(r) = rua_config {
        info!("Updating rua config");
        let mut config = CONFIG.lock().await;
        if r.logging {
            LOGGING.store(true, std::sync::atomic::Ordering::Relaxed);
        } else {
            LOGGING.store(false, std::sync::atomic::Ordering::Relaxed);
        }
        config.rua = r;
        config.write_rua()?;
        MSG_TX.lock().await.send(ConfigMsg::RestartCore).await?;
    }
    Ok(())
}
