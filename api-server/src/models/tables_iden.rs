use sea_query::{Iden, enum_def};

#[derive(Iden)]
pub enum Product {
    Table,
    Id,
    Name,
    Barcode,
    VariationEnabled,
    VariationName,
}

#[derive(Iden)]
pub enum ProductVariation {
    Table,
    Id,
    FullName,
    ShortName,
    Barcode,
    CurrentSellPrice,
    CurrentInventory,
    ProductId,
}
