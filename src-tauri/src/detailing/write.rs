use std::collections::HashMap;
use std::path::Path;

use umya_spreadsheet::{new_file, writer, HorizontalAlignmentValues, NumberingFormat, Worksheet};

use crate::detailing::structs::{OneOperationData, WriteAdditionalInfo, WriteMarketplace};

const FORMAT_CURRENCY_RUB: &'static str = r###"#,##0_-"₽""###;

fn w(sheet: &mut Worksheet, column: &str, row: &i32, text: &str) {
    let column_index = umya_spreadsheet::helper::coordinate::column_index_from_string(column);
    let coordinate = (&column_index, &(row.to_owned() as u32));
    sheet.get_cell_mut(coordinate).set_value(text);
    let style = sheet.get_style_mut(coordinate);
    let font = style.get_font_mut();
    let font_name = font.get_font_name_mut();
    font_name.set_val("Calibri");
}

fn set_center_alignment(sheet: &mut Worksheet, column: &str, row: &i32) {
    let column_index = umya_spreadsheet::helper::coordinate::column_index_from_string(column);
    let coordinate = (&column_index, &(row.to_owned() as u32));
    let style = sheet.get_style_mut(coordinate);
    let alignment = style.get_alignment_mut();
    alignment.set_horizontal(HorizontalAlignmentValues::Center);
}
fn set_cell_format(sheet: &mut Worksheet, column: &str, row: &i32, format: &str) {
    let column_index = umya_spreadsheet::helper::coordinate::column_index_from_string(column);
    let coordinate = (&column_index, &(row.to_owned() as u32));
    let style = sheet.get_style_mut(coordinate);
    let cell_type = style.get_number_format_mut();
    cell_type.set_format_code(format);
}

fn write_table_header(sheet: &mut Worksheet, row: &mut i32, name: &str) {
    sheet.add_merge_cells(format!("A{}:F{}", row, row));
    w(sheet, "A", row, name);
    set_center_alignment(sheet, "A", row);
    *row += 1;
    w(sheet, "A", row, "№");
    w(sheet, "B", row, "Название");
    w(sheet, "C", row, "Артикул");
    w(sheet, "D", row, "Количество");
    w(sheet, "E", row, "Сумма реализации");
    w(sheet, "F", row, "Баркод");
    *row += 1;
}
fn write_table(
    sheet: &mut Worksheet,
    row: &mut i32,
    table: &Vec<(&String, &OneOperationData)>,
    name: &str,
) {
    if !table.is_empty() {
        write_table_header(sheet, row, name);
        let mut sum = 0.0;
        let mut sum_count = 0;
        let mut counter = 0;
        for (id, data) in table {
            counter += 1;
            w(sheet, "A", row, &counter.to_string());
            w(sheet, "B", row, &data.name);
            w(sheet, "C", row, &id);
            w(sheet, "D", row, &data.count.to_string());
            set_cell_format(sheet, "D", row, NumberingFormat::FORMAT_NUMBER);
            w(sheet, "E", row, &data.price.to_string());
            set_cell_format(sheet, "E", row, FORMAT_CURRENCY_RUB);
            if data.barcode.is_some() && data.barcode.as_ref().unwrap() != "" {
                w(sheet, "F", row, &data.barcode.as_ref().unwrap());
            }
            sum += data.price;
            sum_count += data.count;
            *row += 1;
        }
        w(sheet, "C", row, "Итого");
        w(sheet, "D", row, &sum_count.to_string());
        w(sheet, "E", row, &sum.to_string());
        set_cell_format(sheet, "E", row, FORMAT_CURRENCY_RUB);
        *row += 2;
    }
}
fn write_info(sheet: &mut Worksheet, row: &mut i32, info: &WriteAdditionalInfo) {
    sheet.add_merge_cells(format!("A{}:F{}", row, row));
    let info_text = format!(
        "Отчет по продажам на {} за {}",
        info.marketplace_name, info.month
    );
    w(sheet, "A", row, &info_text);
    *row += 2;
}

fn auto_width_columns(sheet: &mut Worksheet) {
    sheet.get_column_dimension_mut("A").set_width(2.5f64);
    sheet.get_column_dimension_mut("B").set_width(50f64);
    sheet.get_column_dimension_mut("C").set_width(20f64);
    sheet.get_column_dimension_mut("D").set_width(10f64);
    sheet.get_column_dimension_mut("E").set_width(16.5f64);
    sheet.get_column_dimension_mut("F").set_width(20f64);
}

pub fn write(marketplace: &WriteMarketplace, fp: &Path) -> (bool, String) {
    let mut workbook = new_file();
    let sheet = workbook.get_sheet_mut(&0).unwrap();
    sheet.set_name("Детализация");
    let mut row = 1;

    write_info(sheet, &mut row, &marketplace.info);

    for (_index, table) in marketplace.tables.iter().enumerate() {
        let mut sorted_data: Vec<_> = table.data.iter().collect();
        sorted_data.sort_by_key(|a| &a.1.name);

        write_table(sheet, &mut row, &sorted_data, table.table_name);
    }

    auto_width_columns(sheet);

    let file_write_result = writer::xlsx::write(&workbook, fp);
    let mut success = true;
    let mut return_text = fp.to_str().unwrap().to_string();
    if file_write_result.is_err() {
        return_text = "Ошибка при записи файла".to_string();
        success = false;
    }
    return (success, return_text);
}
