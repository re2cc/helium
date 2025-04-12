use api_server::run_migrations;
use axum::{
    extract::{Json, Query},
    routing::get,
    Router,
};
use std::sync::Arc;

use helium_types::*;
mod app_state;
use app_state::AppState;

mod schema;
use schema::posts;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let database_url = "F:/Pers/work/helium/helium_gui/api-server/persistent/db.sqlite";

    let state = Arc::new(AppState::new(database_url)?);
    run_migrations(state.pool.get().await?).await?;

    let app = Router::new()
        .route("/search", get(search))
        .route("/select_item", get(select_item))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on {} ...", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn search(
    Query(params): Query<SearchParams>,
    state: axum::extract::State<Arc<AppState>>,
) -> Json<Vec<BasicItem>> {
    println!("{}", params.q);
    todo!("Implement selection logic");
}

async fn select_item(
    Query(params): Query<SelectItemParams>,
    state: axum::extract::State<Arc<AppState>>,
) -> Json<CurrentItem> {
    println!("{}", params.barcode);
    todo!("Implement selection logic");
}
