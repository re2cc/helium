use std::vec;

#[derive(serde::Serialize, serde::Deserialize)]
struct Item {
    name: String,
    price: u32,
    quantity: u32,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![my_custom_command, search_product])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn my_custom_command(item: Item) {
    println!("Item name: {}", item.name);
}

#[tauri::command]
fn search_product(value: String) -> Vec<Item> {
    println!("Search value: {}", value);
    let mut results: Vec<Item> = Vec::new();
    results.push(Item {
        name: "Product 1".to_string(),
        price: 100,
        quantity: 10,
    });
    return results;
}
