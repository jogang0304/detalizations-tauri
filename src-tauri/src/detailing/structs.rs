use std::collections::HashMap;

#[derive(Debug)]
pub struct Columns {
    pub id: &'static str,
    pub name: &'static str,
    pub count: &'static str,
    pub price: &'static str,
    pub operation_date: &'static str,
    pub operation_type: &'static str,
    pub barcode: Option<&'static str>,
}

#[derive(Debug)]
pub struct OneOperationData {
    pub name: String,
    pub count: i64,
    pub price: f64,
    pub barcode: Option<String>,
}
#[derive(Debug)]
pub struct TablesInfo {
    pub start_date: String,
    pub end_date: String,
    pub marketplace: String,
}
impl Default for TablesInfo {
    fn default() -> TablesInfo {
        return TablesInfo {
            start_date: "".to_string(),
            end_date: "".to_string(),
            marketplace: "".to_string(),
        };
    }
}

#[derive(Debug)]
pub struct OperationTables {
    pub info: TablesInfo,
    pub send_table: HashMap<String, OneOperationData>,
    pub sale_table: HashMap<String, OneOperationData>,
    pub refund_table: HashMap<String, OneOperationData>,
    pub service_table: HashMap<String, OneOperationData>,
    pub other_table: HashMap<String, OneOperationData>,
}

#[derive(Debug)]
pub struct ReadTableInfo {
    pub file_path: String,
    pub sheet_index: i32,
    pub start_row: i32,
    pub table_name: &'static str,
    pub accept_words: &'static [&'static str],
    pub accept_any_word: bool,
    pub month: Option<i32>,
    pub accept_any_month: bool,
    pub add_rows_to_next_section: i32,
    pub date_format: Option<&'static str>,
    pub columns: Columns,
}
#[derive(Debug)]
pub struct ReadAdditionalInfo {
    pub marketplace_name: &'static str,
}
#[derive(Debug)]
pub struct ReadMarketplace {
    pub info: ReadAdditionalInfo,
    pub tables: Vec<ReadTableInfo>,
}
#[derive(Debug)]
pub struct WriteTableInfo {
    pub table_name: &'static str,
    pub data: HashMap<String, OneOperationData>,
}
#[derive(Debug)]
pub struct WriteAdditionalInfo<'a> {
    pub marketplace_name: &'a str,
    pub month: &'a str,
}
#[derive(Debug)]
pub struct WriteMarketplace<'a> {
    pub info: WriteAdditionalInfo<'a>,
    pub tables: Vec<WriteTableInfo>,
}

pub const MONTHS: [&str; 13] = [
    "любой",
    "январь",
    "февраль",
    "март",
    "апрель",
    "май",
    "июнь",
    "июль",
    "август",
    "сентябрь",
    "октябрь",
    "ноябрь",
    "декабрь",
];

pub enum MARKETPLACES {
    Yandex,
    Wildberries,
    Ozon,
    Another,
}
