use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies};
use anyhow::Result;
use crate::{
    database::DatabasePool,
    services::{auth::AuthService, session::SessionStore},
};

#[derive(Deserialize)]
struct RegisterRequest {
    username: String,
    password: String,
}

#[derive(Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct UserResponse {
    id: String,
    username: String,
    created_at: String,
}

#[derive(Serialize)]
struct LoginResponse {
    user_id: String,
    username: String,
}

pub fn auth_routes(pool: DatabasePool, session_store: SessionStore) -> Router {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/logout", post(logout))
        .route("/me", get(me))
        .with_state((pool, session_store))
}

async fn register(
    State((pool, _)): State<(DatabasePool, SessionStore)>,
    Json(payload): Json<RegisterRequest>,
) -> Result<(StatusCode, Json<Value>), (StatusCode, Json<Value>)> {
    // Validation
    if payload.username.len() < 3 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Username must be at least 3 characters"})),
        ));
    }
    
    if payload.password.len() < 8 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Password must be at least 8 characters"})),
        ));
    }

    let auth_service = AuthService::new(pool);

    match auth_service.register(&payload.username, &payload.password).await {
        Ok(user) => {
            let response = UserResponse {
                id: user.id,
                username: user.username,
                created_at: user.created_at.to_rfc3339(),
            };
            Ok((StatusCode::CREATED, Json(json!(response))))
        }
        Err(err) => {
            if err.to_string().contains("UNIQUE constraint failed") {
                Err((
                    StatusCode::CONFLICT,
                    Json(json!({"error": "Username already exists"})),
                ))
            } else {
                Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": "Registration failed"})),
                ))
            }
        }
    }
}

async fn login(
    State((pool, session_store)): State<(DatabasePool, SessionStore)>,
    cookies: Cookies,
    Json(payload): Json<LoginRequest>,
) -> Result<(StatusCode, Json<Value>), (StatusCode, Json<Value>)> {
    let auth_service = AuthService::new(pool);

    match auth_service.login(&payload.username, &payload.password).await {
        Ok(user) => {
            // Create session in store
            let session_id = session_store.create_session(user.id.clone());
            let mut cookie = Cookie::new("session_id", session_id);
            cookie.set_http_only(true);
            cookie.set_path("/"); // Allow cookie for all paths
            cookies.add(cookie);

            let response = LoginResponse {
                user_id: user.id,
                username: user.username,
            };
            Ok((StatusCode::OK, Json(json!(response))))
        }
        Err(err) => {
            if err.to_string().contains("Invalid username or password") {
                Err((
                    StatusCode::UNAUTHORIZED,
                    Json(json!({"error": "Invalid credentials"})),
                ))
            } else {
                Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": "Login failed"})),
                ))
            }
        }
    }
}

async fn logout(
    State((_, session_store)): State<(DatabasePool, SessionStore)>,
    cookies: Cookies,
) -> Result<(StatusCode, Json<Value>), (StatusCode, Json<Value>)> {
    // Check if session_id cookie exists
    if let Some(session_cookie) = cookies.get("session_id") {
        // Remove session from store
        session_store.remove_session(session_cookie.value());
        // Remove the session cookie
        cookies.remove(Cookie::named("session_id"));
        Ok((StatusCode::OK, Json(json!({"message": "Logged out"}))))
    } else {
        Err((
            StatusCode::UNAUTHORIZED,
            Json(json!({"error": "No active session"})),
        ))
    }
}

async fn me(
    State((pool, session_store)): State<(DatabasePool, SessionStore)>,
    cookies: Cookies,
) -> Result<(StatusCode, Json<Value>), (StatusCode, Json<Value>)> {
    // Get session from cookie
    let session_cookie = cookies.get("session_id")
        .ok_or_else(|| (StatusCode::UNAUTHORIZED, Json(json!({"error": "No session"}))))?;

    // Get user ID from session
    let user_id = session_store.get_user_id(session_cookie.value())
        .ok_or_else(|| (StatusCode::UNAUTHORIZED, Json(json!({"error": "Invalid session"}))))?;

    // Get user info
    let auth_service = AuthService::new(pool);
    match auth_service.validate_user(&user_id).await {
        Ok(Some(user)) => Ok((StatusCode::OK, Json(json!(user)))),
        Ok(None) => Err((StatusCode::UNAUTHORIZED, Json(json!({"error": "User not found"})))),
        Err(_) => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "Internal error"})))),
    }
}