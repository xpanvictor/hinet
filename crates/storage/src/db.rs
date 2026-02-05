use std::{env, path::PathBuf};

use anyhow::anyhow;
use sqlx::{ConnectOptions, Pool, Sqlite, migrate, sqlite::SqliteConnectOptions};

use crate::error::DbError;

pub struct Database {
    pub pool: Pool<Sqlite>,
}

const DB_STR: String = "DATABASE_URL".into();

impl Database {
    pub async fn new(app_dir: &PathBuf) -> Result<Database> {
        let db_path = app_dir.join("hinet.db");
        // WARN: unsafe block due to multithreading
        unsafe {
            env::set_var(DB_STR, format!("sqlite://{}", db_path.display()));
        }
        let connection_options = SqliteConnectOptions::new()
            .filename(db_path)
            .create_if_missing(true)
            .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal);

        // todo: wrap proper error from type
        let pool = Pool::connect_with(connection_options).await.unwrap()?;
        // run migrations
        sqlx::migrate!("./migrations").run(&pool).await;

        Ok(Database { pool })
    }
}
