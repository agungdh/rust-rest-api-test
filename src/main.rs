mod application;
mod config;
mod domain;
mod infrastructure;
mod presentation;

use std::sync::{Arc, Mutex};

use tracing_subscriber::EnvFilter;

use application::services::{DepartmentService, EmployeeService};
use config::AppConfig;
use infrastructure::establish_connection;
use presentation::routes::{create_router, AppState};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    let config = AppConfig::load().unwrap_or_default();

    let conn1 = establish_connection();
    let conn2 = establish_connection();

    let employee_service = Arc::new(Mutex::new(EmployeeService::new(conn1)));
    let department_service = Arc::new(Mutex::new(DepartmentService::new(conn2)));

    let state = Arc::new(AppState {
        employee_service,
        department_service,
    });

    let app = create_router(state);

    let addr = format!("{}:{}", config.server.host, config.server.port);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    tracing::info!("Server running on http://{}", addr);
    axum::serve(listener, app).await.unwrap();
}
