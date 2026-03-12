use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Employee {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub position: String,
    pub salary: i64,
    pub created_at: DateTime<Utc>,
}

impl Employee {
    pub fn new(name: String, email: String, position: String, salary: i64) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            email,
            position,
            salary,
            created_at: Utc::now(),
        }
    }
}
