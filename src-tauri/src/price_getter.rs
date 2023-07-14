use walkdir::WalkDir;

use self::structs::RawPrice;

pub mod structs;
pub mod get_price;
pub mod write_price_list;

fn get_price_list(target_dir: &str) -> Vec<RawPrice> {
    let mut price_list = Vec::new();
    for entry in WalkDir::new(target_dir)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let filename = entry.file_name().to_string_lossy();

        if filename.ends_with(".xlsm") {
            let filepath = entry.path().to_string_lossy();
            println!("{}", entry.path().to_string_lossy());
            let price_entry = get_price::get_entry(&filepath);
            price_list.push(price_entry);
        }
    }
    return price_list;
}

pub fn process_price_getting(target_dir: &str) -> (bool, String) {
    let price_list = get_price_list(target_dir);
    return (true, "Себестоимости обработаны".to_string());
}
