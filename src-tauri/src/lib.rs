mod zerotier;

use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, TrayIconBuilder, TrayIconEvent},
    Manager,
};

fn show_window(app: &tauri::AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.unminimize();
        let _ = window.show();
        let _ = window.set_focus();
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }

            let show = MenuItem::with_id(app, "show", "Mostrar ZeroDesk", true, None::<&str>)?;
            let quit = MenuItem::with_id(app, "quit", "Salir", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show, &quit])?;

            TrayIconBuilder::with_id("main")
                .icon(app.default_window_icon().unwrap().clone())
                .tooltip("ZeroDesk - ZeroTier Manager")
                .menu(&menu)
                .show_menu_on_left_click(true)
                .on_menu_event(|app, event| match event.id().as_ref() {
                    "show" => show_window(app),
                    "quit" => app.exit(0),
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        ..
                    } = event
                    {
                        show_window(tray.app_handle());
                    }
                })
                .build(app)?;

            Ok(())
        })
        .on_window_event(|window, event| match event {
            tauri::WindowEvent::Resized(_) => {
                if window.is_minimized().unwrap_or(false) {
                    let _ = window.hide();
                }
            }
            tauri::WindowEvent::CloseRequested { api, .. } => {
                api.prevent_close();
                window.app_handle().exit(0);
            }
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![
            zerotier::get_status,
            zerotier::list_networks,
            zerotier::join_network,
            zerotier::leave_network,
            zerotier::list_peers,
            zerotier::get_network_info,
            zerotier::is_service_running,
            zerotier::start_service,
            zerotier::stop_service,
            zerotier::toggle_autoconnect,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
