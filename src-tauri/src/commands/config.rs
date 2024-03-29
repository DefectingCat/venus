use crate::commands::subs::check_subs_update;
use crate::config::{CoreConfig, RConfig};
use crate::message::{ConfigMsg, MSG_TX};
use crate::utils::error::VResult;
use crate::{CONFIG, LOGGING};
use log::info;
use serde::{Deserialize, Serialize};
use tokio::fs::File;
use tokio::io::AsyncReadExt;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ReturnConfig {
    Core(Box<CoreConfig>),
    Rua(Box<RConfig>),
}
#[derive(Debug, Deserialize)]
pub enum WhichConfig {
    Rua,
    Core,
}
/// Return specify config field
#[tauri::command]
pub async fn get_config(config_type: WhichConfig) -> VResult<Option<ReturnConfig>> {
    let config = CONFIG.lock().await;
    match config_type {
        WhichConfig::Core => {
            let core: Option<ReturnConfig> = config
                .core
                .clone()
                .map(|core| ReturnConfig::Core(Box::new(core)));
            Ok(core)
        }
        WhichConfig::Rua => {
            let rua = config.rua.clone();
            Ok(Some(ReturnConfig::Rua(Box::new(rua))))
        }
    }
}

/// Update config file from frontend
/// Core will restart alfter write config to file
#[tauri::command]
pub async fn update_config(
    core_config: Option<CoreConfig>,
    rua_config: Option<RConfig>,
) -> VResult<()> {
    use std::sync::atomic::Ordering::Relaxed;

    let mut config = CONFIG.lock().await;
    if let Some(c) = core_config {
        info!("Updating core config");
        config.core = Some(c);
        config.write_core()?;
    }

    if let Some(r) = rua_config {
        info!("Updating rua config");
        if r.logging {
            LOGGING.store(true, Relaxed);
        } else {
            LOGGING.store(false, Relaxed);
        }
        config.rua = r;
        config.write_rua()?;
    }
    check_subs_update(&mut config).await?;
    MSG_TX.lock().await.send(ConfigMsg::RestartCore).await?;
    Ok(())
}

#[tauri::command]
pub async fn read_config_file(which: WhichConfig) -> VResult<String> {
    let config = CONFIG.lock().await;
    let path = match which {
        WhichConfig::Rua => &config.rua_path,
        WhichConfig::Core => &config.core_path,
    };
    let mut config_file = File::open(path).await?;
    let mut buffer = String::new();
    config_file.read_to_string(&mut buffer).await?;
    Ok(buffer)
}
