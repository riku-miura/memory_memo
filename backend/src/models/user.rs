use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use sqlx::{FromRow, SqlitePool};
use bcrypt::{hash, verify, DEFAULT_COST};
use anyhow::{Result, anyhow};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: String,
    pub username: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
}

impl User {
    /// Create a new user with hashed password
    pub async fn create(pool: &SqlitePool, username: &str, password: &str) -> Result<Self> {
        // Validate username length
        if username.len() < 3 || username.len() > 50 {
            return Err(anyhow!("Username must be between 3 and 50 characters"));
        }

        // Hash the password
        let password_hash = hash(password, DEFAULT_COST)?;
        
        // Generate UUID for user ID
        let user_id = Uuid::new_v4().to_string();
        
        // Insert user into database
        sqlx::query(
            "INSERT INTO users (id, username, password_hash) VALUES (?, ?, ?)"
        )
        .bind(&user_id)
        .bind(username)
        .bind(&password_hash)
        .execute(pool)
        .await
        .map_err(|e| anyhow!("Failed to create user: {}", e))?;

        // Retrieve the created user
        let user = sqlx::query_as::<_, User>(
            "SELECT id, username, password_hash, created_at FROM users WHERE id = ?"
        )
        .bind(&user_id)
        .fetch_one(pool)
        .await?;

        Ok(user)
    }

    /// Find user by username
    pub async fn find_by_username(pool: &SqlitePool, username: &str) -> Result<Option<Self>> {
        let user = sqlx::query_as::<_, User>(
            "SELECT id, username, password_hash, created_at FROM users WHERE username = ?"
        )
        .bind(username)
        .fetch_optional(pool)
        .await?;

        Ok(user)
    }

    /// Find user by ID
    pub async fn find_by_id(pool: &SqlitePool, user_id: &str) -> Result<Option<Self>> {
        let user = sqlx::query_as::<_, User>(
            "SELECT id, username, password_hash, created_at FROM users WHERE id = ?"
        )
        .bind(user_id)
        .fetch_optional(pool)
        .await?;

        Ok(user)
    }

    /// Verify password against stored hash
    pub fn verify_password(&self, password: &str) -> bool {
        verify(password, &self.password_hash).unwrap_or(false)
    }

    /// Get public user info (without password hash)
    pub fn to_public(&self) -> PublicUser {
        PublicUser {
            id: self.id.clone(),
            username: self.username.clone(),
            created_at: self.created_at,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicUser {
    pub id: String,
    pub username: String,
    pub created_at: DateTime<Utc>,
}