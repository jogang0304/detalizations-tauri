use crate::structs::{Columns, ReadAdditionalInfo, ReadMarketplace, ReadTableInfo};

pub fn make_yandex_read_marketplace<'a>(
    fp: String,
    month: Option<i32>,
    accept_any_month: bool,
) -> ReadMarketplace {
    let tables = vec![
        ReadTableInfo {
            file_path: fp.clone(),
            sheet_index: 1,
            start_row: 3,
            table_name: "Продажа",
            accept_words: &["Платёж покупателя"],
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
                operation_date: "H",
                operation_type: "S",
                barcode: None,
            },
        },
        ReadTableInfo {
            file_path: fp.clone(),
            sheet_index: 1,
            start_row: 3,
            table_name: "Оплата услуг",
            accept_words: &["Оплата услуг Яндекс.Маркета"],
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
                operation_date: "H",
                operation_type: "S",
                barcode: None,
            },
        },
        ReadTableInfo {
            file_path: fp.clone(),
            sheet_index: 1,
            start_row: 3,
            table_name: "Возврат",
            accept_words: &["Возврат платежа покупателя"],
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
                operation_date: "H",
                operation_type: "S",
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
