use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};

use crate::domain::entities::Employee;
use crate::infrastructure::AppError;

pub type DbPool = Pool<ConnectionManager<SqliteConnection>>;

pub struct EmployeeRepository {
    pool: DbPool,
}

impl EmployeeRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    pub fn find_all(&self, dept_uuid: Option<&str>) -> Result<Vec<Employee>, AppError> {
        let conn = &mut self
            .pool
            .get()
            .map_err(|e| AppError::Internal(e.to_string()))?;

        use crate::domain::entities::schema::employees::dsl::*;

        let results = if let Some(filter_dept_uuid) = dept_uuid {
            employees
                .filter(department_uuid.eq(filter_dept_uuid))
                .order(created_at.desc())
                .load(conn)
                .map_err(|e| AppError::Internal(e.to_string()))?
        } else {
            employees
                .order(created_at.desc())
                .load(conn)
                .map_err(|e| AppError::Internal(e.to_string()))?
        };

        Ok(results)
    }

    pub fn find_by_uuid(&self, emp_uuid: &str) -> Result<Option<Employee>, AppError> {
        let conn = &mut self
            .pool
            .get()
            .map_err(|e| AppError::Internal(e.to_string()))?;

        use crate::domain::entities::schema::employees::dsl::*;

        let result = employees
            .filter(uuid.eq(emp_uuid))
            .first(conn)
            .optional()
            .map_err(|e| AppError::Internal(e.to_string()))?;

        Ok(result)
    }

    pub fn save(&self, employee: &Employee) -> Result<Employee, AppError> {
        let conn = &mut self
            .pool
            .get()
            .map_err(|e| AppError::Internal(e.to_string()))?;

        use crate::domain::entities::schema::employees::dsl::*;

        let new_employee = NewEmployee {
            uuid: &employee.uuid,
            name: &employee.name,
            email: &employee.email,
            position: &employee.position,
            salary: employee.salary,
            department_id: employee.department_id,
            department_uuid: &employee.department_uuid,
            created_at: &employee.created_at,
            updated_at: employee.updated_at.as_deref(),
        };

        diesel::insert_into(employees)
            .values(&new_employee)
            .execute(conn)
            .map_err(|e| AppError::Internal(e.to_string()))?;

        Ok(employee.clone())
    }

    #[allow(clippy::too_many_arguments)]
    pub fn update(
        &self,
        emp_uuid: &str,
        emp_name: Option<&str>,
        emp_email: Option<&str>,
        emp_position: Option<&str>,
        emp_salary: Option<i64>,
        emp_dept_id: Option<i32>,
        emp_dept_uuid: Option<&str>,
    ) -> Result<Employee, AppError> {
        let conn = &mut self
            .pool
            .get()
            .map_err(|e| AppError::Internal(e.to_string()))?;

        use crate::domain::entities::schema::employees::dsl::*;

        let existing = self.find_by_uuid(emp_uuid)?.ok_or_else(|| {
            AppError::NotFound(format!("Employee with uuid {} not found", emp_uuid))
        })?;

        let new_name = emp_name.unwrap_or(&existing.name);
        let new_email = emp_email.unwrap_or(&existing.email);
        let new_position = emp_position.unwrap_or(&existing.position);
        let new_salary = emp_salary.unwrap_or(existing.salary as i64);
        let new_dept_id = emp_dept_id.unwrap_or(existing.department_id);
        let new_dept_uuid = emp_dept_uuid.unwrap_or(&existing.department_uuid);

        let now = chrono::Utc::now().to_rfc3339();

        let rows_updated = diesel::update(employees.filter(uuid.eq(emp_uuid)))
            .set((
                name.eq(new_name),
                email.eq(new_email),
                position.eq(new_position),
                salary.eq(new_salary as i32),
                department_id.eq(new_dept_id),
                department_uuid.eq(new_dept_uuid),
                updated_at.eq(Some(now)),
            ))
            .execute(conn)
            .map_err(|e| AppError::Internal(e.to_string()))?;

        if rows_updated == 0 {
            return Err(AppError::NotFound(format!(
                "Employee with uuid {} not found",
                emp_uuid
            )));
        }

        self.find_by_uuid(emp_uuid)?
            .ok_or_else(|| AppError::NotFound(format!("Employee with uuid {} not found", emp_uuid)))
    }

    pub fn delete(&self, emp_uuid: &str) -> Result<(), AppError> {
        let conn = &mut self
            .pool
            .get()
            .map_err(|e| AppError::Internal(e.to_string()))?;

        use crate::domain::entities::schema::employees::dsl::*;

        let rows_deleted = diesel::delete(employees.filter(uuid.eq(emp_uuid)))
            .execute(conn)
            .map_err(|e| AppError::Internal(e.to_string()))?;

        if rows_deleted == 0 {
            return Err(AppError::NotFound(format!(
                "Employee with uuid {} not found",
                emp_uuid
            )));
        }

        Ok(())
    }
}

#[derive(Insertable)]
#[diesel(table_name = crate::domain::entities::schema::employees)]
struct NewEmployee<'a> {
    uuid: &'a str,
    name: &'a str,
    email: &'a str,
    position: &'a str,
    salary: i32,
    department_id: i32,
    department_uuid: &'a str,
    created_at: &'a str,
    updated_at: Option<&'a str>,
}
