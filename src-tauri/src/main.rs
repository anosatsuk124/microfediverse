#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use networking::ServerState;
use tauri::Manager;

mod activitypub;
mod database;
mod error;
mod networking;

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .setup(|app| {
            #[cfg(debug_assertions)]
            {
                let window = app.get_window("main").unwrap();
                window.open_devtools();
            }
            tokio::spawn(async {
                crate::database::db_main().await;
            });
            Ok(())
        })
        .manage(ServerState(Default::default()))
        .invoke_handler(tauri::generate_handler![
            crate::activitypub::object::test,
            crate::networking::start_server,
            crate::networking::shutdown_server,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
