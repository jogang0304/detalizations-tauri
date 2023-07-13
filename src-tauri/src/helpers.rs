use std::path::Path;

use crate::{
    files::{check_extension, create_file, delete_file, file_exists},
    read,
    structs::MARKETPLACES,
    write::{self, write},
};

pub fn process_detailing(
    source_file: &str,
    output_file: &str,
    marketplace: &str,
) -> (bool, String) {
    let mut output_file = output_file.to_owned();
    if !file_exists(&source_file) {
        return (false, format!("Исходный файл не существует"));
    }
    if !check_extension(source_file, &["xls", "xlsx", "xlsm"]) {
        return (false, format!("У исходного файла неверное расширение"));
    }
    let marketplace_type = if marketplace == "Yandex" {
        MARKETPLACES::Yandex
    } else if marketplace == "Wildberries" {
        MARKETPLACES::Wildberries
    } else if marketplace == "Ozon" {
        MARKETPLACES::Ozon
    } else {
        MARKETPLACES::Another
    };

    let output_extension = Path::new(&output_file).extension();
    if output_extension.is_none() {
        output_file += ".xlsx";
    } else {
        let passes = check_extension(&output_file, &vec!["xlsx", "xls", "xlsm"]);
        if !passes {
            return (false, format!("У выходного файла неверное расширение"));
        }
    }

    let source_fp = Path::new(source_file);
    let output_fp = Path::new(&output_file);

    let tables = read::read_marketplace(source_fp, marketplace_type);

    let result = write::write(&tables, output_fp);

    return result;
}

pub fn create_output_file(output_file: &str) {
    if !file_exists(&output_file) {
        create_file(output_file);
    }
}
