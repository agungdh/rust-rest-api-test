use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name = crate::domain::entities::schema::departments)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Department {
    pub id: i32,
    pub uuid: String,
    pub name: String,
    pub created_at: String,
    pub updated_at: Option<String>,
}

impl Department {
    pub fn new(name: String) -> Self {
        let now = chrono::Utc::now().to_rfc3339();
        Self {
            id: 0,
            uuid: Uuid::new_v4().to_string(),
            name,
            created_at: now,
            updated_at: None,
        }
    }
}
