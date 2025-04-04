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
                client: reqwest::Client::new()
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
        .invoke_handler(tauri::generate_handler![my_custom_command, search_product])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn my_custom_command(item: structs::Item) {
    println!("Item name: {}", item.name);
}

#[tauri::command]
async fn search_product(search_value: String, state: State<'_, Mutex<structs::AppState>>) -> Result<Vec<structs::Item>, ()> {
    let mut state = state.lock().await;
    let response = state.client.get("http://google.com").send().await.unwrap();
    println!("Response: {}", response.text().await.unwrap());

    println!("Search value: {}", search_value);
    let mut results: Vec<structs::Item> = Vec::new();
    results.push(structs::Item {
        name: "Product 1".to_string(),
        price: 100,
        quantity: 10,
    });

    Ok(results)
}
