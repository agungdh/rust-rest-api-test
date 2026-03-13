use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};

use crate::domain::entities::Department;
use crate::infrastructure::AppError;

pub type DbPool = Pool<ConnectionManager<SqliteConnection>>;

pub struct DepartmentRepository {
    pool: DbPool,
}

impl DepartmentRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    pub fn find_all(&self) -> Result<Vec<Department>, AppError> {
        let conn = &mut self
            .pool
            .get()
            .map_err(|e| AppError::Internal(e.to_string()))?;

        use crate::domain::entities::schema::departments::dsl::*;

        let results = departments
            .order(created_at.desc())
            .load::<Department>(conn)
            .map_err(|e| AppError::Internal(e.to_string()))?;

        Ok(results)
    }

    pub fn find_by_uuid(&self, dept_uuid: &str) -> Result<Option<Department>, AppError> {
        let conn = &mut self
            .pool
            .get()
            .map_err(|e| AppError::Internal(e.to_string()))?;

        use crate::domain::entities::schema::departments::dsl::*;

        let result = departments
            .filter(uuid.eq(dept_uuid))
            .first::<Department>(conn)
            .optional()
            .map_err(|e| AppError::Internal(e.to_string()))?;

        Ok(result)
    }

    pub fn save(&self, department: &Department) -> Result<Department, AppError> {
        let conn = &mut self
            .pool
            .get()
            .map_err(|e| AppError::Internal(e.to_string()))?;

        let new_department = NewDepartment {
            uuid: &department.uuid,
            name: &department.name,
            created_at: &department.created_at,
            updated_at: department.updated_at.as_deref(),
        };

        diesel::insert_into(crate::domain::entities::schema::departments::table)
            .values(&new_department)
            .execute(conn)
            .map_err(|e| AppError::Internal(e.to_string()))?;

        Ok(department.clone())
    }

    pub fn update(&self, dept_uuid: &str, dept_name: &str) -> Result<Department, AppError> {
        let conn = &mut self
            .pool
            .get()
            .map_err(|e| AppError::Internal(e.to_string()))?;

        use crate::domain::entities::schema::departments::dsl::*;

        let now = chrono::Utc::now().to_rfc3339();

        let rows_updated = diesel::update(departments.filter(uuid.eq(dept_uuid)))
            .set((name.eq(dept_name), updated_at.eq(Some(now))))
            .execute(conn)
            .map_err(|e| AppError::Internal(e.to_string()))?;

        if rows_updated == 0 {
            return Err(AppError::NotFound(format!(
                "Department with uuid {} not found",
                dept_uuid
            )));
        }

        self.find_by_uuid(dept_uuid)?.ok_or_else(|| {
            AppError::NotFound(format!("Department with uuid {} not found", dept_uuid))
        })
    }

    pub fn delete(&self, dept_uuid: &str) -> Result<(), AppError> {
        let conn = &mut self
            .pool
            .get()
            .map_err(|e| AppError::Internal(e.to_string()))?;

        use crate::domain::entities::schema::departments::dsl::*;

        let rows_deleted = diesel::delete(departments.filter(uuid.eq(dept_uuid)))
            .execute(conn)
            .map_err(|e| AppError::Internal(e.to_string()))?;

        if rows_deleted == 0 {
            return Err(AppError::NotFound(format!(
                "Department with uuid {} not found",
                dept_uuid
            )));
        }

        Ok(())
    }
}

#[derive(Insertable)]
#[diesel(table_name = crate::domain::entities::schema::departments)]
struct NewDepartment<'a> {
    uuid: &'a str,
    name: &'a str,
    created_at: &'a str,
    updated_at: Option<&'a str>,
}
