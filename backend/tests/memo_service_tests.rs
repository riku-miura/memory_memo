use memory_memo::services::MemoService;
use memory_memo::models::{User, ForeverMemo, FlushMemo};
use memory_memo::database::create_test_database;
use anyhow::Result;

#[tokio::test]
async fn test_memo_create_forever_memo() -> Result<()> {
    let pool = create_test_database().await?;
    let memo_service = MemoService::new(pool.clone());
    
    // Create a user first
    let user = User::create(&pool, "testuser", "password123").await?;
    
    // Test creating forever memo
    let memo = memo_service.create_forever_memo(&user.id, "Test forever memo").await?;
    assert_eq!(memo.content, "Test forever memo");
    assert_eq!(memo.user_id, user.id);
    
    Ok(())
}

#[tokio::test]
async fn test_memo_create_flush_memo() -> Result<()> {
    let pool = create_test_database().await?;
    let memo_service = MemoService::new(pool.clone());
    
    // Create a user first
    let user = User::create(&pool, "testuser", "password123").await?;
    
    // Test creating flush memo
    let memo = memo_service.create_flush_memo(&user.id, "Test flush memo").await?;
    assert_eq!(memo.content, "Test flush memo");
    assert_eq!(memo.user_id, user.id);
    assert!(memo.expires_at > memo.created_at);
    
    Ok(())
}

#[tokio::test]
async fn test_memo_list_user_memos() -> Result<()> {
    let pool = create_test_database().await?;
    let memo_service = MemoService::new(pool.clone());
    
    // Create two users
    let user1 = User::create(&pool, "user1", "password1").await?;
    let user2 = User::create(&pool, "user2", "password2").await?;
    
    // Create memos for both users
    memo_service.create_forever_memo(&user1.id, "User1 forever memo").await?;
    memo_service.create_flush_memo(&user1.id, "User1 flush memo").await?;
    memo_service.create_forever_memo(&user2.id, "User2 forever memo").await?;
    
    // Get memos for user1
    let user1_memos = memo_service.list_user_memos(&user1.id).await?;
    
    // Should have flush memos first, then forever memos
    assert_eq!(user1_memos.flush_memos.len(), 1);
    assert_eq!(user1_memos.forever_memos.len(), 1);
    assert_eq!(user1_memos.flush_memos[0].content, "User1 flush memo");
    assert_eq!(user1_memos.forever_memos[0].content, "User1 forever memo");
    
    // Get memos for user2
    let user2_memos = memo_service.list_user_memos(&user2.id).await?;
    assert_eq!(user2_memos.flush_memos.len(), 0);
    assert_eq!(user2_memos.forever_memos.len(), 1);
    
    Ok(())
}

#[tokio::test]
async fn test_memo_delete_forever_memo() -> Result<()> {
    let pool = create_test_database().await?;
    let memo_service = MemoService::new(pool.clone());
    
    let user = User::create(&pool, "testuser", "password123").await?;
    let memo = memo_service.create_forever_memo(&user.id, "Memo to delete").await?;
    
    // Delete memo
    memo_service.delete_memo(&memo.id, &user.id).await?;
    
    // Verify memo is deleted
    let user_memos = memo_service.list_user_memos(&user.id).await?;
    assert_eq!(user_memos.forever_memos.len(), 0);
    
    Ok(())
}

#[tokio::test]
async fn test_memo_delete_flush_memo() -> Result<()> {
    let pool = create_test_database().await?;
    let memo_service = MemoService::new(pool.clone());
    
    let user = User::create(&pool, "testuser", "password123").await?;
    let memo = memo_service.create_flush_memo(&user.id, "Flush memo to delete").await?;
    
    // Delete memo
    memo_service.delete_memo(&memo.id, &user.id).await?;
    
    // Verify memo is deleted
    let user_memos = memo_service.list_user_memos(&user.id).await?;
    assert_eq!(user_memos.flush_memos.len(), 0);
    
    Ok(())
}

#[tokio::test]
async fn test_memo_delete_security() -> Result<()> {
    let pool = create_test_database().await?;
    let memo_service = MemoService::new(pool.clone());
    
    // Create two users
    let user1 = User::create(&pool, "user1", "password1").await?;
    let user2 = User::create(&pool, "user2", "password2").await?;
    
    // User1 creates a memo
    let memo = memo_service.create_forever_memo(&user1.id, "User1's memo").await?;
    
    // User2 tries to delete User1's memo - should fail
    let result = memo_service.delete_memo(&memo.id, &user2.id).await;
    assert!(result.is_err());
    
    // Verify memo still exists
    let user1_memos = memo_service.list_user_memos(&user1.id).await?;
    assert_eq!(user1_memos.forever_memos.len(), 1);
    
    Ok(())
}

#[tokio::test]
async fn test_memo_empty_content_validation() -> Result<()> {
    let pool = create_test_database().await?;
    let memo_service = MemoService::new(pool.clone());
    
    let user = User::create(&pool, "testuser", "password123").await?;
    
    // Try to create memos with empty content
    let result1 = memo_service.create_forever_memo(&user.id, "").await;
    assert!(result1.is_err());
    
    let result2 = memo_service.create_flush_memo(&user.id, "").await;
    assert!(result2.is_err());
    
    // Try with whitespace only
    let result3 = memo_service.create_forever_memo(&user.id, "   ").await;
    assert!(result3.is_err());
    
    Ok(())
}

#[tokio::test]
async fn test_memo_ordering() -> Result<()> {
    let pool = create_test_database().await?;
    let memo_service = MemoService::new(pool.clone());
    
    let user = User::create(&pool, "testuser", "password123").await?;
    
    // Create memos in sequence
    let _memo1 = memo_service.create_forever_memo(&user.id, "First forever memo").await?;
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    let _memo2 = memo_service.create_flush_memo(&user.id, "First flush memo").await?;
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    let _memo3 = memo_service.create_forever_memo(&user.id, "Second forever memo").await?;
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    let _memo4 = memo_service.create_flush_memo(&user.id, "Second flush memo").await?;
    
    let user_memos = memo_service.list_user_memos(&user.id).await?;
    
    // Flush memos should be newest first
    assert_eq!(user_memos.flush_memos.len(), 2);
    assert_eq!(user_memos.flush_memos[0].content, "Second flush memo");
    assert_eq!(user_memos.flush_memos[1].content, "First flush memo");
    
    // Forever memos should be newest first
    assert_eq!(user_memos.forever_memos.len(), 2);
    assert_eq!(user_memos.forever_memos[0].content, "Second forever memo");
    assert_eq!(user_memos.forever_memos[1].content, "First forever memo");
    
    Ok(())
}