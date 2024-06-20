use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub activeted_at: Option<DateTime<Utc>>,
}

pub struct UserRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

impl User {
    pub fn new(req: UserRequest) -> Self {
        User {
            id: Uuid::new_v4(),
            username: req.username,
            email: req.email,
            password: req.password,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            deleted_at: None,
            activeted_at: None,
        }
    }
}
