use sqlx::{Pool, Sqlite, SqlitePool};
use std::path::Path;
use anyhow::Result;

pub type DatabasePool = Pool<Sqlite>;

/// Initialize the database connection pool and run migrations
pub async fn init_database(database_url: &str) -> Result<DatabasePool> {
    // Create database file if it doesn't exist
    if let Some(parent) = Path::new(database_url.strip_prefix("sqlite://").unwrap_or(database_url)).parent() {
        tokio::fs::create_dir_all(parent).await?;
    }

    // Connect to database
    let pool = SqlitePool::connect(database_url).await?;
    
    // Run migrations
    sqlx::migrate!("./migrations").run(&pool).await?;
    
    tracing::info!("Database initialized successfully");
    Ok(pool)
}

/// Create an in-memory database for testing
pub async fn create_test_database() -> Result<DatabasePool> {
    let pool = SqlitePool::connect("sqlite::memory:").await?;
    sqlx::migrate!("./migrations").run(&pool).await?;
    Ok(pool)
}