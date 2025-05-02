use serde::{Deserialize, Serialize};

#[cfg(feature = "frontend")]
use ts_rs::TS;

#[cfg(feature = "backend")]
use sqlx::FromRow;

#[cfg_attr(feature = "backend", derive(FromRow))]
#[cfg_attr(feature = "frontend", derive(TS), ts(export))]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BasicItem {
    pub name: String,
    pub barcode: String,
    pub current_sell_price: u32,
    pub current_inventory: u32,
}

#[cfg_attr(feature = "backend", derive(FromRow))]
#[cfg_attr(feature = "frontend", derive(TS), ts(export))]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FullItem {
    // TODO
}
