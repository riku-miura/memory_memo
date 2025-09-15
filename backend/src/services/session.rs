use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use chrono::{DateTime, Utc, Duration};
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct Session {
    pub user_id: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

impl Session {
    pub fn new(user_id: String) -> Self {
        let now = Utc::now();
        let expires_at = now + Duration::hours(24); // 24 hour session

        Self {
            user_id,
            created_at: now,
            expires_at,
        }
    }

    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expires_at
    }
}

#[derive(Clone)]
pub struct SessionStore {
    sessions: Arc<RwLock<HashMap<String, Session>>>,
}

impl SessionStore {
    pub fn new() -> Self {
        Self {
            sessions: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Create a new session for a user
    pub fn create_session(&self, user_id: String) -> String {
        let session_id = Uuid::new_v4().to_string();
        let session = Session::new(user_id);

        if let Ok(mut sessions) = self.sessions.write() {
            sessions.insert(session_id.clone(), session);
        }

        session_id
    }

    /// Get session by ID
    pub fn get_session(&self, session_id: &str) -> Option<Session> {
        if let Ok(sessions) = self.sessions.read() {
            sessions.get(session_id).cloned()
        } else {
            None
        }
    }

    /// Remove a session
    pub fn remove_session(&self, session_id: &str) -> bool {
        if let Ok(mut sessions) = self.sessions.write() {
            sessions.remove(session_id).is_some()
        } else {
            false
        }
    }

    /// Clean up expired sessions
    pub fn cleanup_expired(&self) {
        if let Ok(mut sessions) = self.sessions.write() {
            sessions.retain(|_, session| !session.is_expired());
        }
    }

    /// Get user ID from session
    pub fn get_user_id(&self, session_id: &str) -> Option<String> {
        if let Some(session) = self.get_session(session_id) {
            if !session.is_expired() {
                Some(session.user_id)
            } else {
                // Clean up expired session
                self.remove_session(session_id);
                None
            }
        } else {
            None
        }
    }
}

impl Default for SessionStore {
    fn default() -> Self {
        Self::new()
    }
}