use rusqlite::Connection;

use crate::application::dto::{CreateDepartmentDto, DepartmentResponse, UpdateDepartmentDto};
use crate::domain::entities::Department;
use crate::infrastructure::{AppError, DepartmentRepository};

pub struct DepartmentService {
    repository: DepartmentRepository,
}

impl DepartmentService {
    pub fn new(conn: Connection) -> Self {
        Self {
            repository: DepartmentRepository::new(conn),
        }
    }

    pub fn get_all(&self) -> Result<Vec<DepartmentResponse>, AppError> {
        let departments = self.repository.find_all()?;
        Ok(departments
            .into_iter()
            .map(|d| DepartmentResponse {
                uuid: d.uuid,
                name: d.name,
            })
            .collect())
    }

    pub fn get_by_uuid(&self, uuid: &str) -> Result<DepartmentResponse, AppError> {
        let department = self.repository.find_by_uuid(uuid)?.ok_or_else(|| {
            AppError::NotFound(format!("Department with uuid {} not found", uuid))
        })?;

        Ok(DepartmentResponse {
            uuid: department.uuid,
            name: department.name,
        })
    }

    pub fn create(&self, dto: CreateDepartmentDto) -> Result<DepartmentResponse, AppError> {
        let department = Department::new(dto.name);
        let saved = self.repository.save(&department)?;

        Ok(DepartmentResponse {
            uuid: saved.uuid,
            name: saved.name,
        })
    }

    pub fn update(
        &self,
        uuid: &str,
        dto: UpdateDepartmentDto,
    ) -> Result<DepartmentResponse, AppError> {
        let name = dto
            .name
            .ok_or_else(|| AppError::Validation(validator::ValidationErrors::default()))?;

        let updated = self.repository.update(uuid, &name)?;

        Ok(DepartmentResponse {
            uuid: updated.uuid,
            name: updated.name,
        })
    }

    pub fn delete(&self, uuid: &str) -> Result<(), AppError> {
        self.repository.delete(uuid)
    }
}
