use rusqlite::{Connection, OptionalExtension};

use crate::domain::entities::Employee;
use crate::infrastructure::AppError;

pub struct EmployeeRepository {
    conn: Connection,
}

impl EmployeeRepository {
    pub fn new(conn: Connection) -> Self {
        Self { conn }
    }

    pub fn find_all(&self, department_uuid: Option<&str>) -> Result<Vec<Employee>, AppError> {
        let query = if department_uuid.is_some() {
            "SELECT e.id, e.uuid, e.name, e.email, e.position, e.salary, d.uuid, e.created_at, e.updated_at 
             FROM employees e 
             JOIN departments d ON e.department_id = d.id 
             WHERE d.uuid = ? 
             ORDER BY e.created_at DESC"
        } else {
            "SELECT e.id, e.uuid, e.name, e.email, e.position, e.salary, d.uuid, e.created_at, e.updated_at 
             FROM employees e 
             JOIN departments d ON e.department_id = d.id 
             ORDER BY e.created_at DESC"
        };

        let mut stmt = self
            .conn
            .prepare(query)
            .map_err(|e| AppError::Internal(e.to_string()))?;

        let employees = if let Some(dept_uuid) = department_uuid {
            stmt.query_map([dept_uuid], |row| {
                Ok(Employee {
                    id: row.get(0)?,
                    uuid: row.get(1)?,
                    name: row.get(2)?,
                    email: row.get(3)?,
                    position: row.get(4)?,
                    salary: row.get(5)?,
                    department_uuid: row.get(6)?,
                    created_at: row.get(7)?,
                    updated_at: row.get(8)?,
                })
            })
            .map_err(|e| AppError::Internal(e.to_string()))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| AppError::Internal(e.to_string()))?
        } else {
            stmt.query_map([], |row| {
                Ok(Employee {
                    id: row.get(0)?,
                    uuid: row.get(1)?,
                    name: row.get(2)?,
                    email: row.get(3)?,
                    position: row.get(4)?,
                    salary: row.get(5)?,
                    department_uuid: row.get(6)?,
                    created_at: row.get(7)?,
                    updated_at: row.get(8)?,
                })
            })
            .map_err(|e| AppError::Internal(e.to_string()))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| AppError::Internal(e.to_string()))?
        };

        Ok(employees)
    }

    pub fn find_by_uuid(&self, uuid: &str) -> Result<Option<Employee>, AppError> {
        let mut stmt = self.conn.prepare(
            "SELECT e.id, e.uuid, e.name, e.email, e.position, e.salary, d.uuid, e.created_at, e.updated_at 
             FROM employees e 
             JOIN departments d ON e.department_id = d.id 
             WHERE e.uuid = ?"
        ).map_err(|e| AppError::Internal(e.to_string()))?;

        let result = stmt
            .query_row([uuid], |row| {
                Ok(Employee {
                    id: row.get(0)?,
                    uuid: row.get(1)?,
                    name: row.get(2)?,
                    email: row.get(3)?,
                    position: row.get(4)?,
                    salary: row.get(5)?,
                    department_uuid: row.get(6)?,
                    created_at: row.get(7)?,
                    updated_at: row.get(8)?,
                })
            })
            .optional()
            .map_err(|e| AppError::Internal(e.to_string()))?;

        Ok(result)
    }

    pub fn get_department_id_by_uuid(&self, uuid: &str) -> Result<Option<i64>, AppError> {
        let mut stmt = self
            .conn
            .prepare("SELECT id FROM departments WHERE uuid = ?")
            .map_err(|e| AppError::Internal(e.to_string()))?;

        let result = stmt
            .query_row([uuid], |row| row.get(0))
            .optional()
            .map_err(|e| AppError::Internal(e.to_string()))?;

        Ok(result)
    }

    pub fn save(&self, employee: &Employee) -> Result<Employee, AppError> {
        let department_id = self
            .get_department_id_by_uuid(&employee.department_uuid)?
            .ok_or_else(|| {
                AppError::NotFound(format!(
                    "Department with uuid {} not found",
                    employee.department_uuid
                ))
            })?;

        self.conn.execute(
            "INSERT INTO employees (uuid, name, email, position, salary, department_id, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
            (&employee.uuid, &employee.name, &employee.email, &employee.position, &employee.salary, department_id, &employee.created_at, &employee.updated_at),
        ).map_err(|e| AppError::Internal(e.to_string()))?;

        let id = self.conn.last_insert_rowid();
        let mut emp = employee.clone();
        emp.id = id;
        Ok(emp)
    }

    pub fn update(
        &self,
        uuid: &str,
        name: Option<&str>,
        email: Option<&str>,
        position: Option<&str>,
        salary: Option<i64>,
        department_uuid: Option<&str>,
    ) -> Result<Employee, AppError> {
        let existing = self
            .find_by_uuid(uuid)?
            .ok_or_else(|| AppError::NotFound(format!("Employee with uuid {} not found", uuid)))?;

        let new_name = name.unwrap_or(&existing.name);
        let new_email = email.unwrap_or(&existing.email);
        let new_position = position.unwrap_or(&existing.position);
        let new_salary = salary.unwrap_or(existing.salary);

        let new_department_id = if let Some(dept_uuid) = department_uuid {
            self.get_department_id_by_uuid(dept_uuid)?.ok_or_else(|| {
                AppError::NotFound(format!("Department with uuid {} not found", dept_uuid))
            })?
        } else {
            self.get_department_id_by_uuid(&existing.department_uuid)?
                .unwrap()
        };

        let now = chrono::Utc::now().to_rfc3339();

        let rows_updated = self.conn.execute(
            "UPDATE employees SET name = ?, email = ?, position = ?, salary = ?, department_id = ?, updated_at = ? WHERE uuid = ?",
            (new_name, new_email, new_position, new_salary, new_department_id, &now, uuid),
        ).map_err(|e| AppError::Internal(e.to_string()))?;

        if rows_updated == 0 {
            return Err(AppError::NotFound(format!(
                "Employee with uuid {} not found",
                uuid
            )));
        }

        self.find_by_uuid(uuid)?
            .ok_or_else(|| AppError::NotFound(format!("Employee with uuid {} not found", uuid)))
    }

    pub fn delete(&self, uuid: &str) -> Result<(), AppError> {
        let rows_deleted = self
            .conn
            .execute("DELETE FROM employees WHERE uuid = ?", [uuid])
            .map_err(|e| AppError::Internal(e.to_string()))?;

        if rows_deleted == 0 {
            return Err(AppError::NotFound(format!(
                "Employee with uuid {} not found",
                uuid
            )));
        }

        Ok(())
    }
}
