use log::info;
use tauri::State;

use crate::{config::ConfigState, utils::error::VResult};

#[tauri::command]
pub async fn toggle_logging(enable: bool, config: State<'_, ConfigState>) -> VResult<()> {
    let mut config = config.lock().await;
    config.rua.logging = enable;
    if enable {
        info!("Logging enabled");
    }
    Ok(())
}
