use memory_memo::models::User;
use memory_memo::database::create_test_database;
use anyhow::Result;

#[tokio::test]
async fn test_user_creation() -> Result<()> {
    let pool = create_test_database().await?;
    
    // Test user creation with valid data
    let user = User::create(&pool, "testuser", "testpassword123").await?;
    assert_eq!(user.username, "testuser");
    assert!(!user.id.is_empty());
    assert!(!user.password_hash.is_empty());
    assert_ne!(user.password_hash, "testpassword123"); // Password should be hashed
    
    Ok(())
}

#[tokio::test]
async fn test_user_duplicate_username() -> Result<()> {
    let pool = create_test_database().await?;
    
    // Create first user
    User::create(&pool, "testuser", "password1").await?;
    
    // Try to create second user with same username - should fail
    let result = User::create(&pool, "testuser", "password2").await;
    assert!(result.is_err());
    
    Ok(())
}

#[tokio::test]
async fn test_user_password_verification() -> Result<()> {
    let pool = create_test_database().await?;
    
    let user = User::create(&pool, "testuser", "correctpassword").await?;
    
    // Test correct password verification
    assert!(user.verify_password("correctpassword"));
    
    // Test incorrect password verification
    assert!(!user.verify_password("wrongpassword"));
    
    Ok(())
}

#[tokio::test]
async fn test_user_find_by_username() -> Result<()> {
    let pool = create_test_database().await?;
    
    let original_user = User::create(&pool, "findme", "password123").await?;
    
    // Test finding existing user
    let found_user = User::find_by_username(&pool, "findme").await?;
    assert!(found_user.is_some());
    let found_user = found_user.unwrap();
    assert_eq!(found_user.id, original_user.id);
    assert_eq!(found_user.username, "findme");
    
    // Test finding non-existent user
    let not_found = User::find_by_username(&pool, "nonexistent").await?;
    assert!(not_found.is_none());
    
    Ok(())
}

#[tokio::test]
async fn test_user_username_validation() -> Result<()> {
    let pool = create_test_database().await?;
    
    // Test username too short (less than 3 chars)
    let result = User::create(&pool, "ab", "password123").await;
    assert!(result.is_err());
    
    // Test username too long (more than 50 chars)
    let long_username = "a".repeat(51);
    let result = User::create(&pool, &long_username, "password123").await;
    assert!(result.is_err());
    
    // Test valid username (3-50 chars)
    let result = User::create(&pool, "validuser", "password123").await;
    assert!(result.is_ok());
    
    Ok(())
}