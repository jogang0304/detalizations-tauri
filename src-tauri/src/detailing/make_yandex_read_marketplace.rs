use crate::detailing::structs::{Columns, ReadAdditionalInfo, ReadMarketplace, ReadTableInfo};

pub fn make_yandex_read_marketplace<'a>(
    fp: String,
    month: Option<i32>,
    accept_any_month: bool,
) -> ReadMarketplace {
    let tables = vec![
        ReadTableInfo {
            file_path: fp.clone(),
            sheet_index: 1,
            start_row: 5,
            table_name: "Получено от потребителей",
            accept_words: &["Начисление"],
            accept_any_word: false,
            month,
            accept_any_month,
            add_rows_to_next_section: 3,
            date_format: Some("%d.%m.%Y"),
            columns: Columns {
                id: "N",
                name: "O",
                count: "P",
                price: "Q",
                operation_date: "U",
                operation_type: "R",
                barcode: None,
            },
        },
        ReadTableInfo {
            file_path: fp.clone(),
            sheet_index: 2,
            start_row: 5,
            table_name: "Возврат",
            accept_words: &["Возврат"],
            accept_any_word: false,
            month,
            accept_any_month,
            add_rows_to_next_section: 3,
            date_format: Some("%d.%m.%Y"),
            columns: Columns {
                id: "N",
                name: "O",
                count: "P",
                price: "Q",
                operation_date: "L",
                operation_type: "R",
                barcode: None,
            },
        },
        ReadTableInfo {
            file_path: fp.clone(),
            sheet_index: 3,
            start_row: 5,
            table_name: "Удержание",
            accept_words: &["Удержание"],
            accept_any_word: false,
            month,
            accept_any_month,
            add_rows_to_next_section: 3,
            date_format: Some("%d.%m.%Y"),
            columns: Columns {
                id: "N",
                name: "O",
                count: "P",
                price: "Q",
                operation_date: "U",
                operation_type: "R",
                barcode: None,
            },
        },
    ];
    let additional_info = ReadAdditionalInfo {
        marketplace_name: "Яндекс",
    };
    let result = ReadMarketplace {
        info: additional_info,
        tables: tables,
    };

    return result;
}
