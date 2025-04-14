use std::sync::Arc;

use axum::{
    Router,
    extract::{Json, Query},
    routing::get,
};
use dotenvy::dotenv_override;

use api_server::run_migrations;
use helium_types::*;
mod app_state;
use app_state::AppState;
mod config_setter;
use config_setter::HeliumSettings;
mod schema;
use schema::posts;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv_override().ok();

    let settings = HeliumSettings::new();

    let state = Arc::new(AppState::new(&settings.database_path)?);
    run_migrations(state.pool.get().await?).await?;

    let app = Router::new()
        .route("/search", get(search))
        .route("/select_item", get(select_item))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", settings.port))
        .await
        .unwrap();
    println!("Listening on {} ...", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn search(
    Query(params): Query<SearchParams>,
    state: axum::extract::State<Arc<AppState>>,
) -> Json<Vec<BasicItem>> {
    println!("{}", params.q);
    todo!("Implement search logic");
}

async fn select_item(
    Query(params): Query<SelectItemParams>,
    state: axum::extract::State<Arc<AppState>>,
) -> Json<CurrentItem> {
    println!("{}", params.barcode);
    todo!("Implement selection logic");
}
