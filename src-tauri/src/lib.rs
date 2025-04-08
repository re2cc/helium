use std::vec;
use tauri::Manager;
use tauri_plugin_http::reqwest;
use tokio::sync::Mutex;
use tauri::State;

use helium_types;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_http::init())
        .setup(|app| {
            app.manage(Mutex::new(helium_types::AppState {
                client: reqwest::Client::new(),
                universal_state: helium_types::UniversalState {
                    current_item: None,
                    item_list: Vec::new(),
                },
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
        .invoke_handler(tauri::generate_handler![select_item, search_product, add_product])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn select_item(barcode: String, state: State<'_, Mutex<helium_types::AppState>>) -> Result<helium_types::CurrentItem, ()> {
    let mut state = state.lock().await;
    state.universal_state.current_item = Some(helium_types::CurrentItem {
        basic_item: helium_types::BasicItem {
            barcode: barcode,
            name: "Product Name".to_string(),
            price: 100,
            available_quantity: 10,
        },
        config_item: helium_types::ConfigItem {
            sell_quantity: 1,
        },
    });
    Ok(state.universal_state.current_item.clone().unwrap())
}

#[tauri::command]
async fn search_product(search_value: String, state: State<'_, Mutex<helium_types::AppState>>) -> Result<Vec<helium_types::BasicItem>, ()> {
    let response: reqwest::Response;

    {
        let state = state.lock().await;
        response = state.client.get("http://example.com").send().await.unwrap();
    }

    println!("Search value: {}", search_value);
    let mut results: Vec<helium_types::BasicItem> = Vec::new();
    results.push(helium_types::BasicItem {
        barcode: "1234567890123".to_string(),
        name: "Product 1".to_string(),
        price: 100,
        available_quantity: 10,
    });

    Ok(results)
}

#[tauri::command]
async fn add_product(barcode: String, item_config: helium_types::ConfigItem, state: State<'_, Mutex<helium_types::AppState>>) -> Result<bool, ()> {
    let mut state = state.lock().await;

    println!("Added: {}", barcode);

    state.universal_state.item_list.push(helium_types::CurrentItem {
        basic_item: helium_types::BasicItem {
            barcode: barcode,
            name: "Product Name".to_string(),
            price: 100,
            available_quantity: 10,
        },
        config_item: item_config.clone(),
    });

    Ok(true)
}
