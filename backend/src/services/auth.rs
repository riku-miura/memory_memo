use crate::models::User;
use crate::models::user::PublicUser;
use crate::database::DatabasePool;
use anyhow::{Result, anyhow};

#[derive(Clone)]
pub struct AuthService {
    pool: DatabasePool,
}

impl AuthService {
    pub fn new(pool: DatabasePool) -> Self {
        Self { pool }
    }

    /// Register a new user
    pub async fn register(&self, username: &str, password: &str) -> Result<PublicUser> {
        // Validate password length (minimum 8 characters)
        if password.len() < 8 {
            return Err(anyhow!("Password must be at least 8 characters long"));
        }

        let user = User::create(&self.pool, username, password).await?;
        Ok(user.to_public())
    }

    /// Login with username and password
    pub async fn login(&self, username: &str, password: &str) -> Result<PublicUser> {
        let user = User::find_by_username(&self.pool, username)
            .await?
            .ok_or_else(|| anyhow!("Invalid username or password"))?;

        if !user.verify_password(password) {
            return Err(anyhow!("Invalid username or password"));
        }

        Ok(user.to_public())
    }

    /// Validate a user by ID (for session validation)
    pub async fn validate_user(&self, user_id: &str) -> Result<Option<PublicUser>> {
        let user = User::find_by_id(&self.pool, user_id).await?;
        Ok(user.map(|u| u.to_public()))
    }

    /// Check if username exists
    pub async fn username_exists(&self, username: &str) -> Result<bool> {
        let user = User::find_by_username(&self.pool, username).await?;
        Ok(user.is_some())
    }

    /// Get user by ID (returns full user info for internal use)
    async fn get_user_by_id(&self, user_id: &str) -> Result<Option<User>> {
        User::find_by_id(&self.pool, user_id).await
    }
}