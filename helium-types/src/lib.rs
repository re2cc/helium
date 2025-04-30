use serde::{Deserialize, Serialize};

#[cfg(feature = "backend")]
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

#[cfg_attr(feature = "backend", derive(FromRow))]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BasicItem {
    pub name: String,
    pub barcode: String,
    pub current_sell_price: u32,
    pub current_inventory: u32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ConfigItem {
    pub sell_quantity: u32,
}

#[derive(Deserialize)]
pub struct SearchParams {
    pub q: String,
}

#[derive(Deserialize)]
pub struct SelectProductParams {
    pub barcode: String,
}

#[derive(Deserialize)]
pub struct AddProductVariationParams {
    //todo
}

#[derive(Deserialize)]
struct ProductSpecs {
    //todo
}

#[derive(Deserialize)]
pub struct AddProductParams {
    pub variation_enabled: bool,
    pub variation_name: Option<String>,
    pub name: String,
    pub barcode: String,
}
