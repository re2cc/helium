// use diesel::sqlite::Sqlite;
// use diesel_async::{async_connection_wrapper::AsyncConnectionWrapper, AsyncConnection};
// use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

// pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

// pub async fn run_migrations<A>(async_connection: A) -> Result<(), Box<dyn std::error::Error>>
// where
//     A: AsyncConnection<Backend = Sqlite> + 'static,
// {
//     let mut async_wrapper: AsyncConnectionWrapper<A> =
//         AsyncConnectionWrapper::from(async_connection);

//     tokio::task::spawn_blocking(move || {
//         async_wrapper.run_pending_migrations(MIGRATIONS).unwrap();
//     })
//     .await
//     .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
// }
