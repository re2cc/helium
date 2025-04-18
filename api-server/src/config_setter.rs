use std::env;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(name="Helium server API", version="v0.0.1", about="Helium server API. For embedded usage open trough Helim GUI.", long_about = None)]
struct Args {
    #[arg(short, long = "database-file")]
    database_file: Option<String>,
    #[arg(short, long = "index-dir")]
    index_dir: Option<String>,
    #[arg(short, long = "port")]
    port: Option<u16>,
}

pub struct HeliumSettings {
    pub database_file_path: String,
    pub index_dir_path: String,
    pub port: u16,
}

impl HeliumSettings {
    pub fn new() -> Self {
        let args = Args::parse();
        let database_file_path = args.database_file.unwrap_or_else(|| {
            env::var("DATABASE_FILE").unwrap_or_else(|_| {
                panic!("Please set the DATABASE_FILE environment variable or use --database-file argument")
            })
        });
        let index_dir_path = args.index_dir.unwrap_or_else(|| {
            env::var("INDEX_DIR").unwrap_or_else(|_| {
                panic!("Please set the INDEX_DIR environment variable or use --index-dir argument")
            })
        });
        let port = args.port.unwrap_or_else(|| {
            env::var("PORT")
                .map(|p| {
                    p.parse::<u16>()
                        .expect("Invalid PORT environment variable format")
                })
                .unwrap_or_else(|_| {
                    panic!("Please set the PORT environment variable or use --port argument");
                })
        });
        HeliumSettings {
            database_file_path,
            index_dir_path,
            port,
        }
    }
}
