// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod detailing;
mod price_getter;

#[tauri::command]
fn handle_detailing_confirm(
    input_file: &str,
    output_file: &str,
    marketplace: &str,
) -> (bool, String) {
    let result = std::panic::catch_unwind(|| {
        detailing::process_detailing(input_file, output_file, marketplace)
    });
    if result.is_ok() {
        return result.unwrap();
    } else {
        return (false, "Проверьте выбор маркетплейса".to_string());
    }
}

#[tauri::command]
fn handle_price_confirm(target_dir: &str) -> (bool, String) {
    let result = std::panic::catch_unwind(|| price_getter::process_price_getting(target_dir));
    if result.is_ok() {
        return result.unwrap();
    } else {
        return (false, "Проверьте выбор маркетплейса".to_string());
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            handle_detailing_confirm,
            handle_price_confirm
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
