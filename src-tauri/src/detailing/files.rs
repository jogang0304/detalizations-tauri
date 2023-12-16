use std::fs::File;
use std::path::Path;
use std::process::Command;
use std::{env, fs};

pub fn file_exists(fp: &str) -> bool {
    let ans = Path::new(fp).exists();
    return ans;
}
pub fn create_file(fp: &str) {
    let _file = File::create(fp).expect("Ошибка при создании файла");
}
pub fn delete_file(fp: &str) {
    let _file = fs::remove_file(fp).expect("Ошибка при удалении файла");
}
pub fn check_extension(fp: &str, expected_extensions: &[&str]) -> bool {
    let file_extension_raw = Path::new(fp).extension();
    let mut passes = false;
    if file_extension_raw.is_none() {
        passes = false;
    } else {
        let file_extension = file_extension_raw.unwrap().to_str().unwrap();
        for x in expected_extensions {
            if file_extension == x.to_string() {
                passes = true;
                break;
            }
        }
    }
    return passes;
}
pub fn open_explorer(fp: &str) {
    let system_os = env::consts::OS;
    let command;
    let mut path_to_file_or_directory = fp;
    if system_os == "windows" {
        command = "explorer";
        if Path::new(fp).is_file() {
            let folder_path = Path::new(fp).parent().unwrap();
            path_to_file_or_directory = folder_path.to_str().unwrap();
        }
    } else if system_os == "macos" {
        command = "open";
    } else {
        command = "xdg-open";
    }
    Command::new(command)
        .arg(path_to_file_or_directory)
        .spawn()
        .unwrap();
}
