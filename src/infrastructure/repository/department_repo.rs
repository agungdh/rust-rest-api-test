use rusqlite::{Connection, OptionalExtension};

use crate::domain::entities::Department;
use crate::infrastructure::AppError;

pub struct DepartmentRepository {
    conn: Connection,
}

impl DepartmentRepository {
    pub fn new(conn: Connection) -> Self {
        Self { conn }
    }

    pub fn find_all(&self) -> Result<Vec<Department>, AppError> {
        let mut stmt = self.conn.prepare(
            "SELECT id, uuid, name, created_at, updated_at FROM departments ORDER BY created_at DESC"
        ).map_err(|e| AppError::Internal(e.to_string()))?;

        let departments = stmt
            .query_map([], |row| {
                Ok(Department {
                    id: row.get(0)?,
                    uuid: row.get(1)?,
                    name: row.get(2)?,
                    created_at: row.get(3)?,
                    updated_at: row.get(4)?,
                })
            })
            .map_err(|e| AppError::Internal(e.to_string()))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| AppError::Internal(e.to_string()))?;

        Ok(departments)
    }

    pub fn find_by_uuid(&self, uuid: &str) -> Result<Option<Department>, AppError> {
        let mut stmt = self
            .conn
            .prepare(
                "SELECT id, uuid, name, created_at, updated_at FROM departments WHERE uuid = ?",
            )
            .map_err(|e| AppError::Internal(e.to_string()))?;

        let result = stmt
            .query_row([uuid], |row| {
                Ok(Department {
                    id: row.get(0)?,
                    uuid: row.get(1)?,
                    name: row.get(2)?,
                    created_at: row.get(3)?,
                    updated_at: row.get(4)?,
                })
            })
            .optional()
            .map_err(|e| AppError::Internal(e.to_string()))?;

        Ok(result)
    }

    pub fn save(&self, department: &Department) -> Result<Department, AppError> {
        self.conn
            .execute(
                "INSERT INTO departments (uuid, name, created_at, updated_at) VALUES (?, ?, ?, ?)",
                (
                    &department.uuid,
                    &department.name,
                    &department.created_at,
                    &department.updated_at,
                ),
            )
            .map_err(|e| AppError::Internal(e.to_string()))?;

        let id = self.conn.last_insert_rowid();
        let mut dept = department.clone();
        dept.id = id;
        Ok(dept)
    }

    pub fn update(&self, uuid: &str, name: &str) -> Result<Department, AppError> {
        let now = chrono::Utc::now().to_rfc3339();

        let rows_updated = self
            .conn
            .execute(
                "UPDATE departments SET name = ?, updated_at = ? WHERE uuid = ?",
                (name, &now, uuid),
            )
            .map_err(|e| AppError::Internal(e.to_string()))?;

        if rows_updated == 0 {
            return Err(AppError::NotFound(format!(
                "Department with uuid {} not found",
                uuid
            )));
        }

        self.find_by_uuid(uuid)?
            .ok_or_else(|| AppError::NotFound(format!("Department with uuid {} not found", uuid)))
    }

    pub fn delete(&self, uuid: &str) -> Result<(), AppError> {
        let rows_deleted = self
            .conn
            .execute("DELETE FROM departments WHERE uuid = ?", [uuid])
            .map_err(|e| AppError::Internal(e.to_string()))?;

        if rows_deleted == 0 {
            return Err(AppError::NotFound(format!(
                "Department with uuid {} not found",
                uuid
            )));
        }

        Ok(())
    }
}
