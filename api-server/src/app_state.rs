use sqlx::{Pool, Sqlite, migrate::MigrateError, sqlite::SqlitePoolOptions};
use std::fs::OpenOptions;

#[derive(Clone)]
pub struct AppState {
    pub pool: Pool<Sqlite>,
}

impl AppState {
    pub async fn new(database_file: &str) -> Result<Self, Box<dyn std::error::Error>> {
        // DB file creation if not exists
        OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(database_file)?;

        let pool = SqlitePoolOptions::new()
            .min_connections(1)
            .max_connections(3)
            .connect(&format!("sqlite://{}", database_file))
            .await?;

        Ok(AppState { pool })
    }

    pub async fn run_migrations(&self) -> Result<(), MigrateError> {
        sqlx::migrate!().run(&self.pool).await?;
        Ok(())
    }
}
