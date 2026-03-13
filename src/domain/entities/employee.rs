use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name = crate::domain::entities::schema::employees)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Employee {
    pub id: i32,
    pub uuid: String,
    pub name: String,
    pub email: String,
    pub position: String,
    pub salary: i32,
    pub department_id: i32,
    pub department_uuid: String,
    pub created_at: String,
    pub updated_at: Option<String>,
}

impl Employee {
    pub fn new(
        name: String,
        email: String,
        position: String,
        salary: i64,
        department_id: i32,
        department_uuid: String,
    ) -> Self {
        let now = chrono::Utc::now().to_rfc3339();
        Self {
            id: 0,
            uuid: Uuid::new_v4().to_string(),
            name,
            email,
            position,
            salary: salary as i32,
            department_id,
            department_uuid,
            created_at: now,
            updated_at: None,
        }
    }
}
