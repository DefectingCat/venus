use tauri::{AppHandle, CustomMenuItem, Manager, SystemTray, SystemTrayMenu, SystemTrayMenuItem};

use crate::core::AVCore;

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
            core.exit().expect("");
            app.exit(0);
        }
        "hide" => {
            let main_window = app.get_window("main").expect("Can not get main window");
            let main_visible = main_window
                .is_visible()
                .expect("Failed to detect window visible");
            if main_visible {
                main_window.hide().expect("Can not hide main window");
                item_handle
                    .set_title("Show")
                    .expect("Can not set title to menu item");
            } else {
                main_window.show().expect("Can not show main window");
                item_handle
                    .set_title("Hide")
                    .expect("Can not set title tray title");
            }
        }
        _ => {}
    }
}
