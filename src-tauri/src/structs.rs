use tauri_plugin_http::reqwest::Client;

#[derive(Default)]
pub struct AppState {
    pub client: Client,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Item {
    pub name: String,
    pub price: u32,
    pub quantity: u32,
}
