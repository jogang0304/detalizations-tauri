// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::result;


mod files;
mod helpers;
mod make_ozon_read_marketplace;
mod make_wildberries_read_marketplace;
mod make_yandex_read_marketplace;
mod read;
mod read_excel;
mod structs;
mod write;

#[tauri::command]
fn handle_confirm(input_file: &str, output_file: &str, marketplace: &str) -> (bool, String) {
    let result = std::panic::catch_unwind(|| {
        helpers::process_detailing(input_file, output_file, marketplace)
    });
    if result.is_ok() {
        return result.unwrap();
    } else {
        return (false, "Проверьте выбор маркетплейса".to_string());
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![handle_confirm])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
