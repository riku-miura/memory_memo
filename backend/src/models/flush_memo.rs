use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc, Duration};
use sqlx::{FromRow, SqlitePool};
use anyhow::{Result, anyhow};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct FlushMemo {
    pub id: String,
    pub user_id: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

impl FlushMemo {
    /// Create a new flush memo that expires in 24 hours
    pub async fn create(pool: &SqlitePool, user_id: &str, content: &str) -> Result<Self> {
        // Validate content is not empty
        if content.trim().is_empty() {
            return Err(anyhow!("Memo content cannot be empty"));
        }

        // Generate UUID for memo ID
        let memo_id = Uuid::new_v4().to_string();
        
        // Calculate expiry time (24 hours from now)
        let created_at = Utc::now();
        let expires_at = created_at + Duration::hours(24);
        
        // Insert memo into database
        sqlx::query(
            "INSERT INTO flush_memos (id, user_id, content, created_at, expires_at) VALUES (?, ?, ?, ?, ?)"
        )
        .bind(&memo_id)
        .bind(user_id)
        .bind(content)
        .bind(created_at)
        .bind(expires_at)
        .execute(pool)
        .await
        .map_err(|e| anyhow!("Failed to create flush memo: {}", e))?;

        // Retrieve the created memo
        let memo = sqlx::query_as::<_, FlushMemo>(
            "SELECT id, user_id, content, created_at, expires_at FROM flush_memos WHERE id = ?"
        )
        .bind(&memo_id)
        .fetch_one(pool)
        .await?;

        Ok(memo)
    }

    /// List all non-expired flush memos for a specific user, ordered by newest first
    pub async fn list_by_user(pool: &SqlitePool, user_id: &str) -> Result<Vec<Self>> {
        let now = Utc::now();
        let memos = sqlx::query_as::<_, FlushMemo>(
            "SELECT id, user_id, content, created_at, expires_at FROM flush_memos 
             WHERE user_id = ? AND expires_at > ? ORDER BY created_at DESC"
        )
        .bind(user_id)
        .bind(now)
        .fetch_all(pool)
        .await?;

        Ok(memos)
    }

    /// Delete a flush memo (only if owned by the user)
    pub async fn delete(pool: &SqlitePool, memo_id: &str, user_id: &str) -> Result<()> {
        let rows_affected = sqlx::query(
            "DELETE FROM flush_memos WHERE id = ? AND user_id = ?"
        )
        .bind(memo_id)
        .bind(user_id)
        .execute(pool)
        .await?
        .rows_affected();

        if rows_affected == 0 {
            return Err(anyhow!("Memo not found or access denied"));
        }

        Ok(())
    }

    /// Find a specific flush memo by ID (only if owned by the user and not expired)
    pub async fn find_by_id_and_user(
        pool: &SqlitePool, 
        memo_id: &str, 
        user_id: &str
    ) -> Result<Option<Self>> {
        let now = Utc::now();
        let memo = sqlx::query_as::<_, FlushMemo>(
            "SELECT id, user_id, content, created_at, expires_at FROM flush_memos 
             WHERE id = ? AND user_id = ? AND expires_at > ?"
        )
        .bind(memo_id)
        .bind(user_id)
        .bind(now)
        .fetch_optional(pool)
        .await?;

        Ok(memo)
    }

    /// Cleanup expired flush memos (returns number of deleted memos)
    pub async fn cleanup_expired(pool: &SqlitePool) -> Result<u64> {
        let now = Utc::now();
        let result = sqlx::query(
            "DELETE FROM flush_memos WHERE expires_at <= ?"
        )
        .bind(now)
        .execute(pool)
        .await?;

        Ok(result.rows_affected())
    }

    /// Check if this memo is expired
    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expires_at
    }

    /// Get remaining time until expiry
    pub fn time_until_expiry(&self) -> Option<Duration> {
        let now = Utc::now();
        if now >= self.expires_at {
            None
        } else {
            Some(self.expires_at - now)
        }
    }
}