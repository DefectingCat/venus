use crate::utils::{error::VResult, toggle_windows};
use tauri::{AppHandle, Manager};

/// Toggle all windows not only main window
/// and change `mian_visible` in UI
#[tauri::command]
pub async fn toggle_main(app: AppHandle, show: bool) -> VResult<()> {
    let windows = app.windows();
    toggle_windows(windows, show).await?;
    Ok(())
}
