use memory_memo::{create_app, database::init_database};
use tracing_subscriber;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Get database URL from environment or use default
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite://./memory_memo.db".to_string());

    // Create database connection pool and run migrations
    let pool = init_database(&database_url).await?;

    // Create app
    let app = create_app(pool).await?;

    // Get port from environment or use default
    let port = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string());
    
    let bind_addr = format!("127.0.0.1:{}", port);

    // Start server
    let listener = tokio::net::TcpListener::bind(&bind_addr).await?;
    println!("🚀 Server running on http://{}", bind_addr);
    println!("📁 Database: {}", database_url);

    axum::serve(listener, app).await?;

    Ok(())
}