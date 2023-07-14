use crate::detailing::structs::{Columns, ReadAdditionalInfo, ReadMarketplace, ReadTableInfo};

pub fn make_yandex_read_marketplace<'a>(
    fp: String,
    month: Option<i32>,
    accept_any_month: bool,
) -> ReadMarketplace {
    let tables = vec![
        ReadTableInfo {
            file_path: fp.clone(),
            sheet_index: 2,
            start_row: 15,
            table_name: "Продажа",
            accept_words: &["Платёж покупателя"],
            accept_any_word: true,
            month,
            accept_any_month,
            add_rows_to_next_section: 3,
            date_format: Some("%d.%m.%Y"),
            columns: Columns {
                id: "D",
                name: "C",
                count: "F",
                price: "T",
                operation_date: "J",
                operation_type: "G",
                barcode: None,
            },
        },
        ReadTableInfo {
            file_path: fp.clone(),
            sheet_index: 3,
            start_row: 15,
            table_name: "Невыкупленные товары",
            accept_words: &["Оплата услуг Яндекс.Маркета"],
            accept_any_word: true,
            month,
            accept_any_month,
            add_rows_to_next_section: 3,
            date_format: Some("%d.%m.%Y"),
            columns: Columns {
                id: "D",
                name: "C",
                count: "F",
                price: "U",
                operation_date: "K",
                operation_type: "G",
                barcode: None,
            },
        },
        ReadTableInfo {
            file_path: fp.clone(),
            sheet_index: 4,
            start_row: 15,
            table_name: "Возврат",
            accept_words: &["Возврат платежа покупателя"],
            accept_any_word: true,
            month,
            accept_any_month,
            add_rows_to_next_section: 3,
            date_format: Some("%d.%m.%Y"),
            columns: Columns {
                id: "D",
                name: "C",
                count: "F",
                price: "U",
                operation_date: "J",
                operation_type: "G",
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
