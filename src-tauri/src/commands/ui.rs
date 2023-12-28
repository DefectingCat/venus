use crate::{core::exit_core, utils::error::VResult};
use anyhow::anyhow;
use tauri::{AppHandle, Manager};

/// Toggles the visibility of a window with the specified label
///
/// # Arguments
///
/// * `app` - An `AppHandle` instance to interact with the Tauri application.
/// * `label` - A `String` representing the label of the window to toggle.
/// * `show` - A boolean flag indicating whether to show (`true`) or hide (`false`) the window.
#[tauri::command]
pub async fn toggle_window(app: AppHandle, label: String, show: bool) -> VResult<()> {
    let win = app
        .get_window(&label)
        .ok_or(anyhow!("cannot get window {}", label))?;
    if show {
        win.show()?;
        win.set_focus()?;
    } else {
        win.hide()?;
    }
    Ok(())
}

/// Exit the whole APP
#[tauri::command]
pub async fn exit_app(app: AppHandle) -> VResult<()> {
    exit_core().await?;
    app.exit(0);
    Ok(())
}
