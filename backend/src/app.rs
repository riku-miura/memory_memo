use axum::{routing::get, Router};
use tower_http::{trace::TraceLayer, cors::CorsLayer, services::ServeDir};
use tower_cookies::CookieManagerLayer;
use anyhow::Result;
use crate::{
    api::{auth_routes, memo_routes},
    database::DatabasePool,
    services::SessionStore,
};

pub async fn create_app(pool: DatabasePool) -> Result<Router> {
    let session_store = SessionStore::new();
    // Create CORS layer
    let cors = CorsLayer::new()
        .allow_origin("http://127.0.0.1:8080".parse::<axum::http::HeaderValue>()?)
        .allow_origin("http://localhost:8080".parse::<axum::http::HeaderValue>()?)
        .allow_origin("http://127.0.0.1:3000".parse::<axum::http::HeaderValue>()?)
        .allow_origin("http://localhost:3000".parse::<axum::http::HeaderValue>()?)
        .allow_methods([
            axum::http::Method::GET,
            axum::http::Method::POST,
            axum::http::Method::PUT,
            axum::http::Method::DELETE,
            axum::http::Method::OPTIONS,
        ])
        .allow_headers([
            axum::http::header::CONTENT_TYPE,
            axum::http::header::AUTHORIZATION,
            axum::http::header::ACCEPT,
        ])
        .allow_credentials(true);

    let app = Router::new()
        .route("/health", get(health_check))
        .nest("/api/auth", auth_routes(pool.clone(), session_store.clone()))
        .nest("/api/memos", memo_routes(pool.clone(), session_store.clone()))
        // Serve static files from frontend directory
        .nest_service("/", ServeDir::new("../frontend"))
        .layer(cors)
        .layer(CookieManagerLayer::new())
        .layer(TraceLayer::new_for_http());

    Ok(app)
}

async fn health_check() -> &'static str {
    "OK"
}