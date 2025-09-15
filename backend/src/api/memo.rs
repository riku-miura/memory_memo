use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{get, post, put, delete},
    Router,
};
use tower_cookies::Cookies;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use anyhow::Result;
use crate::{
    database::DatabasePool,
    services::{memo::MemoService, session::SessionStore},
    models::User,
};

#[derive(Deserialize)]
struct CreateMemoRequest {
    content: String,
}

#[derive(Deserialize)]
struct UpdateMemoRequest {
    content: String,
}

#[derive(Serialize)]
struct MemoResponse {
    id: String,
    content: String,
    memo_type: String,
    created_at: String,
    updated_at: Option<String>,
    expires_at: Option<String>,
}

pub fn memo_routes(pool: DatabasePool, session_store: SessionStore) -> Router {
    Router::new()
        .route("/", get(list_memos))
        .route("/forever", post(create_forever_memo))
        .route("/flush", post(create_flush_memo))
        .route("/forever/:memo_id", put(update_forever_memo))
        .route("/forever/:memo_id", delete(delete_forever_memo))
        .route("/flush/:memo_id", delete(delete_flush_memo))
        .with_state((pool, session_store))
}

// Helper function to get user from session
async fn get_user_from_session(
    session_store: &SessionStore,
    cookies: &Cookies,
) -> Result<String, (StatusCode, Json<Value>)> {
    let session_cookie = cookies.get("session_id")
        .ok_or_else(|| (StatusCode::UNAUTHORIZED, Json(json!({"error": "No session"}))))?;

    // Get user ID from session store
    let user_id = session_store.get_user_id(session_cookie.value())
        .ok_or_else(|| (StatusCode::UNAUTHORIZED, Json(json!({"error": "Invalid session"}))))?;

    Ok(user_id)
}

async fn create_forever_memo(
    State((pool, session_store)): State<(DatabasePool, SessionStore)>,
    cookies: Cookies,
    Json(payload): Json<CreateMemoRequest>,
) -> Result<(StatusCode, Json<Value>), (StatusCode, Json<Value>)> {
    // Validation
    if payload.content.trim().is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Content cannot be empty"})),
        ));
    }

    let user_id = get_user_from_session(&session_store, &cookies).await?;
    let memo_service = MemoService::new(pool);

    match memo_service.create_forever_memo(&user_id, &payload.content).await {
        Ok(memo) => {
            let response = MemoResponse {
                id: memo.id,
                content: memo.content,
                memo_type: "forever".to_string(),
                created_at: memo.created_at.to_rfc3339(),
                updated_at: None, // ForeverMemo doesn't track updates yet
                expires_at: None,
            };
            Ok((StatusCode::CREATED, Json(json!(response))))
        }
        Err(_) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to create memo"})),
        )),
    }
}

async fn create_flush_memo(
    State((pool, session_store)): State<(DatabasePool, SessionStore)>,
    cookies: Cookies,
    Json(payload): Json<CreateMemoRequest>,
) -> Result<(StatusCode, Json<Value>), (StatusCode, Json<Value>)> {
    // Validation
    if payload.content.trim().is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Content cannot be empty"})),
        ));
    }

    let user_id = get_user_from_session(&session_store, &cookies).await?;
    let memo_service = MemoService::new(pool);

    match memo_service.create_flush_memo(&user_id, &payload.content).await {
        Ok(memo) => {
            let response = MemoResponse {
                id: memo.id,
                content: memo.content,
                memo_type: "flush".to_string(),
                created_at: memo.created_at.to_rfc3339(),
                updated_at: None,
                expires_at: Some(memo.expires_at.to_rfc3339()),
            };
            Ok((StatusCode::CREATED, Json(json!(response))))
        }
        Err(_) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to create memo"})),
        )),
    }
}

async fn list_memos(
    State((pool, session_store)): State<(DatabasePool, SessionStore)>,
    cookies: Cookies,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let user_id = get_user_from_session(&session_store, &cookies).await?;
    let memo_service = MemoService::new(pool);

    match memo_service.list_user_memos(&user_id).await {
        Ok(user_memos) => {
            let forever_memos: Vec<MemoResponse> = user_memos.forever_memos
                .into_iter()
                .map(|memo| MemoResponse {
                    id: memo.id,
                    content: memo.content,
                    memo_type: "forever".to_string(),
                    created_at: memo.created_at.to_rfc3339(),
                    updated_at: None, // ForeverMemo doesn't track updates yet
                    expires_at: None,
                })
                .collect();

            let flush_memos: Vec<MemoResponse> = user_memos.flush_memos
                .into_iter()
                .map(|memo| MemoResponse {
                    id: memo.id,
                    content: memo.content,
                    memo_type: "flush".to_string(),
                    created_at: memo.created_at.to_rfc3339(),
                    updated_at: None,
                    expires_at: Some(memo.expires_at.to_rfc3339()),
                })
                .collect();

            Ok(Json(json!({
                "forever_memos": forever_memos,
                "flush_memos": flush_memos
            })))
        }
        Err(_) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to fetch memos"})),
        )),
    }
}

async fn update_forever_memo(
    State((pool, session_store)): State<(DatabasePool, SessionStore)>,
    Path(memo_id): Path<String>,
    cookies: Cookies,
    Json(payload): Json<UpdateMemoRequest>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    // Validation
    if payload.content.trim().is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Content cannot be empty"})),
        ));
    }

    let user_id = get_user_from_session(&session_store, &cookies).await?;
    let memo_service = MemoService::new(pool);

    match memo_service.update_forever_memo(&memo_id, &user_id, &payload.content).await {
        Ok(Some(memo)) => {
            let response = MemoResponse {
                id: memo.id,
                content: memo.content,
                memo_type: "forever".to_string(),
                created_at: memo.created_at.to_rfc3339(),
                updated_at: None, // ForeverMemo doesn't track updates yet
                expires_at: None,
            };
            Ok(Json(json!(response)))
        }
        Ok(None) => Err((
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Memo not found"})),
        )),
        Err(_) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to update memo"})),
        )),
    }
}

async fn delete_forever_memo(
    State((pool, session_store)): State<(DatabasePool, SessionStore)>,
    Path(memo_id): Path<String>,
    cookies: Cookies,
) -> Result<StatusCode, (StatusCode, Json<Value>)> {
    let user_id = get_user_from_session(&session_store, &cookies).await?;
    let memo_service = MemoService::new(pool);

    match memo_service.delete_forever_memo(&memo_id, &user_id).await {
        Ok(true) => Ok(StatusCode::NO_CONTENT),
        Ok(false) => Err((
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Memo not found"})),
        )),
        Err(_) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to delete memo"})),
        )),
    }
}

async fn delete_flush_memo(
    State((pool, session_store)): State<(DatabasePool, SessionStore)>,
    Path(memo_id): Path<String>,
    cookies: Cookies,
) -> Result<StatusCode, (StatusCode, Json<Value>)> {
    let user_id = get_user_from_session(&session_store, &cookies).await?;
    let memo_service = MemoService::new(pool);

    match memo_service.delete_flush_memo(&memo_id, &user_id).await {
        Ok(true) => Ok(StatusCode::NO_CONTENT),
        Ok(false) => Err((
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Memo not found"})),
        )),
        Err(_) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to delete memo"})),
        )),
    }
}