use std::sync::Arc;
use uuid::Uuid;

use crate::domain::Employee;
use crate::infrastructure::errors::AppError;

pub trait EmployeeRepository: Send + Sync {
    fn find_all(&self) -> Vec<Employee>;
    fn find_by_id(&self, id: Uuid) -> Option<Employee>;
    fn save(&self, employee: Employee) -> Employee;
    fn update(&self, employee: Employee) -> Result<Employee, AppError>;
    fn delete(&self, id: Uuid) -> Result<(), AppError>;
}

#[derive(Clone)]
pub struct InMemoryEmployeeRepository {
    employees: Arc<std::sync::RwLock<Vec<Employee>>>,
}

impl InMemoryEmployeeRepository {
    pub fn new() -> Self {
        Self {
            employees: Arc::new(std::sync::RwLock::new(Vec::new())),
        }
    }
}

impl Default for InMemoryEmployeeRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl EmployeeRepository for InMemoryEmployeeRepository {
    fn find_all(&self) -> Vec<Employee> {
        let employees = self.employees.read().unwrap();
        employees.clone()
    }

    fn find_by_id(&self, id: Uuid) -> Option<Employee> {
        let employees = self.employees.read().unwrap();
        employees.iter().find(|e| e.id == id).cloned()
    }

    fn save(&self, employee: Employee) -> Employee {
        let mut employees = self.employees.write().unwrap();
        employees.push(employee.clone());
        employee
    }

    fn update(&self, employee: Employee) -> Result<Employee, AppError> {
        let mut employees = self.employees.write().unwrap();
        if let Some(idx) = employees.iter().position(|e| e.id == employee.id) {
            employees[idx] = employee.clone();
            Ok(employee)
        } else {
            Err(AppError::NotFound(format!(
                "Employee with id {} not found",
                employee.id
            )))
        }
    }

    fn delete(&self, id: Uuid) -> Result<(), AppError> {
        let mut employees = self.employees.write().unwrap();
        if let Some(idx) = employees.iter().position(|e| e.id == id) {
            employees.remove(idx);
            Ok(())
        } else {
            Err(AppError::NotFound(format!(
                "Employee with id {} not found",
                id
            )))
        }
    }
}
