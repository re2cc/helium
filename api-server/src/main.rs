use std::sync::Arc;

use axum::{
    Router,
    extract::{Json, Query},
    routing::{get, post},
};
use dotenvy::dotenv_override;

use helium_types::*;
mod app_state;
use app_state::AppState;
mod config_setter;
use config_setter::HeliumSettings;
use tantivy::doc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv_override().ok();

    let settings = HeliumSettings::new();

    let state =
        Arc::new(AppState::new(&settings.database_file_path, &settings.index_dir_path).await?);
    state.run_migrations().await?;

    let app = Router::new()
        .route("/search", get(search))
        .route("/select_item", get(select_item))
        .route("/add_item", post(add_item))
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
    println!("{}", params.q);
    todo!("Implement search logic");
}

#[axum::debug_handler]
async fn select_item(
    state: axum::extract::State<Arc<AppState>>,
    Query(params): Query<SelectItemParams>,
) -> Json<CurrentItem> {
    let conn = state.pool.acquire().await.unwrap();
    println!("{}", params.barcode);
    todo!("Implement selection logic");
}

#[axum::debug_handler]
async fn add_item(
    state: axum::extract::State<Arc<AppState>>,
    Json(params): Json<SelectItemParams>,
) -> Json<CurrentItem> {
    {
        // I could delete the let statement and use state.index_writer.lock().await directly, but I prefer to keep it to remember
        // that i can use it for more than one document
        let mut index_writer = state.index_writer.lock().await;
        index_writer
            .add_document(doc!(
                state.index_fields.name => params.barcode,
            ))
            .unwrap();
        index_writer.commit().unwrap();
    }
    println!("{}", params.barcode);
    todo!("Implement selection logic");
}
