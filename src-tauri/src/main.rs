#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use networking::ServerState;

mod networking;

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .manage(ServerState(Default::default()))
        .invoke_handler(tauri::generate_handler![
            crate::networking::start_server,
            crate::networking::shutdown_server,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
