// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


mod helpers;
mod files;
mod read;
mod make_ozon_read_marketplace;
mod make_wildberries_read_marketplace;
mod make_yandex_read_marketplace;
mod structs;
mod read_excel;
mod write;



#[tauri::command]
fn handle_confirm(input_file: &str, output_file: &str, marketplace: &str) -> (bool, String) {
    helpers::create_output_file(output_file);
    return helpers::process_detailing(input_file, output_file, marketplace);


    // println!("{} {} {}", input_file, output_file, marketplace);
    // return (true, "asd".to_string());
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![handle_confirm])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
