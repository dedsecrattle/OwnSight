#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::analyze_snippet,
            commands::analyze_file,
            commands::query_why_cant_use,
            commands::query_where_moved,
            commands::query_what_borrows,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
