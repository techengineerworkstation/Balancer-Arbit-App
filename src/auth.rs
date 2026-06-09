use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};
use sled::Db;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub password_hash: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub token: String,
    pub username: String,
    pub created_at: String,
    pub expires_at: String,
}

pub struct AuthStore {
    db: Db,
}

impl AuthStore {
    pub fn new() -> Result<Self, String> {
        std::fs::create_dir_all("data").map_err(|e| format!("Failed to create data dir: {}", e))?;
        let db =
            sled::open("data/auth.db").map_err(|e| format!("Failed to open auth db: {}", e))?;
        Ok(Self { db })
    }

    fn users_tree(&self) -> sled::Tree {
        self.db
            .open_tree("users")
            .expect("Failed to open users tree")
    }

    fn sessions_tree(&self) -> sled::Tree {
        self.db
            .open_tree("sessions")
            .expect("Failed to open sessions tree")
    }

    pub fn register(&self, username: &str, password: &str) -> Result<String, String> {
        if username.is_empty() || password.is_empty() {
            return Err("Username and password are required".to_string());
        }

        if password.len() < 6 {
            return Err("Password must be at least 6 characters".to_string());
        }

        let users = self.users_tree();
        if users.contains_key(username).map_err(|e| e.to_string())? {
            return Err("Username already exists".to_string());
        }

        let password_hash =
            bcrypt::hash(password, 10).map_err(|e| format!("Failed to hash password: {}", e))?;

        let user = User {
            username: username.to_string(),
            password_hash,
            created_at: Utc::now().to_rfc3339(),
        };

        let user_json =
            serde_json::to_string(&user).map_err(|e| format!("Failed to serialize user: {}", e))?;

        users
            .insert(username.as_bytes(), user_json.as_bytes())
            .map_err(|e| format!("Failed to store user: {}", e))?;

        log::info!("User registered: {}", username);

        let token = Uuid::new_v4().to_string();
        let now = Utc::now();
        let expires = now + Duration::hours(24);

        let session = Session {
            token: token.clone(),
            username: username.to_string(),
            created_at: now.to_rfc3339(),
            expires_at: expires.to_rfc3339(),
        };

        let session_json = serde_json::to_string(&session)
            .map_err(|e| format!("Failed to serialize session: {}", e))?;

        let sessions = self.sessions_tree();
        sessions
            .insert(token.as_bytes(), session_json.as_bytes())
            .map_err(|e| format!("Failed to store session: {}", e))?;

        Ok(token)
    }

    pub fn login(&self, username: &str, password: &str) -> Result<String, String> {
        let users = self.users_tree();
        let user_data = users
            .get(username)
            .map_err(|e| format!("Database error: {}", e))?
            .ok_or_else(|| "Invalid username or password".to_string())?;

        let user: User = serde_json::from_slice(&user_data)
            .map_err(|e| format!("Failed to deserialize user: {}", e))?;

        let valid = bcrypt::verify(password, &user.password_hash)
            .map_err(|e| format!("Failed to verify password: {}", e))?;

        if !valid {
            return Err("Invalid username or password".to_string());
        }

        let token = Uuid::new_v4().to_string();
        let now = Utc::now();
        let expires = now + Duration::hours(24);

        let session = Session {
            token: token.clone(),
            username: username.to_string(),
            created_at: now.to_rfc3339(),
            expires_at: expires.to_rfc3339(),
        };

        let session_json = serde_json::to_string(&session)
            .map_err(|e| format!("Failed to serialize session: {}", e))?;

        let sessions = self.sessions_tree();
        sessions
            .insert(token.as_bytes(), session_json.as_bytes())
            .map_err(|e| format!("Failed to store session: {}", e))?;

        log::info!("User logged in: {}", username);
        Ok(token)
    }

    pub fn validate_token(&self, token: &str) -> Option<Session> {
        let sessions = self.sessions_tree();
        let data = sessions.get(token).ok()??.to_vec();
        let session: Session = serde_json::from_slice(&data).ok()?;

        let now = Utc::now();
        let expires = chrono::DateTime::parse_from_rfc3339(&session.expires_at).ok()?;
        if now > expires {
            let _ = sessions.remove(token.as_bytes());
            return None;
        }

        Some(session)
    }

    pub fn logout(&self, token: &str) -> Result<(), String> {
        let sessions = self.sessions_tree();
        sessions
            .remove(token.as_bytes())
            .map_err(|e| format!("Failed to remove session: {}", e))?;
        log::info!("User logged out, token removed");
        Ok(())
    }

    pub fn get_username(&self, token: &str) -> Option<String> {
        self.validate_token(token).map(|s| s.username)
    }
}
