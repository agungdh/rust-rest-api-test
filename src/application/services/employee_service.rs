use rusqlite::Connection;

use crate::application::dto::{CreateEmployeeDto, EmployeeResponse, UpdateEmployeeDto};
use crate::domain::entities::Employee;
use crate::infrastructure::{AppError, EmployeeRepository};

pub struct EmployeeService {
    repository: EmployeeRepository,
}

impl EmployeeService {
    pub fn new(conn: Connection) -> Self {
        Self {
            repository: EmployeeRepository::new(conn),
        }
    }

    pub fn get_all(
        &self,
        department_uuid: Option<&str>,
    ) -> Result<Vec<EmployeeResponse>, AppError> {
        let employees = self.repository.find_all(department_uuid)?;
        Ok(employees
            .into_iter()
            .map(|e| EmployeeResponse {
                uuid: e.uuid,
                name: e.name,
                email: e.email,
                position: e.position,
                salary: e.salary,
                department_uuid: e.department_uuid,
            })
            .collect())
    }

    pub fn get_by_uuid(&self, uuid: &str) -> Result<EmployeeResponse, AppError> {
        let employee = self
            .repository
            .find_by_uuid(uuid)?
            .ok_or_else(|| AppError::NotFound(format!("Employee with uuid {} not found", uuid)))?;

        Ok(EmployeeResponse {
            uuid: employee.uuid,
            name: employee.name,
            email: employee.email,
            position: employee.position,
            salary: employee.salary,
            department_uuid: employee.department_uuid,
        })
    }

    pub fn create(&self, dto: CreateEmployeeDto) -> Result<EmployeeResponse, AppError> {
        let employee = Employee::new(
            dto.name,
            dto.email,
            dto.position,
            dto.salary,
            dto.department_uuid,
        );
        let saved = self.repository.save(&employee)?;

        Ok(EmployeeResponse {
            uuid: saved.uuid,
            name: saved.name,
            email: saved.email,
            position: saved.position,
            salary: saved.salary,
            department_uuid: saved.department_uuid,
        })
    }

    pub fn update(&self, uuid: &str, dto: UpdateEmployeeDto) -> Result<EmployeeResponse, AppError> {
        let updated = self.repository.update(
            uuid,
            dto.name.as_deref(),
            dto.email.as_deref(),
            dto.position.as_deref(),
            dto.salary,
            dto.department_uuid.as_deref(),
        )?;

        Ok(EmployeeResponse {
            uuid: updated.uuid,
            name: updated.name,
            email: updated.email,
            position: updated.position,
            salary: updated.salary,
            department_uuid: updated.department_uuid,
        })
    }

    pub fn delete(&self, uuid: &str) -> Result<(), AppError> {
        self.repository.delete(uuid)
    }
}
