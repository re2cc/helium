use diesel::SqliteConnection;
use diesel_async::{
    pooled_connection::{deadpool::Pool, AsyncDieselConnectionManager},
    sync_connection_wrapper::SyncConnectionWrapper,
};

#[derive(Clone)]
pub struct AppState {
    pub pool: Pool<SyncConnectionWrapper<SqliteConnection>>,
}

impl AppState {
    pub fn new(
        database_file: &str,
    ) -> Result<Self, diesel_async::pooled_connection::deadpool::BuildError> {
        let config = AsyncDieselConnectionManager::<SyncConnectionWrapper<SqliteConnection>>::new(
            database_file,
        );
        let pool = Pool::builder(config).build()?;
        Ok(AppState { pool })
    }
}
