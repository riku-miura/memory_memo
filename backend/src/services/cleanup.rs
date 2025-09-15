use crate::models::FlushMemo;
use crate::database::DatabasePool;
use anyhow::Result;
use chrono::{DateTime, Utc, Duration};
use serde::{Deserialize, Serialize};
use sqlx::Row;

#[derive(Clone)]
pub struct CleanupService {
    pool: DatabasePool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CleanupStatistics {
    pub total_count: i64,
    pub expired_count: i64,
    pub active_count: i64,
    pub last_cleanup: Option<DateTime<Utc>>,
}

impl CleanupService {
    pub fn new(pool: DatabasePool) -> Self {
        Self { pool }
    }

    /// Clean up expired flush memos and return count of deleted memos
    pub async fn cleanup_expired_flush_memos(&self) -> Result<u64> {
        let cleaned_count = FlushMemo::cleanup_expired(&self.pool).await?;
        
        // Log the cleanup run
        self.log_cleanup_run(cleaned_count, Utc::now()).await?;
        
        tracing::info!("Cleanup completed: {} expired flush memos removed", cleaned_count);
        Ok(cleaned_count)
    }

    /// Dry run cleanup - show what would be deleted without actually deleting
    pub async fn cleanup_expired_flush_memos_dry_run(&self) -> Result<u64> {
        let now = Utc::now();
        let result = sqlx::query(
            "SELECT COUNT(*) as count FROM flush_memos WHERE expires_at <= ?"
        )
        .bind(now)
        .fetch_one(&self.pool)
        .await?;

        let count: i64 = result.get("count");
        tracing::info!("Dry run: {} expired flush memos would be removed", count);
        Ok(count as u64)
    }

    /// Get cleanup statistics
    pub async fn get_cleanup_statistics(&self) -> Result<CleanupStatistics> {
        let now = Utc::now();
        
        // Get total count
        let total_result = sqlx::query("SELECT COUNT(*) as count FROM flush_memos")
            .fetch_one(&self.pool)
            .await?;
        let total_count: i64 = total_result.get("count");
        
        // Get expired count
        let expired_result = sqlx::query(
            "SELECT COUNT(*) as count FROM flush_memos WHERE expires_at <= ?"
        )
        .bind(now)
        .fetch_one(&self.pool)
        .await?;
        let expired_count: i64 = expired_result.get("count");
        
        let active_count = total_count - expired_count;
        
        // Get last cleanup time (if cleanup log table exists)
        let last_cleanup = self.get_last_cleanup_time().await.ok();
        
        Ok(CleanupStatistics {
            total_count,
            expired_count,
            active_count,
            last_cleanup,
        })
    }

    /// Get next suggested cleanup time (current time + 1 hour)
    pub async fn get_next_cleanup_time(&self) -> Result<DateTime<Utc>> {
        Ok(Utc::now() + Duration::hours(1))
    }

    /// Log cleanup run (create simple log table if needed)
    pub async fn log_cleanup_run(&self, cleaned_count: u64, run_time: DateTime<Utc>) -> Result<()> {
        // Create cleanup log table if it doesn't exist
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS cleanup_logs (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                cleaned_count INTEGER NOT NULL,
                run_time DATETIME NOT NULL
            )
            "#
        )
        .execute(&self.pool)
        .await?;
        
        // Insert log entry
        sqlx::query(
            "INSERT INTO cleanup_logs (cleaned_count, run_time) VALUES (?, ?)"
        )
        .bind(cleaned_count as i64)
        .bind(run_time)
        .execute(&self.pool)
        .await?;
        
        tracing::debug!("Logged cleanup run: {} items cleaned at {}", cleaned_count, run_time);
        Ok(())
    }

    /// Get last cleanup time from logs
    async fn get_last_cleanup_time(&self) -> Result<DateTime<Utc>> {
        let result = sqlx::query(
            "SELECT run_time FROM cleanup_logs ORDER BY run_time DESC LIMIT 1"
        )
        .fetch_one(&self.pool)
        .await?;
        
        Ok(result.get("run_time"))
    }

    /// Get cleanup history (last N runs)
    pub async fn get_cleanup_history(&self, limit: i64) -> Result<Vec<(u64, DateTime<Utc>)>> {
        // Create table if it doesn't exist
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS cleanup_logs (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                cleaned_count INTEGER NOT NULL,
                run_time DATETIME NOT NULL
            )
            "#
        )
        .execute(&self.pool)
        .await?;
        
        let rows = sqlx::query(
            "SELECT cleaned_count, run_time FROM cleanup_logs ORDER BY run_time DESC LIMIT ?"
        )
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;
        
        let history = rows
            .into_iter()
            .map(|row| {
                let count: i64 = row.get("cleaned_count");
                let time: DateTime<Utc> = row.get("run_time");
                (count as u64, time)
            })
            .collect();
        
        Ok(history)
    }

    /// Force cleanup all flush memos regardless of expiry (for testing/admin use)
    pub async fn force_cleanup_all_flush_memos(&self) -> Result<u64> {
        let result = sqlx::query("DELETE FROM flush_memos")
            .execute(&self.pool)
            .await?;
        
        let cleaned_count = result.rows_affected();
        self.log_cleanup_run(cleaned_count, Utc::now()).await?;
        
        tracing::warn!("Force cleanup: {} flush memos removed (all)", cleaned_count);
        Ok(cleaned_count)
    }
}