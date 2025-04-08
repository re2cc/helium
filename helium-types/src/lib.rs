#[derive(Default, Clone)]
pub struct AppState {
    pub client: tauri_plugin_http::reqwest::Client,
    pub universal_state: UniversalState,
}

#[derive(Default, serde::Serialize, serde::Deserialize, Clone)]
pub struct UniversalState {
    pub current_item: Option<CurrentItem>,
    pub item_list: Vec<CurrentItem>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct CurrentItem {
    pub basic_item: BasicItem,
    pub config_item: ConfigItem,
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct BasicItem {
    pub barcode: String,
    pub name: String,
    pub price: u32,
    pub available_quantity: u32,
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct ConfigItem{
    pub sell_quantity: u32,
}
