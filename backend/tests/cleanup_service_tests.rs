use memory_memo::services::CleanupService;
use memory_memo::models::{User, FlushMemo};
use memory_memo::database::create_test_database;
use anyhow::Result;
use chrono::{Duration, Utc};
use sqlx::Row;

#[tokio::test]
async fn test_cleanup_expired_flush_memos() -> Result<()> {
    let pool = create_test_database().await?;
    let cleanup_service = CleanupService::new(pool.clone());
    
    let user = User::create(&pool, "testuser", "password123").await?;
    
    // Create some memos
    let fresh_memo = FlushMemo::create(&pool, &user.id, "Fresh memo").await?;
    let expired_memo = FlushMemo::create(&pool, &user.id, "Expired memo").await?;
    
    // Manually set one memo to be expired
    let past_time = Utc::now() - Duration::hours(25);
    sqlx::query("UPDATE flush_memos SET expires_at = ? WHERE id = ?")
        .bind(past_time)
        .bind(&expired_memo.id)
        .execute(&pool)
        .await?;
    
    // Run cleanup
    let cleaned_count = cleanup_service.cleanup_expired_flush_memos().await?;
    assert_eq!(cleaned_count, 1);
    
    // Check that only fresh memo remains
    let remaining_memos = FlushMemo::list_by_user(&pool, &user.id).await?;
    assert_eq!(remaining_memos.len(), 1);
    assert_eq!(remaining_memos[0].content, "Fresh memo");
    
    Ok(())
}

#[tokio::test]
async fn test_cleanup_no_expired_memos() -> Result<()> {
    let pool = create_test_database().await?;
    let cleanup_service = CleanupService::new(pool.clone());
    
    let user = User::create(&pool, "testuser", "password123").await?;
    
    // Create only fresh memos
    FlushMemo::create(&pool, &user.id, "Fresh memo 1").await?;
    FlushMemo::create(&pool, &user.id, "Fresh memo 2").await?;
    
    // Run cleanup - should not delete anything
    let cleaned_count = cleanup_service.cleanup_expired_flush_memos().await?;
    assert_eq!(cleaned_count, 0);
    
    // Check that all memos remain
    let remaining_memos = FlushMemo::list_by_user(&pool, &user.id).await?;
    assert_eq!(remaining_memos.len(), 2);
    
    Ok(())
}

#[tokio::test]
async fn test_cleanup_multiple_users() -> Result<()> {
    let pool = create_test_database().await?;
    let cleanup_service = CleanupService::new(pool.clone());
    
    let user1 = User::create(&pool, "user1", "password1").await?;
    let user2 = User::create(&pool, "user2", "password2").await?;
    
    // Create memos for both users
    let user1_expired = FlushMemo::create(&pool, &user1.id, "User1 expired").await?;
    let user1_fresh = FlushMemo::create(&pool, &user1.id, "User1 fresh").await?;
    let user2_expired = FlushMemo::create(&pool, &user2.id, "User2 expired").await?;
    let user2_fresh = FlushMemo::create(&pool, &user2.id, "User2 fresh").await?;
    
    // Set expired times
    let past_time = Utc::now() - Duration::hours(25);
    for expired_id in [&user1_expired.id, &user2_expired.id] {
        sqlx::query("UPDATE flush_memos SET expires_at = ? WHERE id = ?")
            .bind(past_time)
            .bind(expired_id)
            .execute(&pool)
            .await?;
    }
    
    // Run cleanup
    let cleaned_count = cleanup_service.cleanup_expired_flush_memos().await?;
    assert_eq!(cleaned_count, 2);
    
    // Check that only fresh memos remain for both users
    let user1_remaining = FlushMemo::list_by_user(&pool, &user1.id).await?;
    assert_eq!(user1_remaining.len(), 1);
    assert_eq!(user1_remaining[0].content, "User1 fresh");
    
    let user2_remaining = FlushMemo::list_by_user(&pool, &user2.id).await?;
    assert_eq!(user2_remaining.len(), 1);
    assert_eq!(user2_remaining[0].content, "User2 fresh");
    
    Ok(())
}

#[tokio::test]
async fn test_cleanup_statistics() -> Result<()> {
    let pool = create_test_database().await?;
    let cleanup_service = CleanupService::new(pool.clone());
    
    let user = User::create(&pool, "testuser", "password123").await?;
    
    // Create several memos with different expiry times
    let memo1 = FlushMemo::create(&pool, &user.id, "Will expire").await?;
    let memo2 = FlushMemo::create(&pool, &user.id, "Will also expire").await?;
    let _memo3 = FlushMemo::create(&pool, &user.id, "Will remain fresh").await?;
    
    // Set some to be expired
    let past_time = Utc::now() - Duration::hours(25);
    for expired_id in [&memo1.id, &memo2.id] {
        sqlx::query("UPDATE flush_memos SET expires_at = ? WHERE id = ?")
            .bind(past_time)
            .bind(expired_id)
            .execute(&pool)
            .await?;
    }
    
    // Get cleanup statistics
    let stats_before = cleanup_service.get_cleanup_statistics().await?;
    assert!(stats_before.expired_count >= 2);
    assert!(stats_before.total_count >= 3);
    
    // Run cleanup
    let cleaned_count = cleanup_service.cleanup_expired_flush_memos().await?;
    assert_eq!(cleaned_count, 2);
    
    // Get statistics after cleanup
    let stats_after = cleanup_service.get_cleanup_statistics().await?;
    assert_eq!(stats_after.expired_count, 0);
    assert_eq!(stats_after.total_count, 1);
    
    Ok(())
}

#[tokio::test]
async fn test_cleanup_dry_run() -> Result<()> {
    let pool = create_test_database().await?;
    let cleanup_service = CleanupService::new(pool.clone());
    
    let user = User::create(&pool, "testuser", "password123").await?;
    
    // Create an expired memo
    let expired_memo = FlushMemo::create(&pool, &user.id, "Will be checked but not deleted").await?;
    let past_time = Utc::now() - Duration::hours(25);
    sqlx::query("UPDATE flush_memos SET expires_at = ? WHERE id = ?")
        .bind(past_time)
        .bind(&expired_memo.id)
        .execute(&pool)
        .await?;
    
    // Run dry run - should report what would be deleted but not actually delete
    let would_be_cleaned = cleanup_service.cleanup_expired_flush_memos_dry_run().await?;
    assert_eq!(would_be_cleaned, 1);
    
    // Verify memo is still there
    let all_memos_count = sqlx::query("SELECT COUNT(*) as count FROM flush_memos WHERE user_id = ?")
        .bind(&user.id)
        .fetch_one(&pool)
        .await?;
    let count: i64 = all_memos_count.get("count");
    assert_eq!(count, 1);
    
    Ok(())
}

#[tokio::test]
async fn test_cleanup_with_scheduling_info() -> Result<()> {
    let pool = create_test_database().await?;
    let cleanup_service = CleanupService::new(pool.clone());
    
    // Test that we can get next suggested cleanup time
    let next_cleanup = cleanup_service.get_next_cleanup_time().await?;
    assert!(next_cleanup > Utc::now());
    
    // Test that we can log cleanup run
    cleanup_service.log_cleanup_run(5, Utc::now()).await?;
    
    Ok(())
}