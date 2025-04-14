use std::env;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(name="Helium server API", version="v0.0.1", about="Helium server API. For embedded usage open trough Helim GUI.", long_about = None)]
struct Args {
    #[arg(short, long = "db")]
    database_path: Option<String>,
    #[arg(short, long = "port")]
    port: Option<u16>,
}

pub struct HeliumSettings {
    pub database_path: String,
    pub port: u16,
}

impl HeliumSettings {
    pub fn new() -> Self {
        let args = Args::parse();
        let database_path = args.database_path.unwrap_or_else(|| {
            env::var("DATABASE_PATH").unwrap_or_else(|_| {
                panic!("Please set the DATABASE_PATH environment variable or use --database_path argument")
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
            database_path,
            port,
        }
    }
}
