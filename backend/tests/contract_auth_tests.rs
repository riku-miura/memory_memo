use axum_test::TestServer;
use axum::http::StatusCode;
use memory_memo::create_app;
use memory_memo::database::create_test_database;
use serde_json::{json, Value};
use anyhow::Result;

async fn create_test_server() -> Result<TestServer> {
    let pool = create_test_database().await?;
    let app = create_app(pool).await?;
    Ok(TestServer::new(app)?)
}

#[tokio::test]
async fn test_auth_register_contract_success() -> Result<()> {
    let server = create_test_server().await?;
    
    let response = server
        .post("/api/auth/register")
        .json(&json!({
            "username": "testuser",
            "password": "password123"
        }))
        .await;
    
    // Contract: Should return 201 Created
    response.assert_status(StatusCode::CREATED);
    
    // Contract: Should return JSON with expected fields
    let body: Value = response.json();
    assert!(body.get("id").is_some());
    assert_eq!(body["username"], "testuser");
    assert!(body.get("created_at").is_some());
    
    // Contract: Should not return password hash
    assert!(body.get("password_hash").is_none());
    
    Ok(())
}

#[tokio::test]
async fn test_auth_register_contract_duplicate_username() -> Result<()> {
    let server = create_test_server().await?;
    
    // Register first user
    server
        .post("/api/auth/register")
        .json(&json!({
            "username": "testuser",
            "password": "password1"
        }))
        .await;
    
    // Try to register duplicate username
    let response = server
        .post("/api/auth/register")
        .json(&json!({
            "username": "testuser",
            "password": "password2"
        }))
        .await;
    
    // Contract: Should return 409 Conflict
    response.assert_status(StatusCode::CONFLICT);
    
    // Contract: Should return error message
    let body: Value = response.json();
    assert!(body.get("error").is_some());
    
    Ok(())
}

#[tokio::test]
async fn test_auth_register_contract_validation() -> Result<()> {
    let server = create_test_server().await?;
    
    // Test missing username
    let response = server
        .post("/api/auth/register")
        .json(&json!({
            "password": "password123"
        }))
        .await;
    response.assert_status(StatusCode::UNPROCESSABLE_ENTITY);
    
    // Test missing password
    let response = server
        .post("/api/auth/register")
        .json(&json!({
            "username": "testuser"
        }))
        .await;
    response.assert_status(StatusCode::UNPROCESSABLE_ENTITY);
    
    // Test short username (less than 3 chars)
    let response = server
        .post("/api/auth/register")
        .json(&json!({
            "username": "ab",
            "password": "password123"
        }))
        .await;
    response.assert_status(StatusCode::BAD_REQUEST);
    
    // Test short password (less than 8 chars)
    let response = server
        .post("/api/auth/register")
        .json(&json!({
            "username": "testuser",
            "password": "1234567"
        }))
        .await;
    response.assert_status(StatusCode::BAD_REQUEST);
    
    Ok(())
}

#[tokio::test]
async fn test_auth_login_contract_success() -> Result<()> {
    let server = create_test_server().await?;
    
    // Register user first
    server
        .post("/api/auth/register")
        .json(&json!({
            "username": "testuser",
            "password": "password123"
        }))
        .await;
    
    // Test successful login
    let response = server
        .post("/api/auth/login")
        .json(&json!({
            "username": "testuser",
            "password": "password123"
        }))
        .await;
    
    // Contract: Should return 200 OK
    response.assert_status(StatusCode::OK);
    
    // Contract: Should return user info
    let body: Value = response.json();
    assert!(body.get("user_id").is_some());
    assert_eq!(body["username"], "testuser");
    
    // Contract: Should set session cookie
    let cookies = response.cookies();
    assert!(cookies.iter().any(|c| c.name() == "session_id"));
    
    Ok(())
}

#[tokio::test]
async fn test_auth_login_contract_invalid_credentials() -> Result<()> {
    let server = create_test_server().await?;
    
    // Register user
    server
        .post("/api/auth/register")
        .json(&json!({
            "username": "testuser",
            "password": "correct_password"
        }))
        .await;
    
    // Test login with wrong password
    let response = server
        .post("/api/auth/login")
        .json(&json!({
            "username": "testuser",
            "password": "wrong_password"
        }))
        .await;
    
    // Contract: Should return 401 Unauthorized
    response.assert_status(StatusCode::UNAUTHORIZED);
    
    // Test login with non-existent username
    let response = server
        .post("/api/auth/login")
        .json(&json!({
            "username": "nonexistent",
            "password": "password123"
        }))
        .await;
    
    // Contract: Should return 401 Unauthorized
    response.assert_status(StatusCode::UNAUTHORIZED);
    
    Ok(())
}

#[tokio::test]
async fn test_auth_logout_contract_success() -> Result<()> {
    let server = create_test_server().await?;
    
    // Register and login to get session
    server
        .post("/api/auth/register")
        .json(&json!({
            "username": "testuser",
            "password": "password123"
        }))
        .await;
    
    let login_response = server
        .post("/api/auth/login")
        .json(&json!({
            "username": "testuser",
            "password": "password123"
        }))
        .await;
    
    let session_cookie = login_response
        .cookies()
        .iter()
        .find(|c| c.name() == "session_id")
        .unwrap()
        .clone();
    
    // Test logout
    let response = server
        .post("/api/auth/logout")
        .add_cookie(session_cookie)
        .await;
    
    // Contract: Should return 200 OK
    response.assert_status(StatusCode::OK);
    
    Ok(())
}

#[tokio::test]
async fn test_auth_logout_contract_unauthorized() -> Result<()> {
    let server = create_test_server().await?;
    
    // Test logout without session
    let response = server
        .post("/api/auth/logout")
        .await;
    
    // Contract: Should return 401 Unauthorized
    response.assert_status(StatusCode::UNAUTHORIZED);
    
    Ok(())
}

#[tokio::test]
async fn test_auth_endpoints_content_type() -> Result<()> {
    let server = create_test_server().await?;
    
    // Test that all auth endpoints return JSON
    let endpoints = [
        ("/api/auth/register", json!({"username": "test", "password": "12345678"})),
        ("/api/auth/login", json!({"username": "test", "password": "12345678"})),
    ];
    
    for (endpoint, payload) in endpoints {
        let response = server
            .post(endpoint)
            .json(&payload)
            .await;
        
        // Should have JSON content type
        assert!(response.headers().get("content-type").unwrap().to_str().unwrap().contains("application/json"));
    }
    
    Ok(())
}