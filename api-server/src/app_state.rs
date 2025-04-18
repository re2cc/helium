use sqlx::{Pool, Sqlite, migrate::MigrateError, sqlite::SqlitePoolOptions};
use std::{fs::OpenOptions, path::Path};
use tantivy::{Index, IndexWriter, ReloadPolicy, Searcher, directory::MmapDirectory, schema::*};
use tokio::{fs, sync::Mutex};

pub struct AppState {
    pub pool: Pool<Sqlite>,
    pub index_writer: Mutex<IndexWriter>,
    pub index_fields: IndexFields,
    index_reader: tantivy::IndexReader,
}

pub struct IndexFields {
    pub name: Field,
}

impl AppState {
    pub async fn new(
        database_file_path_str: &str,
        index_dir_path_str: &str,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        // Create path representation for the index directory
        let database_file_path = Path::new(database_file_path_str);
        let index_dir_path = Path::new(index_dir_path_str);

        // DB file creation if not exists
        OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(database_file_path)?;

        // Create index directory if it doesn't exist
        fs::create_dir_all(&index_dir_path).await?;

        // Create SQLite connection pool
        let pool = SqlitePoolOptions::new()
            .min_connections(1)
            .max_connections(3)
            .connect(&format!("sqlite://{}", database_file_path_str))
            .await?;

        // Create tantivy index, index writer, and reader
        let mut schema_builder = Schema::builder();
        schema_builder.add_text_field("name", TEXT | STORED);
        let schema = schema_builder.build();
        let index = Index::open_or_create(MmapDirectory::open(index_dir_path)?, schema.clone())?;
        let index_writer = Mutex::new(index.writer(50_000_000)?);
        let reader = index
            .reader_builder()
            .reload_policy(ReloadPolicy::OnCommitWithDelay)
            .try_into()?;

        let index_fields = IndexFields {
            name: schema.get_field("name")?,
        };

        Ok(AppState {
            pool,
            index_writer,
            index_fields,
            index_reader: reader,
        })
    }

    pub async fn run_migrations(&self) -> Result<(), MigrateError> {
        sqlx::migrate!().run(&self.pool).await?;
        Ok(())
    }

    pub fn searcher(&self) -> Searcher {
        self.index_reader.searcher()
    }
}
