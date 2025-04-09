use std::sync::Arc;
use axum::{
    extract::{Query, Json},
    routing::get,
    Router,
};

use helium_types::{*};

mod app_state;
use app_state::AppState;


#[tokio::main]
async fn main() {
    let db = sea_orm::Database::connect("sqlite://persistent/db.sqlite?mode=rwc").await.unwrap();

    let state = Arc::new(AppState::new(db));

    let app = Router::new()
        .route("/search", get(search))
        .route("/select_item", get(select_item))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on {} ...", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn search(Query(params): Query<SearchParams>, state: axum::extract::State<Arc<AppState>>) -> Json<Vec<BasicItem>> {
    println!("{}", params.q);
    todo!("Implement selection logic");
}

async fn select_item(Query(params): Query<SelectItemParams>, state: axum::extract::State<Arc<AppState>>) -> Json<CurrentItem> {
    println!("{}", params.barcode);
    todo!("Implement selection logic");
}
