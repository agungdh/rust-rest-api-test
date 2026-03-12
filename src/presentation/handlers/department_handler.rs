use std::sync::Arc;

use axum::{
    extract::{Path, State},
    Json,
};
use validator::Validate;

use crate::application::dto::{CreateDepartmentDto, DepartmentResponse, UpdateDepartmentDto};
use crate::infrastructure::AppError;
use crate::presentation::response::ApiResponse;
use crate::presentation::routes::routes::AppState;

pub struct DepartmentHandler;

impl DepartmentHandler {
    pub async fn get_all(
        State(state): State<Arc<AppState>>,
    ) -> Result<Json<Vec<DepartmentResponse>>, AppError> {
        let service = state.department_service.lock().unwrap();
        let departments = service.get_all()?;
        Ok(Json(departments))
    }

    pub async fn get_by_uuid(
        State(state): State<Arc<AppState>>,
        Path(uuid): Path<String>,
    ) -> Result<Json<ApiResponse<DepartmentResponse>>, AppError> {
        let service = state.department_service.lock().unwrap();
        let department = service.get_by_uuid(&uuid)?;
        Ok(Json(ApiResponse::ok(department)))
    }

    pub async fn create(
        State(state): State<Arc<AppState>>,
        Json(dto): Json<CreateDepartmentDto>,
    ) -> Result<Json<ApiResponse<DepartmentResponse>>, AppError> {
        dto.validate()?;
        let service = state.department_service.lock().unwrap();
        let department = service.create(dto)?;
        Ok(Json(ApiResponse::with_message(department, "Department created successfully")))
    }

    pub async fn update(
        State(state): State<Arc<AppState>>,
        Path(uuid): Path<String>,
        Json(dto): Json<UpdateDepartmentDto>,
    ) -> Result<Json<ApiResponse<DepartmentResponse>>, AppError> {
        dto.validate()?;
        let service = state.department_service.lock().unwrap();
        let department = service.update(&uuid, dto)?;
        Ok(Json(ApiResponse::with_message(department, "Department updated successfully")))
    }

    pub async fn delete(
        State(state): State<Arc<AppState>>,
        Path(uuid): Path<String>,
    ) -> Result<Json<ApiResponse<()>>, AppError> {
        let service = state.department_service.lock().unwrap();
        service.delete(&uuid)?;
        Ok(Json(ApiResponse::with_message((), "Department deleted successfully")))
    }
}
