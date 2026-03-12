use std::sync::Arc;

use axum::{
    extract::{Path, State},
    Json,
};
use uuid::Uuid;
use validator::Validate;

use crate::application::dto::{CreateEmployeeDto, UpdateEmployeeDto};
use crate::application::services::EmployeeServiceTrait;
use crate::domain::Employee;
use crate::infrastructure::AppError;
use crate::presentation::response::ApiResponse;

#[derive(Clone)]
pub struct EmployeeHandler;

impl EmployeeHandler {
    pub async fn get_all(
        State(service): State<Arc<dyn EmployeeServiceTrait>>,
    ) -> Result<Json<Vec<Employee>>, AppError> {
        let employees = service.get_all();
        Ok(Json(employees))
    }

    pub async fn get_by_id(
        State(service): State<Arc<dyn EmployeeServiceTrait>>,
        Path(id): Path<Uuid>,
    ) -> Result<Json<ApiResponse<Employee>>, AppError> {
        let employee = service.get_by_id(id)?;
        Ok(Json(ApiResponse::ok(employee)))
    }

    pub async fn create(
        State(service): State<Arc<dyn EmployeeServiceTrait>>,
        Json(dto): Json<CreateEmployeeDto>,
    ) -> Result<Json<ApiResponse<Employee>>, AppError> {
        dto.validate()?;
        let employee = service.create(dto);
        Ok(Json(ApiResponse::with_message(employee, "Employee created successfully")))
    }

    pub async fn update(
        State(service): State<Arc<dyn EmployeeServiceTrait>>,
        Path(id): Path<Uuid>,
        Json(dto): Json<UpdateEmployeeDto>,
    ) -> Result<Json<ApiResponse<Employee>>, AppError> {
        dto.validate()?;
        let employee = service.update(id, dto)?;
        Ok(Json(ApiResponse::with_message(employee, "Employee updated successfully")))
    }

    pub async fn delete(
        State(service): State<Arc<dyn EmployeeServiceTrait>>,
        Path(id): Path<Uuid>,
    ) -> Result<Json<ApiResponse<()>>, AppError> {
        service.delete(id)?;
        Ok(Json(ApiResponse::with_message((), "Employee deleted successfully")))
    }
}
