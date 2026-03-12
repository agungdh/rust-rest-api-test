use std::sync::{Arc, Mutex};

use axum::{routing, Router};

use crate::application::services::{DepartmentService, EmployeeService};
use crate::presentation::handlers::{DepartmentHandler, EmployeeHandler};

pub struct AppState {
    pub employee_service: Arc<Mutex<EmployeeService>>,
    pub department_service: Arc<Mutex<DepartmentService>>,
}

pub fn create_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/employees", routing::get(EmployeeHandler::get_all))
        .route("/api/employees", routing::post(EmployeeHandler::create))
        .route(
            "/api/employees/{uuid}",
            routing::get(EmployeeHandler::get_by_uuid),
        )
        .route(
            "/api/employees/{uuid}",
            routing::put(EmployeeHandler::update),
        )
        .route(
            "/api/employees/{uuid}",
            routing::delete(EmployeeHandler::delete),
        )
        .route("/api/departments", routing::get(DepartmentHandler::get_all))
        .route("/api/departments", routing::post(DepartmentHandler::create))
        .route(
            "/api/departments/{uuid}",
            routing::get(DepartmentHandler::get_by_uuid),
        )
        .route(
            "/api/departments/{uuid}",
            routing::put(DepartmentHandler::update),
        )
        .route(
            "/api/departments/{uuid}",
            routing::delete(DepartmentHandler::delete),
        )
        .with_state(state)
}
