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
