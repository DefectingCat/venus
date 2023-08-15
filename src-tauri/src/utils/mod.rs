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

/// Use for close previous core process
/// when tauri auto reload.
#[cfg(debug_assertions)]
pub fn debug_process() -> VResult<()> {
    use sysinfo::{ProcessExt, System, SystemExt};

    let s = System::new_all();
    s.processes_by_name("v2ray").for_each(|p| {
        println!("[DEV] --- Kill old v2ray core process {}", p.pid());
        p.kill();
    });

    Ok(())
}
