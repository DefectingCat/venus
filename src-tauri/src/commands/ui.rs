use crate::{
    core::exit_core,
    utils::{error::VResult, toggle_windows},
};
use tauri::{AppHandle, Manager};

/// Toggle all windows not only main window
/// and change `mian_visible` in UI
#[tauri::command]
pub async fn toggle_main(app: AppHandle, show: bool) -> VResult<()> {
    let windows = app.windows();
    toggle_windows(windows, show).await?;
    Ok(())
}

/// Exit the whole APP
#[tauri::command]
pub async fn exit_app(app: AppHandle) -> VResult<()> {
    exit_core().await?;
    app.exit(0);
    Ok(())
}
