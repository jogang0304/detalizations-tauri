// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn handle_confirm(input_file: &str, output_file: &str, marketplace: &str) -> (bool, String) {
    return (true, "asd".to_string());
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![handle_confirm])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
