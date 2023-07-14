use walkdir::WalkDir;

pub mod update_price;

pub fn process_price_getting(target_dir: &str) -> (bool, String) {
    for entry in WalkDir::new(target_dir)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let filename = entry.file_name().to_string_lossy();

        if filename.ends_with(".xlsm") {
            let filepath = entry.path().to_string_lossy();
            println!("{}", entry.path().to_string_lossy());
            update_price::update_price(&filepath);
        }
    }

    return (true, "asdf".to_string());
}
