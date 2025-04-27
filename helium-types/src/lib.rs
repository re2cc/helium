use serde::{Deserialize, Serialize};
use sqlx::FromRow;

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

#[derive(Serialize, Deserialize, FromRow, Debug, Clone)]
pub struct BasicItem {
    pub name: String,
    pub barcode: String,
    pub current_sell_price: u32,
    pub current_inventory: u32,
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

#[derive(Deserialize)]
pub struct AddItemParams {
    pub name: String,
}
