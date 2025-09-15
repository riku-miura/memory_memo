use memory_memo::services::AuthService;
use memory_memo::models::User;
use memory_memo::database::create_test_database;
use anyhow::Result;

#[tokio::test]
async fn test_auth_register_user() -> Result<()> {
    let pool = create_test_database().await?;
    let auth_service = AuthService::new(pool.clone());
    
    // Test successful user registration
    let user = auth_service.register("testuser", "password123").await?;
    assert_eq!(user.username, "testuser");
    assert!(!user.id.is_empty());
    
    Ok(())
}

#[tokio::test]
async fn test_auth_register_duplicate_username() -> Result<()> {
    let pool = create_test_database().await?;
    let auth_service = AuthService::new(pool.clone());
    
    // Register first user
    auth_service.register("testuser", "password1").await?;
    
    // Try to register second user with same username - should fail
    let result = auth_service.register("testuser", "password2").await;
    assert!(result.is_err());
    
    Ok(())
}

#[tokio::test]
async fn test_auth_login_valid_credentials() -> Result<()> {
    let pool = create_test_database().await?;
    let auth_service = AuthService::new(pool.clone());
    
    // Register user first
    auth_service.register("testuser", "password123").await?;
    
    // Test successful login
    let user = auth_service.login("testuser", "password123").await?;
    assert_eq!(user.username, "testuser");
    
    Ok(())
}

#[tokio::test]
async fn test_auth_login_invalid_username() -> Result<()> {
    let pool = create_test_database().await?;
    let auth_service = AuthService::new(pool.clone());
    
    // Try to login with non-existent username
    let result = auth_service.login("nonexistent", "password123").await;
    assert!(result.is_err());
    
    Ok(())
}

#[tokio::test]
async fn test_auth_login_invalid_password() -> Result<()> {
    let pool = create_test_database().await?;
    let auth_service = AuthService::new(pool.clone());
    
    // Register user first
    auth_service.register("testuser", "correct_password").await?;
    
    // Try to login with wrong password
    let result = auth_service.login("testuser", "wrong_password").await;
    assert!(result.is_err());
    
    Ok(())
}

#[tokio::test]
async fn test_auth_validate_user() -> Result<()> {
    let pool = create_test_database().await?;
    let auth_service = AuthService::new(pool.clone());
    
    // Register user
    let original_user = auth_service.register("testuser", "password123").await?;
    
    // Test validating existing user
    let validated_user = auth_service.validate_user(&original_user.id).await?;
    assert!(validated_user.is_some());
    assert_eq!(validated_user.unwrap().username, "testuser");
    
    // Test validating non-existent user
    let non_existent = auth_service.validate_user("non-existent-id").await?;
    assert!(non_existent.is_none());
    
    Ok(())
}

#[tokio::test]
async fn test_auth_username_validation() -> Result<()> {
    let pool = create_test_database().await?;
    let auth_service = AuthService::new(pool.clone());
    
    // Test username too short
    let result = auth_service.register("ab", "password123").await;
    assert!(result.is_err());
    
    // Test username too long
    let long_username = "a".repeat(51);
    let result = auth_service.register(&long_username, "password123").await;
    assert!(result.is_err());
    
    // Test valid username
    let result = auth_service.register("validuser", "password123").await;
    assert!(result.is_ok());
    
    Ok(())
}

#[tokio::test]
async fn test_auth_password_validation() -> Result<()> {
    let pool = create_test_database().await?;
    let auth_service = AuthService::new(pool.clone());
    
    // Test password too short
    let result = auth_service.register("testuser", "1234567").await; // 7 chars
    assert!(result.is_err());
    
    // Test valid password
    let result = auth_service.register("testuser", "12345678").await; // 8 chars
    assert!(result.is_ok());
    
    Ok(())
}