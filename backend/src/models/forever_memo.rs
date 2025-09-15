use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use sqlx::{FromRow, SqlitePool};
use anyhow::{Result, anyhow};
use uuid::Uuid;
use crate::database::DatabasePool;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ForeverMemo {
    pub id: String,
    pub user_id: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
}

impl ForeverMemo {
    /// Create a new forever memo
    pub async fn create(pool: &SqlitePool, user_id: &str, content: &str) -> Result<Self> {
        // Validate content is not empty
        if content.trim().is_empty() {
            return Err(anyhow!("Memo content cannot be empty"));
        }

        // Generate UUID for memo ID
        let memo_id = Uuid::new_v4().to_string();
        
        // Insert memo into database
        sqlx::query(
            "INSERT INTO forever_memos (id, user_id, content) VALUES (?, ?, ?)"
        )
        .bind(&memo_id)
        .bind(user_id)
        .bind(content)
        .execute(pool)
        .await
        .map_err(|e| anyhow!("Failed to create forever memo: {}", e))?;

        // Retrieve the created memo
        let memo = sqlx::query_as::<_, ForeverMemo>(
            "SELECT id, user_id, content, created_at FROM forever_memos WHERE id = ?"
        )
        .bind(&memo_id)
        .fetch_one(pool)
        .await?;

        Ok(memo)
    }

    /// List all forever memos for a specific user, ordered by newest first
    pub async fn list_by_user(pool: &SqlitePool, user_id: &str) -> Result<Vec<Self>> {
        let memos = sqlx::query_as::<_, ForeverMemo>(
            "SELECT id, user_id, content, created_at FROM forever_memos 
             WHERE user_id = ? ORDER BY created_at DESC"
        )
        .bind(user_id)
        .fetch_all(pool)
        .await?;

        Ok(memos)
    }

    /// Delete a forever memo (only if owned by the user)
    pub async fn delete(pool: &SqlitePool, memo_id: &str, user_id: &str) -> Result<()> {
        let rows_affected = sqlx::query(
            "DELETE FROM forever_memos WHERE id = ? AND user_id = ?"
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

    /// Find a specific forever memo by ID (only if owned by the user)
    pub async fn find_by_id_and_user(
        pool: &SqlitePool, 
        memo_id: &str, 
        user_id: &str
    ) -> Result<Option<Self>> {
        let memo = sqlx::query_as::<_, ForeverMemo>(
            "SELECT id, user_id, content, created_at FROM forever_memos 
             WHERE id = ? AND user_id = ?"
        )
        .bind(memo_id)
        .bind(user_id)
        .fetch_optional(pool)
        .await?;

        Ok(memo)
    }

    /// Update a forever memo's content (only if owned by the user)
    pub async fn update(pool: &DatabasePool, memo_id: &str, user_id: &str, new_content: &str) -> Result<Option<ForeverMemo>> {
        let updated_memo = sqlx::query_as::<_, ForeverMemo>(
            "UPDATE forever_memos SET content = ? WHERE id = ? AND user_id = ? RETURNING *"
        )
        .bind(new_content)
        .bind(memo_id)
        .bind(user_id)
        .fetch_optional(pool)
        .await?;

        Ok(updated_memo)
    }
}