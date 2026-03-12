use std::sync::Arc;

use axum::{routing, Router};

use crate::presentation::handlers::EmployeeHandler;

pub fn employee_routes() -> Router<Arc<dyn crate::application::services::EmployeeServiceTrait>> {
    Router::new()
        .route("/api/employees", routing::get(EmployeeHandler::get_all))
        .route("/api/employees", routing::post(EmployeeHandler::create))
        .route(
            "/api/employees/{id}",
            routing::get(EmployeeHandler::get_by_id),
        )
        .route("/api/employees/{id}", routing::put(EmployeeHandler::update))
        .route(
            "/api/employees/{id}",
            routing::delete(EmployeeHandler::delete),
        )
}
