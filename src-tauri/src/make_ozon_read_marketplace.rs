use crate::structs::{Columns, ReadAdditionalInfo, ReadMarketplace, ReadTableInfo};

pub fn make_ozon_read_marketplace<'a>(
    fp: String,
    month: Option<i32>,
    accept_any_month: bool,
) -> ReadMarketplace {
    let tables = vec![
        ReadTableInfo {
            file_path: fp.clone(),
            sheet_index: 0,
            start_row: 15,
            table_name: "Продажа",
            accept_words: &[],
            accept_any_word: true,
            month,
            accept_any_month,
            add_rows_to_next_section: 1,
            date_format: None,
            columns: Columns {
                id: "F",
                name: "D",
                count: "P",
                price: "Y",
                operation_date: "M",
                operation_type: "K",
                barcode: Some("M"),
            },
        },
        ReadTableInfo {
            file_path: fp.clone(),
            sheet_index: 0,
            start_row: 15,
            table_name: "Возврат",
            accept_words: &[],
            accept_any_word: true,
            month,
            accept_any_month,
            add_rows_to_next_section: 1,
            date_format: None,
            columns: Columns {
                id: "F",
                name: "D",
                count: "AD",
                price: "AH",
                operation_date: "M",
                operation_type: "K",
                barcode: Some("M"),
            },
        },
    ];
    let additional_info = ReadAdditionalInfo {
        marketplace_name: "Озон",
    };
    let result = ReadMarketplace {
        info: additional_info,
        tables: tables,
    };

    return result;
}
