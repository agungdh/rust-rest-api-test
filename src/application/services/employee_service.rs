use std::sync::Arc;
use uuid::Uuid;

use crate::application::dto::{CreateEmployeeDto, UpdateEmployeeDto};
use crate::domain::Employee;
use crate::infrastructure::{AppError, EmployeeRepository};

pub trait EmployeeServiceTrait: Send + Sync {
    fn get_all(&self) -> Vec<Employee>;
    fn get_by_id(&self, id: Uuid) -> Result<Employee, AppError>;
    fn create(&self, dto: CreateEmployeeDto) -> Employee;
    fn update(&self, id: Uuid, dto: UpdateEmployeeDto) -> Result<Employee, AppError>;
    fn delete(&self, id: Uuid) -> Result<(), AppError>;
}

pub struct EmployeeService {
    repository: Arc<dyn EmployeeRepository>,
}

impl EmployeeService {
    pub fn new(repository: Arc<dyn EmployeeRepository>) -> Self {
        Self { repository }
    }
}

impl EmployeeServiceTrait for EmployeeService {
    fn get_all(&self) -> Vec<Employee> {
        self.repository.find_all()
    }

    fn get_by_id(&self, id: Uuid) -> Result<Employee, AppError> {
        self.repository
            .find_by_id(id)
            .ok_or_else(|| AppError::NotFound(format!("Employee with id {} not found", id)))
    }

    fn create(&self, dto: CreateEmployeeDto) -> Employee {
        let employee = Employee::new(dto.name, dto.email, dto.position, dto.salary);
        self.repository.save(employee)
    }

    fn update(&self, id: Uuid, dto: UpdateEmployeeDto) -> Result<Employee, AppError> {
        let mut employee = self.get_by_id(id)?;

        if let Some(name) = dto.name {
            employee.name = name;
        }
        if let Some(email) = dto.email {
            employee.email = email;
        }
        if let Some(position) = dto.position {
            employee.position = position;
        }
        if let Some(salary) = dto.salary {
            employee.salary = salary;
        }

        self.repository.update(employee)
    }

    fn delete(&self, id: Uuid) -> Result<(), AppError> {
        self.repository.delete(id)
    }
}
