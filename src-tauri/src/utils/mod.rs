use std::collections::HashMap;

use anyhow::{anyhow, Result};
use tauri::Manager;
use tauri::{App, Window};

use crate::message::{ConfigMsg, MSG_TX};
use crate::UI;

pub mod error;

/// Get main window by app
pub fn get_main_window(app: &App) -> Result<Window> {
    let window = app
        .get_window("main")
        .ok_or(anyhow!("Can not get main window"))?;
    Ok(window)
}

/// Toggle all windows visible
///
/// Arguments
///
/// - `windows` - tauri windows, get from `app.windows()`.
/// - `show` - Show or hide all windows.
pub async fn toggle_windows(windows: HashMap<String, Window>, show: bool) -> Result<()> {
    let mut ui = UI.lock().await;
    if show {
        for (_, window) in windows {
            window.show()?;
            window.set_focus()?;
        }
        ui.main_visible = true;
    } else {
        for (_, window) in windows {
            window.hide()?;
        }
        ui.main_visible = false
    }
    MSG_TX.lock().await.send(ConfigMsg::EmitUI).await?;
    Ok(())
}

/// Use for close previous core process
/// when tauri auto reload.
#[cfg(debug_assertions)]
pub fn debug_process() {
    use sysinfo::{ProcessExt, System, SystemExt};

    let s = System::new_all();
    s.processes_by_name("v2ray").for_each(|p| {
        println!("[DEV] --- Kill old v2ray core process {}", p.pid());
        p.kill();
    });
}
