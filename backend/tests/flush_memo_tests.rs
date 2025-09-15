use memory_memo::models::{User, FlushMemo};
use memory_memo::database::create_test_database;
use anyhow::Result;
use chrono::{Duration, Utc};
use sqlx::Row;

#[tokio::test]
async fn test_flush_memo_creation() -> Result<()> {
    let pool = create_test_database().await?;
    
    // Create a user first
    let user = User::create(&pool, "testuser", "password123").await?;
    
    // Test flush memo creation
    let memo = FlushMemo::create(&pool, &user.id, "This is a test flush memo").await?;
    assert_eq!(memo.user_id, user.id);
    assert_eq!(memo.content, "This is a test flush memo");
    assert!(!memo.id.is_empty());
    
    // Expires at should be approximately 24 hours from creation
    let expected_expiry = memo.created_at + Duration::hours(24);
    let actual_expiry = memo.expires_at;
    let diff = if actual_expiry > expected_expiry {
        actual_expiry - expected_expiry
    } else {
        expected_expiry - actual_expiry
    };
    
    assert!(diff < Duration::minutes(1), "Expiry time should be approximately 24 hours from creation");
    
    Ok(())
}

#[tokio::test]
async fn test_flush_memo_list_by_user() -> Result<()> {
    let pool = create_test_database().await?;
    
    // Create two users
    let user1 = User::create(&pool, "user1", "password1").await?;
    let user2 = User::create(&pool, "user2", "password2").await?;
    
    // Create memos for both users
    FlushMemo::create(&pool, &user1.id, "User1 flush memo 1").await?;
    FlushMemo::create(&pool, &user2.id, "User2 flush memo 1").await?;
    FlushMemo::create(&pool, &user1.id, "User1 flush memo 2").await?;
    
    // Get memos for user1
    let user1_memos = FlushMemo::list_by_user(&pool, &user1.id).await?;
    assert_eq!(user1_memos.len(), 2);
    
    // Get memos for user2
    let user2_memos = FlushMemo::list_by_user(&pool, &user2.id).await?;
    assert_eq!(user2_memos.len(), 1);
    
    // Verify content
    assert!(user1_memos.iter().any(|m| m.content == "User1 flush memo 1"));
    assert!(user1_memos.iter().any(|m| m.content == "User1 flush memo 2"));
    assert!(user2_memos.iter().any(|m| m.content == "User2 flush memo 1"));
    
    Ok(())
}

#[tokio::test]
async fn test_flush_memo_ordering() -> Result<()> {
    let pool = create_test_database().await?;
    
    let user = User::create(&pool, "testuser", "password123").await?;
    
    // Create memos in sequence
    let _memo1 = FlushMemo::create(&pool, &user.id, "First flush memo").await?;
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await; // Ensure different timestamps
    let _memo2 = FlushMemo::create(&pool, &user.id, "Second flush memo").await?;
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    let _memo3 = FlushMemo::create(&pool, &user.id, "Third flush memo").await?;
    
    let memos = FlushMemo::list_by_user(&pool, &user.id).await?;
    
    // Should be ordered by newest first (created_at DESC)
    assert_eq!(memos.len(), 3);
    
    // Check ordering - newest should be first
    let contents: Vec<&str> = memos.iter().map(|m| m.content.as_str()).collect();
    assert_eq!(contents[0], "Third flush memo", "Newest flush memo should be first");
    
    // Verify all memos are present
    assert!(contents.contains(&"First flush memo"));
    assert!(contents.contains(&"Second flush memo"));
    assert!(contents.contains(&"Third flush memo"));
    
    Ok(())
}

#[tokio::test]
async fn test_flush_memo_delete() -> Result<()> {
    let pool = create_test_database().await?;
    
    let user = User::create(&pool, "testuser", "password123").await?;
    let memo = FlushMemo::create(&pool, &user.id, "Flush memo to delete").await?;
    
    // Verify memo exists
    let memos_before = FlushMemo::list_by_user(&pool, &user.id).await?;
    assert_eq!(memos_before.len(), 1);
    
    // Delete memo
    FlushMemo::delete(&pool, &memo.id, &user.id).await?;
    
    // Verify memo is deleted
    let memos_after = FlushMemo::list_by_user(&pool, &user.id).await?;
    assert_eq!(memos_after.len(), 0);
    
    Ok(())
}

#[tokio::test]
async fn test_flush_memo_delete_security() -> Result<()> {
    let pool = create_test_database().await?;
    
    // Create two users
    let user1 = User::create(&pool, "user1", "password1").await?;
    let user2 = User::create(&pool, "user2", "password2").await?;
    
    // User1 creates a memo
    let memo = FlushMemo::create(&pool, &user1.id, "User1's flush memo").await?;
    
    // User2 tries to delete User1's memo - should fail
    let result = FlushMemo::delete(&pool, &memo.id, &user2.id).await;
    assert!(result.is_err());
    
    // Verify memo still exists
    let memos = FlushMemo::list_by_user(&pool, &user1.id).await?;
    assert_eq!(memos.len(), 1);
    
    Ok(())
}

#[tokio::test]
async fn test_flush_memo_cleanup_expired() -> Result<()> {
    let pool = create_test_database().await?;
    
    let user = User::create(&pool, "testuser", "password123").await?;
    
    // Create a flush memo
    let memo = FlushMemo::create(&pool, &user.id, "Memo to expire").await?;
    
    // Manually set expiry to past (simulate expired memo)
    let past_time = Utc::now() - Duration::hours(1);
    sqlx::query("UPDATE flush_memos SET expires_at = ? WHERE id = ?")
        .bind(past_time)
        .bind(&memo.id)
        .execute(&pool)
        .await?;
    
    // Create another memo that hasn't expired
    FlushMemo::create(&pool, &user.id, "Fresh memo").await?;
    
    // Before cleanup - should have 2 memos total (but list_by_user only returns non-expired)
    let all_memos_count = sqlx::query("SELECT COUNT(*) as count FROM flush_memos WHERE user_id = ?")
        .bind(&user.id)
        .fetch_one(&pool)
        .await?;
    let count: i64 = all_memos_count.get("count");
    assert_eq!(count, 2, "Should have 2 total memos before cleanup");
    
    // list_by_user should only show 1 (the non-expired one)
    let memos_before = FlushMemo::list_by_user(&pool, &user.id).await?;
    assert_eq!(memos_before.len(), 1, "list_by_user should only show non-expired memos");
    
    // Run cleanup
    let cleaned_count = FlushMemo::cleanup_expired(&pool).await?;
    assert_eq!(cleaned_count, 1);
    
    // After cleanup - should have 1 memo (the fresh one)
    let memos_after = FlushMemo::list_by_user(&pool, &user.id).await?;
    assert_eq!(memos_after.len(), 1);
    assert_eq!(memos_after[0].content, "Fresh memo");
    
    Ok(())
}

#[tokio::test]
async fn test_flush_memo_list_excludes_expired() -> Result<()> {
    let pool = create_test_database().await?;
    
    let user = User::create(&pool, "testuser", "password123").await?;
    
    // Create two flush memos
    let _memo1 = FlushMemo::create(&pool, &user.id, "Active memo").await?;
    let memo2 = FlushMemo::create(&pool, &user.id, "Expired memo").await?;
    
    // Manually set one memo to be expired
    let past_time = Utc::now() - Duration::hours(1);
    sqlx::query("UPDATE flush_memos SET expires_at = ? WHERE id = ?")
        .bind(past_time)
        .bind(&memo2.id)
        .execute(&pool)
        .await?;
    
    // List should only return non-expired memos
    let memos = FlushMemo::list_by_user(&pool, &user.id).await?;
    assert_eq!(memos.len(), 1);
    assert_eq!(memos[0].content, "Active memo");
    
    Ok(())
}

#[tokio::test]
async fn test_flush_memo_empty_content() -> Result<()> {
    let pool = create_test_database().await?;
    
    let user = User::create(&pool, "testuser", "password123").await?;
    
    // Try to create memo with empty content - should fail
    let result = FlushMemo::create(&pool, &user.id, "").await;
    assert!(result.is_err());
    
    Ok(())
}