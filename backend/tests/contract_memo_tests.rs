use axum_test::TestServer;
use axum::http::StatusCode;
use memory_memo::create_app;
use memory_memo::database::create_test_database;
use serde_json::{json, Value};
use tower_cookies::cookie::Cookie;
use anyhow::Result;

async fn create_test_server() -> Result<TestServer> {
    let pool = create_test_database().await?;
    let app = create_app(pool).await?;
    Ok(TestServer::new(app)?)
}

async fn create_authenticated_user(server: &TestServer) -> Result<Cookie<'static>> {
    // Register user
    server
        .post("/api/auth/register")
        .json(&json!({
            "username": "testuser",
            "password": "password123"
        }))
        .await;

    // Login to get session
    let login_response = server
        .post("/api/auth/login")
        .json(&json!({
            "username": "testuser",
            "password": "password123"
        }))
        .await;

    let cookies = login_response.cookies();
    let session_cookie = cookies
        .iter()
        .find(|c| c.name() == "session_id")
        .unwrap();

    // Convert to cookie::Cookie with owned value
    let cookie_value = session_cookie.value().to_owned();
    let cookie = Cookie::new("session_id", cookie_value);
    Ok(cookie)
}

#[tokio::test]
async fn test_memo_create_forever_memo_success() -> Result<()> {
    let server = create_test_server().await?;
    let session = create_authenticated_user(&server).await?;

    let response = server
        .post("/api/memos/forever")
        .add_cookie(session.clone())
        .json(&json!({
            "content": "This is a forever memo"
        }))
        .await;

    // Contract: Should return 201 Created
    response.assert_status(StatusCode::CREATED);

    // Contract: Should return JSON with expected fields
    let body: Value = response.json();
    assert!(body.get("id").is_some());
    assert_eq!(body["content"], "This is a forever memo");
    assert_eq!(body["memo_type"], "forever");
    assert!(body.get("created_at").is_some());
    assert!(body.get("updated_at").is_some());

    Ok(())
}

#[tokio::test]
async fn test_memo_create_flush_memo_success() -> Result<()> {
    let server = create_test_server().await?;
    let session = create_authenticated_user(&server).await?;

    let response = server
        .post("/api/memos/flush")
        .add_cookie(session.clone())
        .json(&json!({
            "content": "This is a flush memo"
        }))
        .await;

    // Contract: Should return 201 Created
    response.assert_status(StatusCode::CREATED);

    // Contract: Should return JSON with expected fields
    let body: Value = response.json();
    assert!(body.get("id").is_some());
    assert_eq!(body["content"], "This is a flush memo");
    assert_eq!(body["memo_type"], "flush");
    assert!(body.get("created_at").is_some());
    assert!(body.get("expires_at").is_some());

    Ok(())
}

#[tokio::test]
async fn test_memo_create_unauthorized() -> Result<()> {
    let server = create_test_server().await?;

    // Test creating forever memo without authentication
    let response = server
        .post("/api/memos/forever")
        .json(&json!({
            "content": "This should fail"
        }))
        .await;

    // Contract: Should return 401 Unauthorized
    response.assert_status(StatusCode::UNAUTHORIZED);

    // Test creating flush memo without authentication
    let response = server
        .post("/api/memos/flush")
        .json(&json!({
            "content": "This should fail"
        }))
        .await;

    // Contract: Should return 401 Unauthorized
    response.assert_status(StatusCode::UNAUTHORIZED);

    Ok(())
}

#[tokio::test]
async fn test_memo_create_validation() -> Result<()> {
    let server = create_test_server().await?;
    let session = create_authenticated_user(&server).await?;

    // Test empty content
    let response = server
        .post("/api/memos/forever")
        .add_cookie(session.clone())
        .json(&json!({
            "content": ""
        }))
        .await;

    response.assert_status(StatusCode::BAD_REQUEST);

    // Test missing content field
    let response = server
        .post("/api/memos/forever")
        .add_cookie(session.clone())
        .json(&json!({}))
        .await;

    response.assert_status(StatusCode::UNPROCESSABLE_ENTITY);

    Ok(())
}

#[tokio::test]
async fn test_memo_list_success() -> Result<()> {
    let server = create_test_server().await?;
    let session = create_authenticated_user(&server).await?;

    // Create some memos first
    server
        .post("/api/memos/forever")
        .add_cookie(session.clone())
        .json(&json!({
            "content": "Forever memo 1"
        }))
        .await;

    server
        .post("/api/memos/flush")
        .add_cookie(session.clone())
        .json(&json!({
            "content": "Flush memo 1"
        }))
        .await;

    // Test listing memos
    let response = server
        .get("/api/memos")
        .add_cookie(session.clone())
        .await;

    // Contract: Should return 200 OK
    response.assert_status(StatusCode::OK);

    // Contract: Should return JSON array
    let body: Value = response.json();
    assert!(body.is_object());
    assert!(body.get("forever_memos").is_some());
    assert!(body.get("flush_memos").is_some());

    let forever_memos = body["forever_memos"].as_array().unwrap();
    let flush_memos = body["flush_memos"].as_array().unwrap();

    assert_eq!(forever_memos.len(), 1);
    assert_eq!(flush_memos.len(), 1);
    assert_eq!(forever_memos[0]["content"], "Forever memo 1");
    assert_eq!(flush_memos[0]["content"], "Flush memo 1");

    Ok(())
}

#[tokio::test]
async fn test_memo_list_unauthorized() -> Result<()> {
    let server = create_test_server().await?;

    let response = server
        .get("/api/memos")
        .await;

    // Contract: Should return 401 Unauthorized
    response.assert_status(StatusCode::UNAUTHORIZED);

    Ok(())
}

#[tokio::test]
async fn test_memo_update_forever_memo_success() -> Result<()> {
    let server = create_test_server().await?;
    let session = create_authenticated_user(&server).await?;

    // Create a memo first
    let create_response = server
        .post("/api/memos/forever")
        .add_cookie(session.clone())
        .json(&json!({
            "content": "Original content"
        }))
        .await;

    let memo: Value = create_response.json();
    let memo_id = memo["id"].as_str().unwrap();

    // Update the memo
    let response = server
        .put(&format!("/api/memos/forever/{}", memo_id))
        .add_cookie(session.clone())
        .json(&json!({
            "content": "Updated content"
        }))
        .await;

    // Contract: Should return 200 OK
    response.assert_status(StatusCode::OK);

    // Contract: Should return updated memo
    let body: Value = response.json();
    assert_eq!(body["content"], "Updated content");
    assert_eq!(body["id"], memo_id);

    Ok(())
}

#[tokio::test]
async fn test_memo_update_unauthorized() -> Result<()> {
    let server = create_test_server().await?;
    let session = create_authenticated_user(&server).await?;

    // Create a memo first
    let create_response = server
        .post("/api/memos/forever")
        .add_cookie(session.clone())
        .json(&json!({
            "content": "Original content"
        }))
        .await;

    let memo: Value = create_response.json();
    let memo_id = memo["id"].as_str().unwrap();

    // Try to update without authentication
    let response = server
        .put(&format!("/api/memos/forever/{}", memo_id))
        .json(&json!({
            "content": "Updated content"
        }))
        .await;

    // Contract: Should return 401 Unauthorized
    response.assert_status(StatusCode::UNAUTHORIZED);

    Ok(())
}

#[tokio::test]
async fn test_memo_update_not_found() -> Result<()> {
    let server = create_test_server().await?;
    let session = create_authenticated_user(&server).await?;

    let response = server
        .put("/api/memos/forever/nonexistent-id")
        .add_cookie(session.clone())
        .json(&json!({
            "content": "Updated content"
        }))
        .await;

    // Contract: Should return 404 Not Found
    response.assert_status(StatusCode::NOT_FOUND);

    Ok(())
}

#[tokio::test]
async fn test_memo_delete_success() -> Result<()> {
    let server = create_test_server().await?;
    let session = create_authenticated_user(&server).await?;

    // Create a memo first
    let create_response = server
        .post("/api/memos/forever")
        .add_cookie(session.clone())
        .json(&json!({
            "content": "To be deleted"
        }))
        .await;

    let memo: Value = create_response.json();
    let memo_id = memo["id"].as_str().unwrap();

    // Delete the memo
    let response = server
        .delete(&format!("/api/memos/forever/{}", memo_id))
        .add_cookie(session.clone())
        .await;

    // Contract: Should return 204 No Content
    response.assert_status(StatusCode::NO_CONTENT);

    Ok(())
}

#[tokio::test]
async fn test_memo_delete_unauthorized() -> Result<()> {
    let server = create_test_server().await?;
    let session = create_authenticated_user(&server).await?;

    // Create a memo first
    let create_response = server
        .post("/api/memos/forever")
        .add_cookie(session.clone())
        .json(&json!({
            "content": "To be deleted"
        }))
        .await;

    let memo: Value = create_response.json();
    let memo_id = memo["id"].as_str().unwrap();

    // Try to delete without authentication
    let response = server
        .delete(&format!("/api/memos/forever/{}", memo_id))
        .await;

    // Contract: Should return 401 Unauthorized
    response.assert_status(StatusCode::UNAUTHORIZED);

    Ok(())
}

#[tokio::test]
async fn test_memo_endpoints_content_type() -> Result<()> {
    let server = create_test_server().await?;
    let session = create_authenticated_user(&server).await?;

    // Test that all memo endpoints return JSON
    let endpoints_and_payloads = [
        ("/api/memos/forever", json!({"content": "test"})),
        ("/api/memos/flush", json!({"content": "test"})),
    ];

    for (endpoint, payload) in endpoints_and_payloads {
        let response = server
            .post(endpoint)
            .add_cookie(session.clone())
            .json(&payload)
            .await;

        // Should have JSON content type
        assert!(response.headers().get("content-type").unwrap().to_str().unwrap().contains("application/json"));
    }

    // Test GET endpoint
    let response = server
        .get("/api/memos")
        .add_cookie(session.clone())
        .await;

    assert!(response.headers().get("content-type").unwrap().to_str().unwrap().contains("application/json"));

    Ok(())
}