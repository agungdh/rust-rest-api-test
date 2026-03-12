use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    Json,
};
use validator::Validate;

use crate::application::dto::{CreateEmployeeDto, EmployeeResponse, UpdateEmployeeDto};
use crate::infrastructure::AppError;
use crate::presentation::response::ApiResponse;
use crate::presentation::routes::routes::AppState;

#[derive(serde::Deserialize)]
pub struct ListEmployeesQuery {
    pub department_uuid: Option<String>,
}

pub struct EmployeeHandler;

impl EmployeeHandler {
    pub async fn get_all(
        State(state): State<Arc<AppState>>,
        Query(query): Query<ListEmployeesQuery>,
    ) -> Result<Json<Vec<EmployeeResponse>>, AppError> {
        let service = state.employee_service.lock().unwrap();
        let employees = service.get_all(query.department_uuid.as_deref())?;
        Ok(Json(employees))
    }

    pub async fn get_by_uuid(
        State(state): State<Arc<AppState>>,
        Path(uuid): Path<String>,
    ) -> Result<Json<ApiResponse<EmployeeResponse>>, AppError> {
        let service = state.employee_service.lock().unwrap();
        let employee = service.get_by_uuid(&uuid)?;
        Ok(Json(ApiResponse::ok(employee)))
    }

    pub async fn create(
        State(state): State<Arc<AppState>>,
        Json(dto): Json<CreateEmployeeDto>,
    ) -> Result<Json<ApiResponse<EmployeeResponse>>, AppError> {
        dto.validate()?;
        let service = state.employee_service.lock().unwrap();
        let employee = service.create(dto)?;
        Ok(Json(ApiResponse::with_message(employee, "Employee created successfully")))
    }

    pub async fn update(
        State(state): State<Arc<AppState>>,
        Path(uuid): Path<String>,
        Json(dto): Json<UpdateEmployeeDto>,
    ) -> Result<Json<ApiResponse<EmployeeResponse>>, AppError> {
        dto.validate()?;
        let service = state.employee_service.lock().unwrap();
        let employee = service.update(&uuid, dto)?;
        Ok(Json(ApiResponse::with_message(employee, "Employee updated successfully")))
    }

    pub async fn delete(
        State(state): State<Arc<AppState>>,
        Path(uuid): Path<String>,
    ) -> Result<Json<ApiResponse<()>>, AppError> {
        let service = state.employee_service.lock().unwrap();
        service.delete(&uuid)?;
        Ok(Json(ApiResponse::with_message((), "Employee deleted successfully")))
    }
}
