use std::sync::atomic::Ordering;

use log::info;

use crate::{utils::error::VResult, LOGGING};

#[tauri::command]
pub async fn toggle_logging(enable: bool) -> VResult<()> {
    // LOGGING.store(!LOGGING.load(Ordering::Relaxed), Ordering::Relaxed);
    if LOGGING.load(Ordering::Relaxed) {
        info!("Logging enabled");
    }
    LOGGING.store(enable, Ordering::Relaxed);
    Ok(())
}
