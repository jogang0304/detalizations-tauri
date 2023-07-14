use tauri::api::process::Command;

fn get_name_and_price(file: &str, to_find_price: &str, to_find_name: &str) -> (String, f32) {
    let cmd = Command::new_sidecar("getNameAndPrice")
        .expect("cant create sidecar")
        .args([file, to_find_price, to_find_name]);
    let output = cmd.output().expect("cant run cmd").stdout;

    let mut splitted = output.split_whitespace().collect::<Vec<&str>>();
    let price = splitted
        .pop()
        .expect("no output")
        .parse::<f32>()
        .expect("price not float");
    let name = splitted.join(" ");
    return (name, price);
}

pub fn update_price(file: &str) {
    let name_and_price = get_name_and_price(file, "СЕБЕСТОИМОСТЬ", "ПОЛНОЕ НАЗВАНИЕ ИЗДЕЛИЯ");
    println!("{}", name_and_price.0);
    println!("{}", name_and_price.1);
}
