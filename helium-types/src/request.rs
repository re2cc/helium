use serde::Deserialize;

use crate::request_util::ProductSpecs;

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
pub struct AddProductParams {
    pub variation_enabled: bool,
    pub variation_name: Option<String>,
    pub product_specs: ProductSpecs,
    pub name: String,
    pub barcode: String,
}
