use std::{collections::HashMap, i32};

use crate::structs::{Columns, OneOperationData, ReadTableInfo, WriteTableInfo};
use calamine::{open_workbook_auto, DataType, Range, Reader};
use chrono::{Datelike, NaiveDate};

fn column_letter_to_number(letter: &str) -> u32 {
    let mut answer = 0;
    let letters_in_alphabet: u32 = 26;
    for (i, c) in letter.char_indices() {
        let pos = (letter.chars().count() - 1 - i) as u32;
        answer += letters_in_alphabet.pow(pos) * (c as u32 - 'A' as u32 + 1);
        if pos == 0 {
            answer -= 1;
        }
    }
    return answer;
}

fn val(sheet: &Range<DataType>, letter: &str, number: i32) -> DataType {
    let number_column = column_letter_to_number(letter);
    let coordinate = ((number - 1) as u32, number_column);
    let data = sheet.get_value(coordinate);
    let result = data.unwrap_or(&DataType::Empty).to_owned();
    return result;
}
fn val_to_string(cell_value: DataType, default_value: &str) -> String {
    let result;
    if cell_value.is_string() {
        result = cell_value.get_string().unwrap().to_string();
    } else if cell_value.is_int() {
        result = cell_value.get_int().unwrap().to_string();
    } else if cell_value.is_float() {
        result = cell_value.get_float().unwrap().to_string();
    } else {
        result = default_value.to_string();
    }
    return result;
}

fn add_to_table(
    sheet: &Range<DataType>,
    row: i32,
    operations_table: &mut HashMap<String, OneOperationData>,
    columns: &Columns,
) {
    let id = val_to_string(val(sheet, columns.id, row), "-1");

    let name = val(sheet, columns.name, row)
        .get_string()
        .unwrap_or("")
        .to_string();
    let count_raw = val(sheet, columns.count, row);
    let price_raw = val(sheet, columns.price, row);
    if count_raw.is_empty() || price_raw.is_empty() {
        return;
    }

    let count = val_to_string(count_raw, "1").parse::<i64>().unwrap_or(1);
    let price = val_to_string(price_raw, "0").parse::<f64>().unwrap_or(0.0);
    let mut table_data = operations_table
        .entry(id)
        .or_insert_with(|| OneOperationData {
            name,
            price: 0.0,
            count: 0,
            barcode: None,
        });
    table_data.count += count;
    table_data.price += (price * 100.0).round() / 100.0;

    if columns.barcode.is_some() {
        let barcode = val_to_string(val(sheet, columns.barcode.unwrap(), row), "");
        table_data.barcode = Some(barcode);
    }
}

fn check_date<'a>(
    sheet: &Range<DataType>,
    row: i32,
    table: &'a ReadTableInfo,
    columns: &Columns,
) -> bool {
    if table.accept_any_month {
        return true;
    }
    let operation_date = val(sheet, columns.operation_date, row)
        .get_string()
        .unwrap_or("")
        .to_string();
    let date_formatted =
        NaiveDate::parse_from_str(&operation_date, table.date_format.unwrap()).unwrap();
    let month = date_formatted.month();
    return month as i32 == table.month.unwrap_or(0);
}

fn read_table<'a>(table: &'a ReadTableInfo) -> WriteTableInfo {
    let mut output_table: WriteTableInfo = WriteTableInfo {
        table_name: "",
        data: HashMap::new(),
    };
    output_table.table_name = table.table_name;
    let mut workbook = open_workbook_auto(table.file_path.as_str()).unwrap();
    let sheet = &workbook
        .worksheet_range_at(table.sheet_index as usize)
        .unwrap()
        .unwrap();
    let mut row = table.start_row;

    let columns = &table.columns;
    let mut null_rows = 0;
    while null_rows < 5 {
        let operation_type = val(sheet, columns.operation_type, row);
        if !table.accept_any_word && (operation_type.is_empty() || !operation_type.is_string()) {
            null_rows += 1;
            row += table.add_rows_to_next_section;
            continue;
        } else if table.accept_any_word
            || table
                .accept_words
                .contains(&operation_type.get_string().unwrap_or(""))
        {
            let name = val(sheet, columns.name, row)
                .get_string()
                .unwrap_or("-1")
                .to_string();
            println!(
                "{}{}: {} {}",
                row,
                columns.name,
                name,
                val(sheet, columns.name, row).is_empty()
            );
            if name == "-1" || name == "" || val(sheet, columns.name, row).is_empty() {
                break;
            }
            if check_date(sheet, row, table, &columns) {
                add_to_table(sheet, row, &mut output_table.data, &table.columns);
            }
        } else {
            row += 1;
            if table.accept_any_word {
                null_rows += 1;
            }
            continue;
        }
        row += 1;
    }

    return output_table;
}

pub fn get_write_table_infos(tables: &[ReadTableInfo]) -> Vec<WriteTableInfo> {
    let mut ouput_tables: Vec<WriteTableInfo> = vec![];

    for (_index, table) in tables.iter().enumerate() {
        ouput_tables.push(read_table(table));
    }

    return ouput_tables;
}
