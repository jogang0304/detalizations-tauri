use crate::detailing::structs::{Columns, ReadAdditionalInfo, ReadMarketplace, ReadTableInfo};

pub fn make_wildberries_read_marketplace<'a>(
    fp: String,
    month: Option<i32>,
    accept_any_month: bool,
) -> ReadMarketplace {
    let tables = vec![
        ReadTableInfo {
            file_path: fp.clone(),
            sheet_index: 0,
            start_row: 2,
            table_name: "Продажа",
            accept_words: &["Продажа", "Корректная продажа"],
            accept_any_word: false,
            month,
            accept_any_month,
            add_rows_to_next_section: 3,
            date_format: Some("%Y-%m-%d"),
            columns: Columns {
                id: "F",
                name: "G",
                count: "N",
                price: "P",
                operation_date: "M",
                operation_type: "K",
                barcode: Some("I"),
            },
        },
        ReadTableInfo {
            file_path: fp.clone(),
            sheet_index: 0,
            start_row: 2,
            table_name: "Возврат",
            accept_words: &["Возврат"],
            accept_any_word: false,
            month,
            accept_any_month,
            add_rows_to_next_section: 3,
            date_format: Some("%Y-%m-%d"),
            columns: Columns {
                id: "F",
                name: "G",
                count: "N",
                price: "P",
                operation_date: "M",
                operation_type: "K",
                barcode: Some("I"),
            },
        },
    ];
    let additional_info = ReadAdditionalInfo {
        marketplace_name: "Вайлдберриз",
    };
    let result = ReadMarketplace {
        info: additional_info,
        tables: tables,
    };

    return result;
}
