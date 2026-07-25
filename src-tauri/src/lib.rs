mod zerotier;

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
            Ok(())
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
