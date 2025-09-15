use crate::models::{ForeverMemo, FlushMemo};
use crate::database::DatabasePool;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct MemoService {
    pool: DatabasePool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserMemos {
    pub flush_memos: Vec<FlushMemo>,
    pub forever_memos: Vec<ForeverMemo>,
}

impl MemoService {
    pub fn new(pool: DatabasePool) -> Self {
        Self { pool }
    }

    /// Create a new forever memo
    pub async fn create_forever_memo(&self, user_id: &str, content: &str) -> Result<ForeverMemo> {
        ForeverMemo::create(&self.pool, user_id, content).await
    }

    /// Create a new flush memo
    pub async fn create_flush_memo(&self, user_id: &str, content: &str) -> Result<FlushMemo> {
        FlushMemo::create(&self.pool, user_id, content).await
    }

    /// List all memos for a user (flush memos first, then forever memos)
    pub async fn list_user_memos(&self, user_id: &str) -> Result<UserMemos> {
        // Get both types of memos concurrently
        let (flush_memos_result, forever_memos_result) = tokio::join!(
            FlushMemo::list_by_user(&self.pool, user_id),
            ForeverMemo::list_by_user(&self.pool, user_id)
        );

        Ok(UserMemos {
            flush_memos: flush_memos_result?,
            forever_memos: forever_memos_result?,
        })
    }

    /// Delete a memo (works for both forever and flush memos)
    pub async fn delete_memo(&self, memo_id: &str, user_id: &str) -> Result<()> {
        // Try to delete from forever memos first
        let forever_result = ForeverMemo::delete(&self.pool, memo_id, user_id).await;
        
        match forever_result {
            Ok(()) => Ok(()), // Successfully deleted from forever memos
            Err(_) => {
                // If not found in forever memos, try flush memos
                FlushMemo::delete(&self.pool, memo_id, user_id).await
            }
        }
    }

    /// Get a specific forever memo by ID (if owned by user)
    pub async fn get_forever_memo(&self, memo_id: &str, user_id: &str) -> Result<Option<ForeverMemo>> {
        ForeverMemo::find_by_id_and_user(&self.pool, memo_id, user_id).await
    }

    /// Get a specific flush memo by ID (if owned by user and not expired)
    pub async fn get_flush_memo(&self, memo_id: &str, user_id: &str) -> Result<Option<FlushMemo>> {
        FlushMemo::find_by_id_and_user(&self.pool, memo_id, user_id).await
    }

    /// Update a forever memo's content
    pub async fn update_forever_memo(&self, memo_id: &str, user_id: &str, new_content: &str) -> Result<Option<ForeverMemo>> {
        ForeverMemo::update(&self.pool, memo_id, user_id, new_content).await
    }

    /// Delete a forever memo specifically
    pub async fn delete_forever_memo(&self, memo_id: &str, user_id: &str) -> Result<bool> {
        match ForeverMemo::delete(&self.pool, memo_id, user_id).await {
            Ok(()) => Ok(true),
            Err(_) => Ok(false), // Memo not found or not owned by user
        }
    }

    /// Delete a flush memo specifically  
    pub async fn delete_flush_memo(&self, memo_id: &str, user_id: &str) -> Result<bool> {
        match FlushMemo::delete(&self.pool, memo_id, user_id).await {
            Ok(()) => Ok(true),
            Err(_) => Ok(false), // Memo not found or not owned by user
        }
    }

    /// Count user's memos
    pub async fn count_user_memos(&self, user_id: &str) -> Result<(usize, usize)> {
        let user_memos = self.list_user_memos(user_id).await?;
        Ok((user_memos.forever_memos.len(), user_memos.flush_memos.len()))
    }
}