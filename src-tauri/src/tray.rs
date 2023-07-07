use std::sync::atomic::Ordering;

use tauri::{
    async_runtime, AppHandle, CustomMenuItem, Manager, SystemTray, SystemTrayMenu,
    SystemTrayMenuItem,
};

use crate::{core::AVCore, utils::error::VError, CORE_SHUTDOWN};

/// Build new system tray.
pub fn new_tray() -> SystemTray {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let hide = CustomMenuItem::new("hide".to_string(), "Hide");
    let tray_menu = SystemTrayMenu::new()
        .add_item(hide)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);
    SystemTray::new().with_menu(tray_menu)
}

/// Handle system tray menu item click.
pub fn handle_tray_click(app: &AppHandle, id: String, core: &AVCore) {
    let item_handle = app.tray_handle().get_item(&id);
    match id.as_str() {
        "quit" => {
            let mut core = core.lock().expect("");
            CORE_SHUTDOWN.store(true, Ordering::Relaxed);
            core.exit().expect("Kill core failed");
            app.exit(0);
        }
        "hide" => {
            let windows = app.windows();
            let main_window = app.get_window("main").expect("Can not get main window");
            let task = async move {
                let main_visible = main_window.is_visible()?;
                if main_visible {
                    for (_, window) in windows {
                        window.hide()?;
                    }
                    item_handle.set_title("Show")?;
                } else {
                    for (_, window) in windows {
                        window.show()?;
                        window.set_focus()?;
                    }
                    item_handle.set_title("Hide")?;
                    // .expect("Can not set title tray title");
                }
                Ok::<(), VError>(())
            };
            async_runtime::spawn(task);
        }
        _ => {}
    }
}
