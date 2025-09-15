use memory_memo::models::{User, ForeverMemo};
use memory_memo::database::create_test_database;
use anyhow::Result;

#[tokio::test]
async fn test_forever_memo_creation() -> Result<()> {
    let pool = create_test_database().await?;
    
    // Create a user first
    let user = User::create(&pool, "testuser", "password123").await?;
    
    // Test forever memo creation
    let memo = ForeverMemo::create(&pool, &user.id, "This is a test memo").await?;
    assert_eq!(memo.user_id, user.id);
    assert_eq!(memo.content, "This is a test memo");
    assert!(!memo.id.is_empty());
    
    Ok(())
}

#[tokio::test]
async fn test_forever_memo_list_by_user() -> Result<()> {
    let pool = create_test_database().await?;
    
    // Create two users
    let user1 = User::create(&pool, "user1", "password1").await?;
    let user2 = User::create(&pool, "user2", "password2").await?;
    
    // Create memos for both users
    ForeverMemo::create(&pool, &user1.id, "User1 memo 1").await?;
    ForeverMemo::create(&pool, &user2.id, "User2 memo 1").await?;
    ForeverMemo::create(&pool, &user1.id, "User1 memo 2").await?;
    
    // Get memos for user1
    let user1_memos = ForeverMemo::list_by_user(&pool, &user1.id).await?;
    assert_eq!(user1_memos.len(), 2);
    
    // Get memos for user2
    let user2_memos = ForeverMemo::list_by_user(&pool, &user2.id).await?;
    assert_eq!(user2_memos.len(), 1);
    
    // Verify content
    assert!(user1_memos.iter().any(|m| m.content == "User1 memo 1"));
    assert!(user1_memos.iter().any(|m| m.content == "User1 memo 2"));
    assert!(user2_memos.iter().any(|m| m.content == "User2 memo 1"));
    
    Ok(())
}

#[tokio::test]
async fn test_forever_memo_ordering() -> Result<()> {
    let pool = create_test_database().await?;
    
    let user = User::create(&pool, "testuser", "password123").await?;
    
    // Create memos in sequence
    let _memo1 = ForeverMemo::create(&pool, &user.id, "First memo").await?;
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await; // Ensure different timestamps
    let _memo2 = ForeverMemo::create(&pool, &user.id, "Second memo").await?;
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    let _memo3 = ForeverMemo::create(&pool, &user.id, "Third memo").await?;
    
    let memos = ForeverMemo::list_by_user(&pool, &user.id).await?;
    
    // Should be ordered by newest first (created_at DESC)
    assert_eq!(memos.len(), 3);
    
    // Check ordering by content and timing - newest should be first
    let contents: Vec<&str> = memos.iter().map(|m| m.content.as_str()).collect();
    
    // At minimum, the last created memo should be first
    assert_eq!(contents[0], "Third memo", "Newest memo should be first");
    
    // Verify all memos are present
    assert!(contents.contains(&"First memo"));
    assert!(contents.contains(&"Second memo"));
    assert!(contents.contains(&"Third memo"));
    
    Ok(())
}

#[tokio::test]
async fn test_forever_memo_delete() -> Result<()> {
    let pool = create_test_database().await?;
    
    let user = User::create(&pool, "testuser", "password123").await?;
    let memo = ForeverMemo::create(&pool, &user.id, "Memo to delete").await?;
    
    // Verify memo exists
    let memos_before = ForeverMemo::list_by_user(&pool, &user.id).await?;
    assert_eq!(memos_before.len(), 1);
    
    // Delete memo
    ForeverMemo::delete(&pool, &memo.id, &user.id).await?;
    
    // Verify memo is deleted
    let memos_after = ForeverMemo::list_by_user(&pool, &user.id).await?;
    assert_eq!(memos_after.len(), 0);
    
    Ok(())
}

#[tokio::test]
async fn test_forever_memo_delete_security() -> Result<()> {
    let pool = create_test_database().await?;
    
    // Create two users
    let user1 = User::create(&pool, "user1", "password1").await?;
    let user2 = User::create(&pool, "user2", "password2").await?;
    
    // User1 creates a memo
    let memo = ForeverMemo::create(&pool, &user1.id, "User1's memo").await?;
    
    // User2 tries to delete User1's memo - should fail
    let result = ForeverMemo::delete(&pool, &memo.id, &user2.id).await;
    assert!(result.is_err());
    
    // Verify memo still exists
    let memos = ForeverMemo::list_by_user(&pool, &user1.id).await?;
    assert_eq!(memos.len(), 1);
    
    Ok(())
}

#[tokio::test]
async fn test_forever_memo_empty_content() -> Result<()> {
    let pool = create_test_database().await?;
    
    let user = User::create(&pool, "testuser", "password123").await?;
    
    // Try to create memo with empty content - should fail
    let result = ForeverMemo::create(&pool, &user.id, "").await;
    assert!(result.is_err());
    
    Ok(())
}