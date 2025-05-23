use std::sync::Arc;

use axum::{
    Router,
    extract::{Json, Query},
    routing::{get, post},
};
use dotenvy::dotenv_override;
use sea_query::{Expr, Query as SeaQuery, SqliteQueryBuilder};
use sea_query_binder::SqlxBinder;
use tantivy::{
    TantivyDocument, Term,
    collector::{Count, TopDocs},
    doc,
    query::FuzzyTermQuery,
    schema::Value,
};

use helium_types::request::*;
use helium_types::response::*;
mod models;
use models::app_state::AppState;
use models::config_setter::HeliumSettings;
use models::tables_iden::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv_override().ok();

    let settings = HeliumSettings::new();

    let state =
        Arc::new(AppState::new(&settings.database_file_path, &settings.index_dir_path).await?);
    state.run_migrations().await?;

    let app = Router::new()
        .route("/search", get(search))
        .route("/select_item", get(select_product))
        .route("/add_item", post(add_product))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", settings.port)).await?;
    println!("Listening on {} ...", listener.local_addr()?);
    axum::serve(listener, app).await?;

    Ok(())
}

#[axum::debug_handler]
async fn search(
    state: axum::extract::State<Arc<AppState>>,
    Query(params): Query<SearchParams>,
) -> Json<Vec<BasicItem>> {
    let searcher = state.searcher();
    let term = Term::from_field_text(state.index_fields.name, &params.q);
    let query = FuzzyTermQuery::new(term, 2, true);
    let (top_docs, count) = searcher
        .search(&query, &(TopDocs::with_limit(10), Count))
        .unwrap();

    let ids: Vec<u64> = top_docs
        .into_iter()
        .filter_map(|(score, doc_address)| {
            let retrieved_doc: TantivyDocument = searcher.doc(doc_address).unwrap(); // Handle potential error
            retrieved_doc
                .get_first(state.index_fields.id)
                .and_then(|v| v.as_u64())
        })
        .collect();

    // Esentially, this is the same as the code, but less efficient
    // its here just to remember what the code does
    // let mut ids = Vec::new();
    // for (score, doc_address) in top_docs {
    //     let retrieved_doc: TantivyDocument = searcher.doc(doc_address).unwrap();
    //     let id = retrieved_doc.get_first(state.index_fields.id).and_then(|v| v.as_u64()).unwrap();
    //     ids.push(id);
    // }

    if ids.is_empty() {
        return Json(vec![]);
    }

    let (sql, values) = SeaQuery::select()
        .column(ProductVariation::FullName)
        .column(ProductVariation::Barcode)
        .column(ProductVariation::CurrentSellPrice)
        .column(ProductVariation::CurrentInventory)
        .from(ProductVariation::Table)
        .and_where(Expr::col(ProductVariation::Id).is_in(ids))
        .build_sqlx(SqliteQueryBuilder);

    let rows = sqlx::query_as_with::<_, BasicItem, _>(&sql, values)
        .fetch_all(&state.pool)
        .await
        .unwrap();

    return Json(rows);
}

#[axum::debug_handler]
async fn select_product(
    state: axum::extract::State<Arc<AppState>>,
    Query(params): Query<SelectProductParams>,
) -> Json<FullItem> {
    let conn = state.pool.acquire().await.unwrap();
    println!("{}", params.barcode);
    todo!("Implement selection logic");
}

#[axum::debug_handler]
async fn add_product(
    state: axum::extract::State<Arc<AppState>>,
    Json(params): Json<AddProductParams>,
) -> Json<bool> {
    let (sql, values) = SeaQuery::insert()
        .into_table(Product::Table)
        .columns([
            Product::Name,
            Product::Barcode,
            Product::VariationEnabled,
            Product::VariationName,
        ])
        .values([
            params.name.into(),
            params.barcode.into(),
            params.variation_enabled.into(),
            params.variation_name.into(),
        ])
        .unwrap()
        .build_sqlx(SqliteQueryBuilder);

    // let result = sqlx::query_with(&sql, values).execute(&state.pool).await.unwrap();
    // result.last_insert_rowid();

    // let new_doc = doc!(
    //     state.index_fields.name => params.name,
    //     state.index_fields.id => u64::from(params.name.len() as u64),
    // );
    // {
    //     // I could delete the let statement and use state.index_writer.lock().await directly, but I prefer to keep it to remember
    //     // that i can use it for more than one document
    //     let mut index_writer = state.index_writer.lock().await;
    //     index_writer
    //         .add_document(new_doc)
    //         .unwrap();
    //     index_writer.commit().unwrap();
    // }
    // todo!("Implement database addition logic");
    return Json(true);
}
