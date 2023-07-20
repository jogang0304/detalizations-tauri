use std::path::Path;

use crate::detailing::make_ozon_read_marketplace::make_ozon_read_marketplace;
use crate::detailing::make_wildberries_read_marketplace::make_wildberries_read_marketplace;
use crate::detailing::make_yandex_read_marketplace::make_yandex_read_marketplace;
use crate::detailing::read_excel::get_write_table_infos;
use crate::detailing::structs::{WriteAdditionalInfo, WriteMarketplace, MARKETPLACES, MONTHS};

pub fn read_marketplace(fp: &Path, marketplace_type: MARKETPLACES, month: i32) -> WriteMarketplace {
    let mut accept_any_month = false;
    let mut option_month = Some(month);
    if month <= 0 {
        accept_any_month = true;
        option_month = None;
    }
    let marketplace;
    let marketplace_name;
    if matches!(marketplace_type, MARKETPLACES::Yandex) {
        marketplace = make_yandex_read_marketplace(
            fp.to_str().unwrap().to_string(),
            option_month,
            accept_any_month,
        );
        marketplace_name = "Яндекс";
    } else if matches!(marketplace_type, MARKETPLACES::Wildberries) {
        marketplace = make_wildberries_read_marketplace(
            fp.to_str().unwrap_or("").to_string(),
            option_month,
            accept_any_month,
        );
        marketplace_name = "Вайлдберриз";
    } else if matches!(marketplace_type, MARKETPLACES::Ozon) {
        marketplace =
            make_ozon_read_marketplace(fp.to_str().unwrap_or("").to_string(), option_month, true);
        marketplace_name = "Озон";
    } else {
        return WriteMarketplace {
            info: WriteAdditionalInfo {
                marketplace_name: "Неизвестный маркетплейс",
                month: MONTHS[month as usize],
            },
            tables: vec![],
        };
    }
    let read_tables = get_write_table_infos(&marketplace.tables);
    let result = WriteMarketplace {
        info: WriteAdditionalInfo {
            marketplace_name: marketplace_name,
            month: MONTHS[month as usize],
        },
        tables: read_tables,
    };
    return result;
}
