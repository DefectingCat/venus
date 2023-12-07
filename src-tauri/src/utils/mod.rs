use anyhow::{anyhow, Result};
use tauri::Manager;
use tauri::{App, Window};

pub mod consts;
pub mod error;

/// Get main window by app
pub fn get_main_window(app: &App) -> Result<Window> {
    let window = app
        .get_window("main")
        .ok_or(anyhow!("Can not get main window"))?;
    Ok(window)
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
