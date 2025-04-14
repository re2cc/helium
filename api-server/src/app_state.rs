use diesel::SqliteConnection;
use diesel_async::{
    pooled_connection::{AsyncDieselConnectionManager, deadpool::Pool},
    sync_connection_wrapper::SyncConnectionWrapper,
};
use std::fs::OpenOptions;

#[derive(Clone)]
pub struct AppState {
    pub pool: Pool<SyncConnectionWrapper<SqliteConnection>>,
}

impl AppState {
    pub fn new(database_file: &str) -> Result<Self, Box<dyn std::error::Error>> {
        OpenOptions::new()
            .append(true)
            .create(true)
            .open(database_file)?;

        let config = AsyncDieselConnectionManager::<SyncConnectionWrapper<SqliteConnection>>::new(
            database_file,
        );
        let pool = Pool::builder(config).build()?;
        Ok(AppState { pool })
    }
}
