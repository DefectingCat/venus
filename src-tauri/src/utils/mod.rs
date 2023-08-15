use std::collections::HashMap;

use tauri::Manager;
use tauri::{App, Window};

use self::error::{VError, VResult};

pub mod error;

/// Get main window by app
pub fn get_main_window(app: &App) -> VResult<Window> {
    let window = app
        .get_window("main")
        .ok_or(VError::WindowError("Can not get main window"))?;
    Ok(window)
}

/// Toggle all windows visible
///
/// Arguments
///
/// - `windows` - tauri windows, get from `app.windows()`.
/// - `show` - Show or hide all windows.
pub fn toggle_windows(windows: HashMap<String, Window>, show: bool) -> VResult<()> {
    if show {
        for (_, window) in windows {
            window.show()?;
            window.set_focus()?;
        }
    } else {
        for (_, window) in windows {
            window.hide()?;
        }
    }
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
