use crate::utils::{error::VResult, toggle_windows};
use tauri::{AppHandle, Manager};

#[tauri::command]
pub async fn toggle_main(app: AppHandle, show: bool) -> VResult<()> {
    let windows = app.windows();
    toggle_windows(windows, show)?;
    Ok(())
}
