use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct UniversalState {
    pub current_item: Option<CurrentItem>,
    pub item_list: Vec<CurrentItem>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CurrentItem {
    pub basic_item: BasicItem,
    pub config_item: ConfigItem,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct BasicItem {
    pub barcode: String,
    pub name: String,
    pub price: u32,
    pub available_quantity: u32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ConfigItem{
    pub sell_quantity: u32,
}

#[derive(Deserialize)]
pub struct SearchParams {
    pub q: String,
}

#[derive(Deserialize)]
pub struct SelectItemParams {
    pub barcode: String,
}
