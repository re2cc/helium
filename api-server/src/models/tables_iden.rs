use sea_query::Iden;

#[derive(Iden)]
pub enum ProductVariation {
    Table,
    Id,
    Name,
    Barcode,
    CurrentSellPrice,
    CurrentInventory,
    ProductId,
}
