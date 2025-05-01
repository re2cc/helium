use serde::{Deserialize, Serialize};

use helium_types::response::BasicItem;

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
pub struct ConfigItem {
    pub sell_quantity: u32,
}
