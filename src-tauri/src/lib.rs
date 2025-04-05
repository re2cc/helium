use std::vec;
use tauri::Manager;
use tauri_plugin_http::reqwest;
use tokio::sync::Mutex;
use tauri::State;

mod structs;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_http::init())
        .setup(|app| {
            app.manage(Mutex::new(structs::AppState {
                client: reqwest::Client::new(),
                current_item: None,
            }));
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![select_item, search_product])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn select_item(barcode: String, state: State<'_, Mutex<structs::AppState>>) -> Result<structs::CurrentItem, ()> {
    let mut state = state.lock().await;
    state.current_item = Some(structs::CurrentItem {
        basic_item: structs::BasicItem {
            barcode: barcode,
            name: "Product Name".to_string(),
            price: 100,
            available_quantity: 10,
        },
        config_item: structs::ConfigItem {
            sell_quantity: 1,
        },
    });
    Ok(state.current_item.clone().unwrap())
}

#[tauri::command]
async fn search_product(search_value: String, state: State<'_, Mutex<structs::AppState>>) -> Result<Vec<structs::BasicItem>, ()> {
    let state = state.lock().await;
    let response = state.client.get("http://google.com").send().await.unwrap();
    println!("Response: {}", response.text().await.unwrap());

    println!("Search value: {}", search_value);
    let mut results: Vec<structs::BasicItem> = Vec::new();
    results.push(structs::BasicItem {
        barcode: "1234567890123".to_string(),
        name: "Product 1".to_string(),
        price: 100,
        available_quantity: 10,
    });

    Ok(results)
}
